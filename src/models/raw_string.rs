use neon::context::Context;
use neon::object::Object;
use neon::prelude::JsResult;
use neon::types::JsObject;
use serde::{Serialize, Deserialize};
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RawStrings {
    pub raw_event_string: String,
    pub raw_context_string: String
}
impl RawStrings {
    pub fn to_neon_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject>{
        let obj = cx.empty_object();

        let raw_event_string = cx.string(&self.raw_event_string);
        obj.set(cx, "rawEventString", raw_event_string).err();

        let raw_context_string = cx.string(&self.raw_context_string);
        obj.set(cx, "rawContextString", raw_context_string).err();

        Ok(obj)
    }
}
