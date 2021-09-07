use crate::converter::error::Result;
use crate::converter::field::Page;
use std::fs::File;

pub(crate) fn write_to_file(input: Page, output_path: &str) -> Result<()> {
  serde_json::to_writer_pretty(&File::create(output_path)?, &input)?;
  Ok(())
}
