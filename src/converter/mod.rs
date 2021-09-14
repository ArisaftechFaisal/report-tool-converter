use crate::converter::error::ConvertError;
use napi::{CallContext, Env, JsNumber, JsObject, JsUnknown, Result, Task};
use serde::{Deserialize, Serialize};

mod error;
mod field;
mod reader;
mod subject;
mod test;
mod writer;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ConvertAsync {
  input_path: String,
  output_path: String,
}

impl Task for ConvertAsync {
  type Output = i32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    convert(&self.input_path, &self.output_path)?;
    Ok(1)
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_int32(output)
  }
}

#[js_function(1)]
pub fn convert_async(ctx: CallContext) -> Result<JsObject> {
  let arg0 = ctx.get::<JsUnknown>(0)?;
  let task: ConvertAsync = ctx.env.from_js_value(arg0)?;
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}

fn convert(input_path: &str, ouput_path: &str) -> error::Result<()> {
  let page = reader::parse(input_path)?;
  writer::write_to_file(page, ouput_path)?;
  Ok(())
}

impl From<ConvertError> for napi::Error {
  fn from(err: ConvertError) -> Self {
    // napi::Error::from_reason(format!("{:?}", err))
    napi::Error::from_reason(err.to_string())
  }
}
