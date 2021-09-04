use thiserror::Error;
use std::convert::From;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("No worksheet in selected Xlsx file")]
    NoWorksheet,
    #[error("ReadError: {0}")]
    ReadError(String),
    #[error("DeserializeError: {0}")]
    DeserializeError(String),
    #[error("XlsxError: {0}")]
    XlsxError(String),
    #[error("Wrong subject format \"{0}\"")]
    IncorrectSubject(String),
    #[error("Wrong field type \"{0}\"")]
    IncorrectFieldVariant(String),
    #[error("Wrong required format \"{0}\"")]
    IncorrectRequired(String),
    #[error("Wrong input specification format \"{0}\"")]
    IncorrectInputSpecificationError(String),
    #[error("Expected string, found something else")]
    ExpectedString,
    #[error("Expected whole number, found something else")]
    ExpectedInt,
    #[error("Expected whole number or string, found something else")]
    ExpectedIntOrString,
    #[error("Couldn't parse string to get field number")]
    UnparseableFieldNumber,
    #[error("Could not parse cell")]
    UnparseableCell,
    #[error("Placeholder for multiselect not in options")]
    PlaceholderNotInOptions,
    #[error("ParseError: {0}")]
    CustomMsg(String)
}

impl From<calamine::Error> for ParseError {
    fn from(err: calamine::Error) -> Self {
       match err {
         _ => {
            ParseError::ReadError(err.to_string())
         }
       }
    }
}

impl From<calamine::DeError> for ParseError {
    fn from(err: calamine::DeError) -> Self {
        match err {
            _ => {
                ParseError::DeserializeError(err.to_string())
            }
        }
    }
}

impl From<calamine::XlsxError> for ParseError {
    fn from(err: calamine::XlsxError) -> Self {
        match err {
            _ => {
                ParseError::XlsxError(err.to_string())
            }
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        match err {
            _ => {
               ParseError::ExpectedInt
            }
        }

    }
}

pub(crate) type Result<T> = std::result::Result<T, ParseError>;
