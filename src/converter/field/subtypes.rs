use crate::converter::error::ConvertError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum FieldVariant {
  #[serde(rename = "dropdown")]
  Dropdown,

  #[serde(rename = "text")]
  Text,

  #[serde(rename = "textarea")]
  TextArea,

  #[serde(rename = "radio")]
  Radio,

  #[serde(rename = "checkbox")]
  Multiselect,
}

impl Default for FieldVariant {
  fn default() -> Self {
    FieldVariant::Text
  }
}

impl FromStr for FieldVariant {
  type Err = ConvertError;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      "プルダウン" => Ok(FieldVariant::Dropdown),
      "テキスト一行" => Ok(FieldVariant::Text),
      "テキストエリア" => Ok(FieldVariant::TextArea),
      "マルチセレクト" => Ok(FieldVariant::Multiselect),
      "ラジオボタン" => Ok(FieldVariant::Radio),
      unknown_string => Err(ConvertError::IncorrectFieldVariant(
        unknown_string.to_owned(),
      )),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct OptionType {
  #[serde(rename = "Value")]
  value: String,

  #[serde(rename = "Label")]
  label: String,
}

impl OptionType {
  pub fn new(val: String) -> Self {
    OptionType {
      label: val.clone(),
      value: val,
    }
  }

  pub fn is_val(&self, v: &str) -> bool {
    self.value == v
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum InputSpec {
  HalfWidthNumber,
  HalfWidthKanji,
}

impl FromStr for InputSpec {
  type Err = ConvertError;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      "半角数字" => Ok(InputSpec::HalfWidthNumber),
      "半角英字" => Ok(InputSpec::HalfWidthKanji),
      unknown_string => Err(ConvertError::IncorrectInputSpecificationError(
        unknown_string.to_owned(),
      )),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub(crate) struct NumInputSpec {
  pub max: u32,
  pub min: u32,
}

impl FromStr for NumInputSpec {
  type Err = ConvertError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split('~').collect::<Vec<&str>>().as_slice() {
      [mn, mx] => {
        if let (Ok(min), Ok(max)) = (mn.parse::<u32>(), mx.parse::<u32>()) {
          Ok(Self { min, max })
        } else {
          Err(ConvertError::IncorrectNumInputSpecificationError(
            s.to_owned(),
          ))
        }
      }
      _ => Err(ConvertError::IncorrectNumInputSpecificationError(
        s.to_owned(),
      )),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ProcessedData {
  #[serde(rename = "Placeholder", skip_serializing_if = "Option::is_none")]
  pub placeholder: Option<String>,

  #[serde(rename = "OptionsCaption", skip_serializing_if = "Option::is_none")]
  pub options_caption: Option<String>,

  #[serde(rename = "Validators", skip_serializing_if = "Option::is_none")]
  pub validators: Option<Vec<Validator>>,

  #[serde(rename = "PriceMax", skip_serializing_if = "Option::is_none")]
  pub price_max: Option<u64>,

  #[serde(rename = "Visible", skip_serializing_if = "Option::is_none")]
  pub visible: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Validator {
  #[serde(rename = "Type")]
  pub validator_type: ValidatorType,

  #[serde(rename = "Text")]
  pub text: String,

  #[serde(rename = "Expression", skip_serializing_if = "Option::is_none")]
  pub expression: Option<String>,

  #[serde(rename = "MinLength", skip_serializing_if = "Option::is_none")]
  pub min_length: Option<u64>,

  #[serde(rename = "MaxLength", skip_serializing_if = "Option::is_none")]
  pub max_length: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum ValidatorType {
  #[serde(rename = "text")]
  Text,

  #[serde(rename = "expression")]
  Expression,

  #[serde(rename = "answercount")]
  AnswerCount,
}
