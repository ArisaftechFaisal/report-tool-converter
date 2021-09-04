use calamine::{open_workbook, Xlsx, Reader, RangeDeserializerBuilder, RangeDeserializer, ToCellDeserializer, DataType};
use serde::{Serialize, Deserialize, de};
use super::error::{ParseError, Result};
use super::subject::Subject;
use crate::constants;
use std::str::FromStr;
use crate::converter::field::{IsRequired, Field, FieldVariant};

fn parse(path: &str) -> Result<Vec<Field>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let worksheet = workbook.worksheet_range_at(0).ok_or(ParseError::NoWorksheet)??;
    let mut rows = worksheet.rows();
    // let mut iter = RangeDeserializerBuilder::new().has_headers(false).from_range::<_, Vec<String>>(&worksheet)?;

    // while let Some(res) = iter.next() {
    //     let row = res?;
    //     println!("row: {:?}", row);
    // }

    let mut fields = Vec::<Field>::new();
    let mut field_cnt = 100usize;

    for field_index in 1..field_cnt {
        let mut is_required: IsRequired = IsRequired::NA;
        let mut variant: FieldVariant = FieldVariant::Text;
        let mut min: Option<u64> = None;
        let mut max: Option<u64> = None;
        let mut label: String = "".to_owned();
        let mut placeholder: Option<String> = None;
        let mut input_specification: Option<String> = None;
        let mut options_from_key: Option<String> = None;
        let mut options = Vec::<String>::new();
        let mut display_condition_first = Vec::<String>::new();
        let mut display_condition_second = Vec::<String>::new();
        let mut display_condition_third = Vec::<String>::new();

        let mut ignore_options = false;

        while let Some(row) = rows.next() {
            let mut subject: Subject;
            if let DataType::String(s) = row.get(0).unwrap() {
                subject = s.parse::<Subject>()?;
            } else {
                continue;
            }
            let dt = row.get(field_index).unwrap();
            let dt_next = row.get(field_index + 1).unwrap();
            match subject {
                Subject::Required => {
                    // println!("{:?}", row);
                    is_required = Field::required_from_datatype(dt)?;
                    if let IsRequired::NA = is_required { field_cnt = field_index + 1; }
                },
                Subject::Type => {
                    variant = Field::variant_from_datatype(dt)?;
                    ignore_options = match variant {
                        FieldVariant::TextArea | FieldVariant::Text => true,
                        _ => ignore_options
                    };
                },
                Subject::Max => {
                    max = Field::optional_u64_from_datatype(dt)?;
                },
                Subject::Min => {
                    min = Field::optional_u64_from_datatype(dt)?;
                },
                Subject::Label => {
                    label = Field::label_from_datatype(dt)?;
                },
                Subject::Placeholder => {
                    let field_ref = Field::field_number_from_datatype(dt_next);
                    match field_ref {
                        // Ignore options and placeholder if OptionsFromKey is present
                        Ok(field_number) => {
                            options_from_key = Some(format!("field{}", field_number));
                            ignore_options = true;
                        },
                        Err(_) => {
                            placeholder = Field::optional_string_from_datatype(dt)?;
                        }
                    }
                },
                Subject::InputSpecification => {
                    input_specification = Field::optional_string_from_datatype(dt)?;
                },
                Subject::Options => {
                    if ignore_options {break;}
                    let option = Field::optional_string_from_datatype(dt)?;
                    match option {
                        Some(s) => { options.push(s.to_owned()); },
                        None => { break; }
                    }
                },
                _ => { continue; }
            }
        }
        let display_condition_first = Field::vec_to_optional_vec(display_condition_first);
        let display_condition_second = Field::vec_to_optional_vec(display_condition_second);
        let display_condition_third = Field::vec_to_optional_vec(display_condition_third);
        let options = Field::vec_to_optional_vec(options);

        fields.push(Field{
            is_required,
            variant,
            label,
            display_condition_first,
            display_condition_second,
            display_condition_third,
            input_specification,
            min,
            max,
            placeholder,
            options_from_key,
            options
        });
    }

    Ok(fields)


}

#[cfg(test)]
mod parse_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_read_worksheet() {
        let path = Path::new(constants::PATH_TEST_01).to_str().unwrap();
        println!("path: {}", path);
        let res = parse(path);
        println!("{:?}", res.unwrap().get(0).unwrap());
        assert_eq!(1,1);
    }
}