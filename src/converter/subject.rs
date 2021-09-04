use std::str::FromStr;
use super::error::ParseError;
use serde::{Serialize, Deserialize};

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
    InputSpecification,
    #[serde(rename = "プルダウン")]
    Options
}


impl FromStr for Subject {
    type Err = ParseError;

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
            "入力指定" => Ok(Subject::InputSpecification),
            options if options.starts_with("プルダウン") => Ok(Subject::Options),
            unknown_subject => Err(ParseError::IncorrectSubject(unknown_subject.to_owned()))
        }
    }
}
