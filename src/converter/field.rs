use std::str::FromStr;
use super::error::{ParseError, Result};
use calamine::DataType;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Field {
    pub is_required: IsRequired,
    pub variant: FieldVariant,
    pub label: String,
    pub placeholder: Option<String>,
    pub input_specification: Option<String>,
    pub options: Option<Vec<String>>,
    pub options_from_key: Option<String>,
    pub max: Option<u64>,
    pub min: Option<u64>,
    pub display_condition_first: Option<Vec<String>>,
    pub display_condition_second: Option<Vec<String>>,
    pub display_condition_third: Option<Vec<String>>
}

impl Field {
    pub(crate) fn field_number_from_datatype(dt: &DataType) -> Result<usize> {
        match dt {
            DataType::String(s) => {
                Ok(s.trim_start_matches("field").parse::<usize>()?)
            },
            DataType::Int(i) => Ok(*i as usize),
            _ => Err(ParseError::ExpectedIntOrString)
        }
    }

    pub(crate) fn required_from_datatype(dt: &DataType) -> Result<IsRequired> {
       match dt {
           DataType::String(s) => Ok(s.parse::<IsRequired>()?),
           DataType::Empty => Ok(IsRequired::NA),
           _ => Err(ParseError::ExpectedString)
       }
    }

    pub(crate) fn variant_from_datatype(dt: &DataType) -> Result<FieldVariant> {
        match dt {
            DataType::String(s) => Ok(s.parse::<FieldVariant>()?),
            _ => Err(ParseError::ExpectedString)
        }
    }

    pub(crate) fn label_from_datatype(dt: &DataType) -> Result<String> {
        match dt {
            DataType::String(s) => Ok(s.to_owned()),
            _ => Err(ParseError::ExpectedString)
        }
    }

    pub(crate) fn optional_string_from_datatype(dt: &DataType) -> Result<Option<String>> {
        match dt {
            DataType::Empty => Ok(None),
            DataType::String(s) if s.is_empty() => Ok(None),
            DataType::String(s) => Ok(Some(s.to_owned())),
            DataType::Float(f) => Ok(Some(format!("{}", f))),
            DataType::Int(i) => Ok(Some(format!("{}", i))),
            DataType::DateTime(f) => Ok(Some(format!("{}", f))) ,
            _ => Err(ParseError::UnparseableCell)
        }
    }

    pub(crate) fn optional_u64_from_datatype(dt: &DataType) -> Result<Option<u64>> {
        match dt {
            DataType::Empty => Ok(None),
            DataType::String(s) if s.is_empty() => Ok(None),
            DataType::String(s) => Ok(Some(s.parse::<u64>()?)),
            DataType::Float(f) => Ok(Some(*f as u64)),
            DataType::Int(i) => Ok(Some(*i as u64)),
            DataType::DateTime(f) => Ok(Some(*f as u64)) ,
            _ => Err(ParseError::UnparseableCell)
        }
    }

    pub(crate) fn vec_to_optional_vec<T>(v: Vec<T>) -> Option<Vec<T>> {
        match v.len() {
            0 => None,
            _ => Some(v)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum IsRequired {
    True,
    False,
    NA
}

impl Default for IsRequired {
    fn default() -> Self {
        IsRequired::NA
    }
}

impl FromStr for IsRequired {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "表示(必須)" => Ok(IsRequired::True),
            "表示(任意)" => Ok(IsRequired::False),
            empty if empty.is_empty() => Ok(IsRequired::NA),
            unknown_required => Err(ParseError::IncorrectRequired(unknown_required.to_owned()))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum FieldVariant {
    Dropdown,
    Text,
    TextArea,
    Radio,
    Multiselect
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
            unknown_field => Err(ParseError::IncorrectFieldVariant(unknown_field.to_owned()))
        }
    }
}