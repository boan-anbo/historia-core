use neon::context::Context;
use neon::object::Object;
use neon::result::JsResult;
use neon::types::{JsNull, JsObject};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Debug, Clone, Deserialize, Serialize, PartialOrd, PartialEq)]
pub struct EventDate {
    pub(crate) number_string_raw: String,
    pub(crate) number_string_arabic: String,
    pub(crate) number: isize,
    pub is_inferred: bool,
    pub inferred_from: Option<Uuid>
}

impl Default for EventDate {
    fn default() -> Self {
        EventDate {
            number_string_raw: "".to_string(),
            number_string_arabic: "".to_string(),
            number: 0,
            is_inferred: false,
            inferred_from: None
        }
    }
}


impl EventDate {
    pub fn to_neon_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject>{
        let obj = cx.empty_object();

        let number_string_raw = cx.string(&self.number_string_raw);
        obj.set(cx, "numberStringRaw", number_string_raw).err();

        let number_string_arabic = cx.string(&self.number_string_arabic);
        obj.set(cx, "numberStringArabic", number_string_arabic).err();

        let number = cx.number(self.number as f64);
        obj.set(cx, "number", number).err();


        let is_inferred = cx.boolean(self.is_inferred);
        obj.set(cx, "isInferred", is_inferred).err();


        let inferred_from_field = "inferredFrom";
        if let Some(inferred_from) = &self.inferred_from {
            let value = cx.string(inferred_from.to_string());
            obj.set(cx, inferred_from_field, value).err();
        } else {
            let null = JsNull::new(cx);
            obj.set(cx, inferred_from_field, null).err();
        }

        Ok(obj)
    }
}
