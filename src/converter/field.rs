use std::str::FromStr;
use super::error::{ParseError, Result};
use super::subject::Subject::Placeholder;
use calamine::DataType;
use serde::{Serialize, Deserialize};
use serde::de::Expected;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Field {
    #[serde(rename = "QuestionKey")]
    field_name: String,
    #[serde(rename = "Required")]
    is_required: bool,
    #[serde(rename = "Type")]
    variant: FieldVariant,
    #[serde(rename = "Label")]
    label: String,
    #[serde(skip)]
    placeholder_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_specification: Option<InputSpecification>,
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    options: Option<Vec<String>>,
    #[serde(rename = "OptionsFromKey", skip_serializing_if = "Option::is_none")]
    options_from_key: Option<String>,
    #[serde(skip)]
    max: Option<u64>,
    #[serde(skip)]
    min: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_condition_first: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_condition_second: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_condition_third: Option<Vec<String>>,
    #[serde(flatten)]
    processed: ProcessedData,
}

impl Field {
    pub fn new(
        is_required: bool,
        field_name: String,
        variant: FieldVariant,
        label: String,
        placeholder_text: Option<String>,
        input_specification: Option<InputSpecification>,
        options: Option<Vec<String>>,
        options_from_key: Option<String>,
        max: Option<u64>,
        min: Option<u64>,
        display_condition_first: Option<Vec<String>>,
        display_condition_second: Option<Vec<String>>,
        display_condition_third: Option<Vec<String>>,
    ) -> Result<Self> {
        let mut validators: Option<Vec<Validator>>;
        let price_max: Option<u64>;
        let mut placeholder: Option<String> = None;
        let mut options_key: Option<String> = None;

        // Placeholder logic
        if let Some(ref plc_text) = placeholder_text {
            match variant {
                FieldVariant::Text | FieldVariant::TextArea => {
                    placeholder = Some(plc_text.to_owned());
                },
                FieldVariant::Dropdown => {
                    options_key = Some(plc_text.to_owned())
                },
                _ => (),
            }
        }

        // PriceMax logic
        if let None = max {
            price_max = None;
        } else if let FieldVariant::TextArea = variant {
            price_max = Some(max.unwrap());
        } else {
            price_max = None;
        }

        // Validators logic
        match variant {
            FieldVariant::Text => {
                validators = Some(Self::text_validators(&field_name, &min, &max, &input_specification));
            },
            FieldVariant::TextArea => {
                validators = Some(Self::textarea_validators(&min));
            },
            FieldVariant::Multiselect => {
                validators = Some(Self::multiselect_validators(&field_name, &min, &max, &placeholder_text, &options)?);
            },
            _ => {
                validators = None;
            }
        }
        if let Some(ref vlds) = validators {
            if vlds.is_empty() { validators = None; }
        }


         Ok(Field {
            is_required,
             field_name,
             variant,
             label,
             placeholder_text,
             input_specification,
             options,
             options_from_key,
             max,
             min,
             display_condition_first,
             display_condition_second,
             display_condition_third,
             processed: ProcessedData {
                 validators,
                 price_max,
                 placeholder,
                 options_key
             }
         })
    }

    fn text_validators(
        field_name: &String,
        min: &Option<u64>,
        max: &Option<u64>,
        input_specification: &Option<InputSpecification>
    ) -> Vec<Validator> {
        let mut validators = Vec::<Validator>::new();
        match (min.clone(), max.clone()) {
            (Some(mn), Some(mx)) => {
                if mn == mx {
                    validators.push(Validator {
                       validator_type: ValidatorType::Text,
                        text: format!("{}文字で入力してください", mx),
                        min_length: min.clone(),
                        max_length: max.clone(),
                        expression: None
                    });
                } else if mn == 1 && mx == 2 {
                    match input_specification {
                        Some(InputSpecification::HalfWidthNumber) => {
                            validators.push(Validator {
                                validator_type: ValidatorType::Expression,
                                text: "1歳以上100歳未満で入力してください。".to_owned(),
                                min_length: None,
                                max_length: None,
                                expression: Some(format!("${{{0}}} && ${{{0}}} >= 1 && ${{{0}}} < 100", field_name))
                            });
                        },
                        _ => ()
                    }
                }
            },
            _ => ()
        }

        if let Some(inp_spec) = input_specification {
           match inp_spec {
                InputSpecification::HalfWidthNumber => {
                    if let (Some(mn), Some(mx)) = (min.clone(), max.clone()) {
                        if mx == 2 && mn == 1 {
                           validators.push(Validator {
                                validator_type: ValidatorType::Expression,
                                text: "入力できるのは半角数字のみです".to_owned(),
                                min_length: None,
                                max_length: None,
                                expression: Some(format!("${{{0}}} && ${{{0}}}.match(/^[0-9]+$/)", field_name))
                            });
                        }
                    }
                },
                InputSpecification::HalfWidthKanji => {
                    validators.push(Validator {
                        validator_type: ValidatorType::Expression,
                        text: "入力できるのは半角英字のみです".to_owned(),
                        min_length: None,
                        max_length: None,
                        expression: Some(format!("${{{0}}} && ${{{0}}}.match(/^([a-zA-Z])+$/)", field_name))
                    });
                }
            }
        }

        validators
    }

