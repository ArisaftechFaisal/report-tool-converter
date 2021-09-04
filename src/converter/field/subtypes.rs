use crate::converter::error::ParseError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum FieldVariant {
  Dropdown,
  Text,
  TextArea,
  Radio,
  Multiselect,
}

impl Default for FieldVariant {
  fn default() -> Self {
    FieldVariant::Text
  }
}

impl FromStr for FieldVariant {
  type Err = ParseError;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      "プルダウン" => Ok(FieldVariant::Dropdown),
      "テキスト一行" => Ok(FieldVariant::Text),
      "テキストエリア" => Ok(FieldVariant::TextArea),
      "マルチセレクト" => Ok(FieldVariant::Multiselect),
      "ラジオボタン" => Ok(FieldVariant::Radio),
      unknown_string => Err(ParseError::IncorrectFieldVariant(unknown_string.to_owned())),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum InputSpecification {
  HalfWidthNumber,
  HalfWidthKanji,
}

impl FromStr for InputSpecification {
  type Err = ParseError;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      "半角数字" => Ok(InputSpecification::HalfWidthNumber),
      "半角英字" => Ok(InputSpecification::HalfWidthKanji),
      unknown_string => Err(ParseError::IncorrectInputSpecificationError(
        unknown_string.to_owned(),
      )),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ProcessedData {
  #[serde(rename = "Placeholder", skip_serializing_if = "Option::is_none")]
  pub placeholder: Option<String>,

  #[serde(rename = "OptionsKey", skip_serializing_if = "Option::is_none")]
  pub options_key: Option<String>,

  #[serde(rename = "Validators", skip_serializing_if = "Option::is_none")]
  pub validators: Option<Vec<Validator>>,

  #[serde(rename = "PriceMax", skip_serializing_if = "Option::is_none")]
  pub price_max: Option<u64>,
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
