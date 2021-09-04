use super::error::{ParseError, Result};
use calamine::DataType;
use serde::{Deserialize, Serialize};

pub mod subtypes;
mod validators;
use subtypes::*;

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

  #[serde(skip_serializing_if = "Option::is_none")]
  input_specification: Option<InputSpecification>,

  #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
  options: Option<Vec<String>>,

  #[serde(rename = "OptionsFromKey", skip_serializing_if = "Option::is_none")]
  options_from_key: Option<String>,

  #[serde(skip)]
  max: Option<u64>,

  #[serde(skip)]
  min: Option<u64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  display_condition_first: Option<Vec<String>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  display_condition_second: Option<Vec<String>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  display_condition_third: Option<Vec<String>>,

  #[serde(flatten)]
  processed: ProcessedData,
}

impl Field {
  pub fn new(
    is_required: bool,
    field_name: String,
    variant: FieldVariant,
    label: String,
    placeholder_text: Option<String>,
    input_specification: Option<InputSpecification>,
    options: Option<Vec<String>>,
    options_from_key: Option<String>,
    max: Option<u64>,
    min: Option<u64>,
    display_condition_first: Option<Vec<String>>,
    display_condition_second: Option<Vec<String>>,
    display_condition_third: Option<Vec<String>>,
  ) -> Result<Self> {
    let mut validators: Option<Vec<Validator>>;
    let price_max: Option<u64>;
    let mut placeholder: Option<String> = None;
    let mut options_key: Option<String> = None;

    // Placeholder logic
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
    if let None = max {
      price_max = None;
    } else if let FieldVariant::TextArea = variant {
      price_max = Some(max.unwrap());
    } else {
      price_max = None;
    }

    // Validators logic
    match variant {
      FieldVariant::Text => {
        validators = Some(Self::text_validators(
          &field_name,
          &min,
          &max,
          &input_specification,
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

    Ok(Field {
      is_required,
      field_name,
      variant,
      label,
      placeholder_text,
      input_specification,
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
        options_key,
      },
    })
  }

  pub fn field_number_from_datatype(dt: &DataType) -> Result<usize> {
    match dt {
      DataType::String(s) => Ok(
        s.trim_start_matches("field")
          .parse::<usize>()
          .map_err(|_| ParseError::UnparseableFieldNumber)?,
      ),
      DataType::Int(i) => Ok(*i as usize),
      _ => Err(ParseError::ExpectedIntOrString),
    }
  }

  pub fn required_from_datatype(dt: &DataType) -> Result<Option<bool>> {
    match dt {
      DataType::Empty => Ok(None),
      DataType::String(s) if s.is_empty() => Ok(None),
      DataType::String(s) => match s.as_str() {
        "表示(必須)" => Ok(Some(true)),
        "表示(任意)" => Ok(Some(false)),
        unknown_string => Err(ParseError::IncorrectRequired(unknown_string.to_owned())),
      },
      _ => Err(ParseError::ExpectedString),
    }
  }

  pub fn variant_from_datatype(dt: &DataType) -> Result<FieldVariant> {
    match dt {
      DataType::String(s) => Ok(s.parse::<FieldVariant>()?),
      _ => Err(ParseError::ExpectedString),
    }
  }

  pub fn label_from_datatype(dt: &DataType) -> Result<String> {
    match dt {
      DataType::String(s) => Ok(s.to_owned()),
      _ => Err(ParseError::ExpectedString),
    }
  }

  pub fn input_specification_from_datatype(dt: &DataType) -> Result<Option<InputSpecification>> {
    match dt {
      DataType::Empty => Ok(None),
      DataType::String(s) if s.is_empty() => Ok(None),
      DataType::String(s) => Ok(Some(s.parse::<InputSpecification>()?)),
      _ => Err(ParseError::ExpectedString),
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
      _ => Err(ParseError::UnparseableCell),
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
      _ => Err(ParseError::UnparseableCell),
    }
  }

  pub fn vec_to_optional_vec<T>(v: Vec<T>) -> Option<Vec<T>> {
    match v.len() {
      0 => None,
      _ => Some(v),
    }
  }
}
