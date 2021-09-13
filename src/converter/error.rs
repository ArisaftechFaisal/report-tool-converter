use serde_json::Error;
use std::convert::From;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
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
  #[error("Wrong number input specification format \"{0}\"")]
  IncorrectNumInputSpecificationError(String),
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
  #[error("Error while trying to serialize: {0}")]
  SerializeError(String),
  #[error("IO Error: {0}")]
  IOError(String),
}

impl From<calamine::Error> for ConvertError {
  fn from(err: calamine::Error) -> Self {
    ConvertError::ReadError(err.to_string())
  }
}

impl From<calamine::DeError> for ConvertError {
  fn from(err: calamine::DeError) -> Self {
    ConvertError::DeserializeError(err.to_string())
  }
}

impl From<calamine::XlsxError> for ConvertError {
  fn from(err: calamine::XlsxError) -> Self {
    ConvertError::XlsxError(err.to_string())
  }
}

impl From<ParseIntError> for ConvertError {
  fn from(_: ParseIntError) -> Self {
    ConvertError::ExpectedInt
  }
}

impl From<serde_json::Error> for ConvertError {
  fn from(err: Error) -> Self {
    ConvertError::SerializeError(err.to_string())
  }
}

impl From<std::io::Error> for ConvertError {
  fn from(err: std::io::Error) -> Self {
    ConvertError::IOError(err.to_string())
  }
}

pub(crate) type Result<T> = std::result::Result<T, ConvertError>;
