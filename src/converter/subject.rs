use super::error::ConvertError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Subject {
  #[serde(rename = "ページング")]
  Paging,
  #[serde(rename = "表示")]
  Required,
  #[serde(rename = "表示条件1")]
  DisplayConditionFirst,
  #[serde(rename = "表示条件2")]
  DisplayConditionSecond,
  #[serde(rename = "表示条件3")]
  DisplayConditionThird,
  #[serde(rename = "タイプ")]
  Type,
  #[serde(rename = "最大")]
  Max,
  #[serde(rename = "最小")]
  Min,
  #[serde(rename = "ラベル")]
  Label,
  #[serde(rename = "プレースホルダ")]
  Placeholder,
  #[serde(rename = "入力指定")]
  InputSpec,
  #[serde(rename = "数字入力指定範囲(例：10~90)")]
  NumInputSpec,
  #[serde(rename = "入力指定エラー文言")]
  NumInputSpecError,
  #[serde(rename = "プルダウン")]
  Options,
}

impl FromStr for Subject {
  type Err = ConvertError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "ページング" => Ok(Subject::Paging),
      "表示" => Ok(Subject::Required),
      "表示条件1" => Ok(Subject::DisplayConditionFirst),
      "表示条件2" => Ok(Subject::DisplayConditionSecond),
      "表示条件3" => Ok(Subject::DisplayConditionThird),
      "タイプ" => Ok(Subject::Type),
      "最大" => Ok(Subject::Max),
      "最小" => Ok(Subject::Min),
      "ラベル" => Ok(Subject::Label),
      "プレースホルダ" => Ok(Subject::Placeholder),
      "入力指定" => Ok(Subject::InputSpec),
      "数字入力指定範囲(例：10~90)" => Ok(Subject::NumInputSpec),
      "入力指定エラー文言" => Ok(Subject::NumInputSpecError),
      options if options.starts_with("プルダウン") => Ok(Subject::Options),
      unknown_subject => Err(ConvertError::IncorrectSubject(unknown_subject.to_owned())),
    }
  }
}
