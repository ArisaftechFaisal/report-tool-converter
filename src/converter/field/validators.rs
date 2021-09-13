use super::subtypes::{InputSpec, Validator, ValidatorType};
use super::Field;
use crate::converter::error::{ConvertError, Result};
use crate::converter::field::subtypes::{OptionType, NumInputSpec};

impl Field {
  pub(super) fn text_validators(
    field_name: &str,
    min: &Option<u64>,
    max: &Option<u64>,
    input_spec: &Option<InputSpec>,
    num_input_spec: &Option<NumInputSpec>,
    num_input_spec_error: &Option<String>
  ) -> Vec<Validator> {
    let mut validators = Vec::<Validator>::new();
    if let (Some(mn), Some(mx)) = (min, max) {
      if mn == mx {
        validators.push(Validator {
          validator_type: ValidatorType::Text,
          text: format!("{}文字で入力してください", mx),
          min_length: *min,
          max_length: *max,
          expression: None,
        });
      }
      if let (Some(val), Some(error_text)) = (num_input_spec, num_input_spec_error) {
        validators.push(Validator{
          validator_type: ValidatorType::Expression,
          text: error_text.to_owned(),
          min_length: None,
          max_length: None,
          expression: Some(format!(
            "${{{0}}} && ${{{0}}} >= {1} && ${{{0}}} < {2}", field_name, val.min, val.max
          )),
        });
      }
    }

    if let Some(inp_spec) = input_spec {
      match inp_spec {
        InputSpec::HalfWidthNumber => {
          if let (Some(mn), Some(mx)) = (*min, *max) {
            if mx == 2 && mn == 1 {
              validators.push(Validator {
                validator_type: ValidatorType::Expression,
                text: "入力できるのは半角数字のみです".to_owned(),
                min_length: None,
                max_length: None,
                expression: Some(format!(
                  "${{{0}}} && ${{{0}}}.match(/^[0-9]+$/)",
                  field_name
                )),
              });
            }
          }
        }
        InputSpec::HalfWidthKanji => {
          validators.push(Validator {
            validator_type: ValidatorType::Expression,
            text: "入力できるのは半角英字のみです".to_owned(),
            min_length: None,
            max_length: None,
            expression: Some(format!(
              "${{{0}}} && ${{{0}}}.match(/^([a-zA-Z])+$/)",
              field_name
            )),
          });
        }
      }
    }

    validators
  }

  pub(super) fn textarea_validators(min: &Option<u64>) -> Vec<Validator> {
    let mut validators = Vec::<Validator>::new();

    if let Some(mn) = min {
      validators.push(Validator {
        validator_type: ValidatorType::Text,
        text: format!("{}文字以上で入力してください", mn),
        min_length: *min,
        max_length: None,
        expression: None,
      });
    }
    validators
  }

  pub(super) fn multiselect_validators(
    field_name: &str,
    min: &Option<u64>,
    max: &Option<u64>,
    placeholder_text: &Option<String>,
    options: &Option<Vec<OptionType>>,
  ) -> Result<Vec<Validator>> {
    let mut validators = Vec::<Validator>::new();
    match (min, max) {
      (Some(mn), Some(mx)) => validators.push(Validator {
        validator_type: ValidatorType::AnswerCount,
        text: format!("選択肢は{}個以上{}以下", mn, mx),
        min_length: *min,
        max_length: *max,
        expression: None,
      }),
      (Some(mn), None) => validators.push(Validator {
        validator_type: ValidatorType::AnswerCount,
        text: format!("選択肢は{}個以上", mn),
        min_length: *min,
        max_length: *max,
        expression: None,
      }),
      (None, Some(mx)) => validators.push(Validator {
        validator_type: ValidatorType::AnswerCount,
        text: format!("選択肢は{}個以下", mx),
        min_length: *min,
        max_length: *max,
        expression: None,
      }),
      _ => (),
    }

    if let Some(placeholder) = placeholder_text {
      let exceptions = placeholder.split(',').collect::<Vec<&str>>();
      // If options empty or placeholder text(s) (exception) text not in options, return error
      for &exc in exceptions.iter() {
        if let Some(opts) = options {
          // if opts.is_empty() || !opts.contains(&exc.to_owned()) {
          // if opts.is_empty() || opts.iter().find(|&s| s.is_val(exc)).is_none() {
          if opts.is_empty() || !opts.iter().any(|s| s.is_val(exc)) {
            return Err(ConvertError::PlaceholderNotInOptions);
          }
        } else {
          return Err(ConvertError::PlaceholderNotInOptions);
        }
      }
      match exceptions.len() {
                0 => (),
                1 => validators.push(Validator{
                    validator_type: ValidatorType::Expression,
                    text: format!("[{}]が選択されています。", exceptions.get(0).unwrap()),
                    min_length: None,
                    max_length: None,
                    expression: Some(format!("${{{0}}} && (${{{0}}}.includes('{1}') && ${{{0}}}.length === 1) || !${{{0}}}.includes('{1}')",
                                        field_name, exceptions.get(0).unwrap())),
                }),
                _ => {
                    let formatted_exceptions_text = exceptions.iter()
                        .map(|&s| format!("[{}]", s))
                        .collect::<Vec<String>>().join("または");
                    let formatted_exceptions_expression = exceptions.into_iter()
                        .map(|s| format!("'{}'",s))
                        .collect::<Vec<String>>().join(", ");
                    validators.push(Validator{
                        validator_type: ValidatorType::Expression,
                        text: format!("{}が選択されています。", formatted_exceptions_text),
                        min_length: None,
                        max_length: None,
                        expression: Some(format!("${{{0}}} && (${{{0}}}.some(item => [{1}].includes(item)) && ${{{0}}}.length === 1) || !${{{0}}}.some(item => [{1}].includes(item))",
                                                 field_name, formatted_exceptions_expression))

                    })
                }
            }
    }

    Ok(validators)
  }
}