    fn textarea_validators(
        min: &Option<u64>,
    ) -> Vec<Validator> {
        let mut validators = Vec::<Validator>::new();

        if let Some(mn) = min {
            validators.push(Validator {
                validator_type: ValidatorType::Text,
                text: format!("{}文字以上で入力してください", mn),
                min_length: min.clone(),
                max_length: None,
                expression: None
            });
        }
        validators
    }

    fn multiselect_validators(
        field_name: &String,
        min: &Option<u64>,
        max: &Option<u64>,
        placeholder_text: &Option<String>,
        options: &Option<Vec<String>>
    ) -> Result<Vec<Validator>> {
        let mut validators = Vec::<Validator>::new();
        match (min, max) {
            (Some(mn), Some(mx)) => validators.push(Validator {
                validator_type: ValidatorType::AnswerCount,
                text: format!("選択肢は{}個以上{}以下", mn, mx),
                min_length: min.clone(),
                max_length: max.clone(),
                expression: None,
            }),
            (Some(mn), None) => validators.push(Validator{
                validator_type: ValidatorType::AnswerCount,
                text: format!("選択肢は{}個以上", mn),
                min_length: min.clone(),
                max_length: max.clone(),
                expression: None,
            }),
            (None, Some(mx)) => validators.push(Validator{
                validator_type: ValidatorType::AnswerCount,
                text: format!("選択肢は{}個以下", mx),
                min_length: min.clone(),
                max_length: max.clone(),
                expression: None,
            }),
            _ => ()
        }

        if let Some(placeholder) = placeholder_text {
            let exceptions = placeholder.split(",").collect::<Vec<&str>>();
            // If options empty or placeholder text(s) (exception) text not in options, return error
            for &exc in exceptions.iter() {
                if let Some(opts) = options {
                    if opts.is_empty() || !opts.contains(&exc.to_owned()) { return Err(ParseError::PlaceholderNotInOptions); }
                } else {
                    return Err(ParseError::PlaceholderNotInOptions);
                }
            }
            match exceptions.len() {
                0 => (),
                1 => validators.push(Validator{
                    validator_type: ValidatorType::Expression,
                    text: format!("{}が選択されています。", exceptions.get(0).unwrap()),
                    min_length: None,
                    max_length: None,
                    expression: Some(format!("${{{0}}} && (${{{0}}}.includes('{1}') && ${{{0}}}.length === 1) || !${{{0}}}.includes('{1}')",
                                        field_name, exceptions.get(0).unwrap())),
                }),
                _ => {
                    let first_exception = exceptions.get(0).unwrap().clone();
                    let formatted_exceptions = exceptions.into_iter()
                        .map(|s| format!("'{}'",s))
                        .collect::<Vec<String>>();
                    let formatted_exceptions = formatted_exceptions.join(", ");
                    validators.push(Validator{
                        validator_type: ValidatorType::Expression,
                        text: format!("{}が選択されています。", first_exception),
                        min_length: None,
                        max_length: None,
                        expression: Some(format!("${{{0}}} && (${{{0}}}.some(item => [{1}].includes(item)) && ${{{0}}}.length === 1) || !${{{0}}}.some(item => [{1}].includes(item))",
                                                 field_name, formatted_exceptions))

                    })
                }
            }
        }

        Ok(validators)
    }

    pub fn field_number_from_datatype(dt: &DataType) -> Result<usize> {
        match dt {
            DataType::String(s) => {
                Ok(s.trim_start_matches("field").parse::<usize>()?)
            },
            DataType::Int(i) => Ok(*i as usize),
            _ => Err(ParseError::ExpectedIntOrString)
        }
    }

