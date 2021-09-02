use napi::{CallContext, Env, JsBoolean, JsObject, JsUnknown, JsNumber, Result, Task};
use serde::{Deserialize, Serialize};

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
    Ok(1)
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_int32(output as i32)
  }
}

#[js_function(1)]
pub fn convert_async(ctx: CallContext) -> Result<JsObject> {
  let arg0 = ctx.get::<JsUnknown>(0)?;
  let task: ConvertAsync = ctx.env.from_js_value(arg0)?;
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}
