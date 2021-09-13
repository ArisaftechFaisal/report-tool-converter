use super::error::{ConvertError, Result};
use calamine::DataType;
use serde::{Deserialize, Serialize};

pub mod subtypes;
mod validators;
use subtypes::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Page {
  #[serde(rename = "Elements")]
  elements: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Field {
  #[serde(rename = "QuestionKey")]
  field_name: String,

  #[serde(rename = "Required")]
  is_required: bool,

  #[serde(rename = "Type")]
  variant: FieldVariant,

  #[serde(rename = "Label")]
  label: String,

  #[serde(skip)]
  placeholder_text: Option<String>,

  // #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(skip)]
  input_spec: Option<InputSpec>,

  // #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(skip)]
  num_input_spec: Option<NumInputSpec>,

  // #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(skip)]
  num_input_spec_error: Option<String>,

  #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
  options: Option<Vec<OptionType>>,

  #[serde(rename = "OptionsFromKey", skip_serializing_if = "Option::is_none")]
  options_from_key: Option<String>,

  #[serde(skip)]
  max: Option<u64>,

  #[serde(skip)]
  min: Option<u64>,

  // #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(skip)]
  display_condition_first: Option<Vec<String>>,

  // #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(skip)]
  display_condition_second: Option<Vec<String>>,

  // #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(skip)]
  display_condition_third: Option<Vec<String>>,

  #[serde(flatten)]
  processed: ProcessedData,
}

impl Page {
  pub fn new(elements: Vec<Field>) -> Self {
    Page { elements }
  }
}

impl Field {
  pub fn new(
    is_required: bool,
    field_name: String,
    variant: FieldVariant,
    label: String,
    placeholder_text: Option<String>,
    input_spec: Option<InputSpec>,
    num_input_spec: Option<NumInputSpec>,
    num_input_spec_error: Option<String>,
    options: Option<Vec<OptionType>>,
    options_from_key: Option<String>,
    max: Option<u64>,
    min: Option<u64>,
    display_condition_first: Option<Vec<String>>,
    display_condition_second: Option<Vec<String>>,
    display_condition_third: Option<Vec<String>>,
  ) -> Result<Self> {
    // Placeholder logic
    let mut placeholder: Option<String> = None;
    let mut options_key: Option<String> = None;
    if let Some(ref plc_text) = placeholder_text {
      match variant {
        FieldVariant::Text | FieldVariant::TextArea => {
          placeholder = Some(plc_text.to_owned());
        }
        FieldVariant::Dropdown => options_key = Some(plc_text.to_owned()),
        _ => (),
      }
    }

    // PriceMax logic
    let mut price_max: Option<u64> = None;
    if let FieldVariant::TextArea = variant {
      price_max = max;
    }

    // Validators logic
    let mut validators: Option<Vec<Validator>>;
    match variant {
      FieldVariant::Text => {
        validators = Some(Self::text_validators(
          &field_name,
          &min,
          &max,
          &input_spec,
          &num_input_spec,
          &num_input_spec_error,
        ));
      }
      FieldVariant::TextArea => {
        validators = Some(Self::textarea_validators(&min));
      }
      FieldVariant::Multiselect => {
        validators = Some(Self::multiselect_validators(
          &field_name,
          &min,
          &max,
          &placeholder_text,
          &options,
        )?);
      }
      _ => {
        validators = None;
      }
    }
    if let Some(ref vlds) = validators {
      if vlds.is_empty() {
        validators = None;
      }
    }

    // Visibility logic
    let mut visible: Option<String> = None;
    if let Some(ref opt_from_key) = options_from_key {
      visible = Some(format!("${{{0}}} && ${{{0}}}.length > 1", opt_from_key));
    }

    Ok(Field {
      is_required,
      field_name,
      variant,
      label,
      placeholder_text,
      input_spec,
      num_input_spec,
      num_input_spec_error,
      options,
      options_from_key,
      max,
      min,
      display_condition_first,
      display_condition_second,
      display_condition_third,
      processed: ProcessedData {
        validators,
        price_max,
        placeholder,
        options_caption: options_key,
        visible,
      },
    })
  }

  pub fn field_number_from_datatype(dt: &DataType) -> Result<usize> {
    match dt {
      DataType::String(s) => Ok(
        s.trim_start_matches("field")
          .parse::<usize>()
          .map_err(|_| ConvertError::UnparseableFieldNumber)?,
      ),
      DataType::Int(i) => Ok(*i as usize),
      _ => Err(ConvertError::ExpectedIntOrString),
    }
  }

  pub fn required_from_datatype(dt: &DataType) -> Result<Option<bool>> {
    match dt {
      DataType::Empty => Ok(None),
      DataType::String(s) if s.is_empty() => Ok(None),
      DataType::String(s) => match s.as_str() {
        "表示(必須)" => Ok(Some(true)),
        "表示(任意)" => Ok(Some(false)),
        unknown_string => Err(ConvertError::IncorrectRequired(unknown_string.to_owned())),
      },
      _ => Err(ConvertError::ExpectedString),
    }
  }

  pub fn variant_from_datatype(dt: &DataType) -> Result<FieldVariant> {
    match dt {
      DataType::String(s) => Ok(s.parse::<FieldVariant>()?),
      _ => Err(ConvertError::ExpectedString),
    }
  }

  pub fn label_from_datatype(dt: &DataType) -> Result<String> {
    match dt {
      DataType::String(s) => Ok(s.to_owned()),
      _ => Err(ConvertError::ExpectedString),
    }
  }

  pub fn input_specification_from_datatype(dt: &DataType) -> Result<Option<InputSpec>> {
    match dt {
      DataType::Empty => Ok(None),
      DataType::String(s) if s.is_empty() => Ok(None),
      DataType::String(s) => Ok(Some(s.parse::<InputSpec>()?)),
      _ => Err(ConvertError::ExpectedString),
    }
  }

  pub fn num_input_specification_from_datatype(dt: &DataType) -> Result<Option<NumInputSpec>> {
    match dt {
      DataType::Empty => Ok(None),
      DataType::String(s) if s.is_empty() => Ok(None),
      DataType::String(s) => Ok(Some(s.parse::<NumInputSpec>()?)),
      _ => Err(ConvertError::ExpectedString),
    }
  }

  pub fn optional_string_from_datatype(dt: &DataType) -> Result<Option<String>> {
    match dt {
      DataType::Empty => Ok(None),
      DataType::String(s) if s.is_empty() => Ok(None),
      DataType::String(s) => Ok(Some(s.to_owned())),
      DataType::Float(f) => Ok(Some(format!("{}", f))),
      DataType::Int(i) => Ok(Some(format!("{}", i))),
      DataType::DateTime(f) => Ok(Some(format!("{}", f))),
      _ => Err(ConvertError::UnparseableCell),
    }
  }

  pub fn optional_u64_from_datatype(dt: &DataType) -> Result<Option<u64>> {
    match dt {
      DataType::Empty => Ok(None),
      DataType::String(s) if s.is_empty() => Ok(None),
      DataType::String(s) => Ok(Some(s.parse::<u64>()?)),
      DataType::Float(f) => Ok(Some(*f as u64)),
      DataType::Int(i) => Ok(Some(*i as u64)),
      DataType::DateTime(f) => Ok(Some(*f as u64)),
      _ => Err(ConvertError::UnparseableCell),
    }
  }

  pub fn vec_to_optional_vec<T>(v: Vec<T>) -> Option<Vec<T>> {
    match v.len() {
      0 => None,
      _ => Some(v),
    }
  }
}