    pub fn required_from_datatype(dt: &DataType) -> Result<Option<bool>> {
       match dt {
           DataType::Empty => Ok(None),
           DataType::String(s) if s.is_empty() => Ok(None),
           DataType::String(s) => {
               match s.as_str() {
                   "表示(必須)" => Ok(Some(true)),
                   "表示(任意)" => Ok(Some(false)),
                    unknown_string => Err(ParseError::IncorrectRequired(unknown_string.to_owned())),
               }
           },
           _ => Err(ParseError::ExpectedString)
       }
    }

    pub fn variant_from_datatype(dt: &DataType) -> Result<FieldVariant> {
        match dt {
            DataType::String(s) => Ok(s.parse::<FieldVariant>()?),
            _ => Err(ParseError::ExpectedString)
        }
    }

    pub fn label_from_datatype(dt: &DataType) -> Result<String> {
        match dt {
            DataType::String(s) => Ok(s.to_owned()),
            _ => Err(ParseError::ExpectedString)
        }
    }

    pub fn input_specification_from_datatype(dt: &DataType) -> Result<Option<InputSpecification>> {
        match dt {
            DataType::Empty => Ok(None),
            DataType::String(s) if s.is_empty() => Ok(None),
            DataType::String(s) => Ok(Some(s.parse::<InputSpecification>()?)),
            _ => Err(ParseError::ExpectedString)
        }
    }

    pub fn optional_string_from_datatype(dt: &DataType) -> Result<Option<String>> {
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

    pub fn optional_u64_from_datatype(dt: &DataType) -> Result<Option<u64>> {
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

    pub fn vec_to_optional_vec<T>(v: Vec<T>) -> Option<Vec<T>> {
        match v.len() {
            0 => None,
            _ => Some(v)
        }
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// pub(crate) enum IsRequired {
//     True,
//     False,
//     NA
// }
//
// impl Default for IsRequired {
//     fn default() -> Self {
//         IsRequired::NA
//     }
// }
//
// impl FromStr for IsRequired {
//     type Err = ParseError;
//
//     fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//         match s {
//             "表示(必須)" => Ok(IsRequired::True),
//             "表示(任意)" => Ok(IsRequired::False),
//             empty if empty.is_empty() => Ok(IsRequired::NA),
//             unknown_required => Err(ParseError::IncorrectRequired(unknown_required.to_owned()))
//         }
//     }
// }

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
            unknown_string => Err(ParseError::IncorrectFieldVariant(unknown_string.to_owned()))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum InputSpecification {
    HalfWidthNumber,
    HalfWidthKanji
}

impl FromStr for InputSpecification {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "半角数字" => Ok(InputSpecification::HalfWidthNumber),
            "半角英字" => Ok(InputSpecification::HalfWidthKanji),
            unknown_string => Err(ParseError::IncorrectInputSpecificationError(unknown_string.to_owned()))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ProcessedData {
    // #[serde(flatten)]
    // placeholder_variant: Option<PlaceholderVariant>,
    #[serde(rename = "Placeholder", skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
    #[serde(rename = "OptionsKey", skip_serializing_if = "Option::is_none")]
    options_key: Option<String>,
    #[serde(rename = "Validators", skip_serializing_if = "Option::is_none")]
    validators: Option<Vec<Validator>>,
    #[serde(rename = "PriceMax", skip_serializing_if = "Option::is_none")]
    price_max: Option<u64>
}

// #[derive(Serialize, Deserialize, Debug)]
// pub(crate) enum PlaceholderVariant {
//     #[serde(flatten)]
//     Normal {
//         #[serde(rename = "Placeholder")]
//         text: String
//     },
//     #[serde(flatten)]
//     Options {
//         #[serde(rename = "OptionsKey")]
//         text: String
//     },
// }

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Validator {
    #[serde(rename = "Type")]
    validator_type: ValidatorType,
    #[serde(rename = "Text")]
    text: String,
    #[serde(rename = "Expression", skip_serializing_if = "Option::is_none")]
    expression: Option<String>,
    #[serde(rename = "MinLength", skip_serializing_if = "Option::is_none")]
    min_length: Option<u64>,
    #[serde(rename = "MaxLength", skip_serializing_if = "Option::is_none")]
    max_length: Option<u64>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum ValidatorType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "expression")]
    Expression,
    #[serde(rename = "answercount")]
    AnswerCount
}
