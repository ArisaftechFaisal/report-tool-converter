#[cfg(test)]
mod convert_tests {
  use crate::converter::reader::parse;
  use crate::converter::writer;

  static PATH_INPUT_DROPDOWN_TEST: &'static str = "resources/test_dropdown.xlsx";
  static PATH_OUTPUT_DROPDOWN_TEST: &'static str = "resources/test_dropdown_output.json";

  static PATH_INPUT_TEXT_TEST: &'static str = "resources/test_text.xlsx";
  static PATH_OUTPUT_TEXT_TEST: &'static str = "resources/test_text_output.json";

  static PATH_INPUT_TEXTAREA_TEST: &'static str = "resources/test_textarea.xlsx";
  static PATH_OUTPUT_TEXTAREA_TEST: &'static str = "resources/test_textarea_output.json";

  static PATH_INPUT_MULTISELECT_TEST: &'static str = "resources/test_multiselect.xlsx";
  static PATH_OUTPUT_MULTISELECT_TEST: &'static str = "resources/test_multiselect_output.json";

  static PATH_INPUT_RADIO_TEST: &'static str = "resources/test_radio.xlsx";
  static PATH_OUTPUT_RADIO_TEST: &'static str = "resources/test_radio_output.json";

  fn test_parse_write(input_path: &'static str, output_path: &'static str) {
    // let path = Path::new(constants::PATH_INPUT_DROPDOWN_TEST).to_str().unwrap();
    let parse_res = parse(input_path);
    if let Ok(inp) = parse_res {
      let write_res = writer::write_to_file(inp, output_path);
      if let Ok(_) = write_res {
        assert!(true);
      } else {
        assert!(false, "write failed");
      }
    } else {
      println!("{:?}", parse_res);
      assert!(false, "read/parse failed");
    }
  }

  #[test]
  fn test_dropdowns() {
    test_parse_write(PATH_INPUT_DROPDOWN_TEST, PATH_OUTPUT_DROPDOWN_TEST)
  }

  #[test]
  fn test_text() {
    test_parse_write(PATH_INPUT_TEXT_TEST, PATH_OUTPUT_TEXT_TEST)
  }

  #[test]
  fn test_textarea() {
    test_parse_write(PATH_INPUT_TEXTAREA_TEST, PATH_OUTPUT_TEXTAREA_TEST)
  }

  #[test]
  fn test_multiselect() {
    test_parse_write(PATH_INPUT_MULTISELECT_TEST, PATH_OUTPUT_MULTISELECT_TEST)
  }

  #[test]
  fn test_radio() {
    test_parse_write(PATH_INPUT_RADIO_TEST, PATH_OUTPUT_RADIO_TEST)
  }
}
