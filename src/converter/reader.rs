use super::error::{ConvertError, Result};
use super::field::{
  subtypes::{FieldVariant, InputSpec},
  Field, Page,
};
use super::subject::Subject;
use crate::converter::field::subtypes::{NumInputSpec, OptionType};
use calamine::{open_workbook, DataType, Reader, Xlsx};

pub(crate) fn parse(path: &str) -> Result<Page> {
  let mut workbook: Xlsx<_> = open_workbook(path)?;
  let worksheet = workbook
    .worksheet_range_at(0)
    .ok_or(ConvertError::NoWorksheet)??;

  let mut fields = Vec::<Field>::new();
  let max_field_cnt = 100usize;

  'field_loop: for field_index in 1..max_field_cnt {
    let field_name = format!("field{}", field_index);
    let mut is_required: bool = false;
    let mut variant: FieldVariant = FieldVariant::Text;
    let mut min: Option<u64> = None;
    let mut max: Option<u64> = None;
    let mut label: String = "".to_owned();
    let mut placeholder_text: Option<String> = None;
    let mut input_spec: Option<InputSpec> = None;
    let mut num_input_spec: Option<NumInputSpec> = None;
    let mut num_input_spec_error: Option<String> = None;
    let mut options_from_key: Option<String> = None;
    let mut options = Vec::<OptionType>::new();
    // TODO: implement deserialize and serialize logic for display conditions
    let display_condition_first = Vec::<String>::new();
    let display_condition_second = Vec::<String>::new();
    let display_condition_third = Vec::<String>::new();

    let mut ignore_options = false;

    let rows = worksheet.rows();
    // 'row_loop: while let Some(row) = rows.next() {
    'row_loop: for row in rows {
      let subject: Subject;
      if let DataType::String(s) = row.get(0).unwrap() {
        subject = s.parse::<Subject>()?;
      } else {
        continue;
      }
      let col_index = if field_index == 1 {
        1
      } else {
        (field_index - 1) * 2 + 1
      };
      let dt = row.get(col_index).unwrap();
      let dt_next = row.get(col_index + 1).unwrap();
      match subject {
        Subject::Required => {
          let has_required = Field::required_from_datatype(dt)?;
          match has_required {
            Some(b) => {
              is_required = b;
            }
            None => {
              break 'field_loop;
            }
          }
          // if let IsRequired::NA = is_required { field_cnt = field_index + 1; }
        }
        Subject::Type => {
          variant = Field::variant_from_datatype(dt)?;
          ignore_options = match variant {
            FieldVariant::TextArea | FieldVariant::Text => true,
            _ => ignore_options,
          };
        }
        Subject::Max => {
          max = Field::optional_u64_from_datatype(dt)?;
        }
        Subject::Min => {
          min = Field::optional_u64_from_datatype(dt)?;
        }
        Subject::Label => {
          label = Field::label_from_datatype(dt)?;
        }
        Subject::Placeholder => {
          let field_ref = Field::field_number_from_datatype(dt_next);
          match field_ref {
            // Ignore options and placeholder if OptionsFromKey is present
            Ok(field_number) => {
              options_from_key = Some(format!("field{}", field_number));
              ignore_options = true;
            }
            Err(_) => {
              placeholder_text = Field::optional_string_from_datatype(dt)?;
            }
          }
        }
        Subject::InputSpec => {
          input_spec = Field::input_specification_from_datatype(dt)?;
        }
        Subject::NumInputSpec => {
          num_input_spec = Field::num_input_specification_from_datatype(dt)?;
        }
        Subject::NumInputSpecError => {
          num_input_spec_error = Field::optional_string_from_datatype(dt)?;
        }
        Subject::Options => {
          if ignore_options {
            break;
          }
          let option = Field::optional_string_from_datatype(dt)?;
          match option {
            Some(s) => {
              options.push(OptionType::new(s.to_owned()));
            }
            None => {
              break 'row_loop;
            }
          }
        }
        _ => {
          continue;
        }
      }
    }
    let display_condition_first = Field::vec_to_optional_vec(display_condition_first);
    let display_condition_second = Field::vec_to_optional_vec(display_condition_second);
    let display_condition_third = Field::vec_to_optional_vec(display_condition_third);
    let options = Field::vec_to_optional_vec(options);

    fields.push(Field::new(
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
    )?);
  }

  Ok(Page::new(fields))
}
