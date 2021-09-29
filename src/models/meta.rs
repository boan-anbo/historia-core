use chrono::{DateTime, Utc};
use neon::context::Context;
use neon::object::Object;
use neon::result::JsResult;
use neon::types::JsObject;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Meta {
    id: Uuid,
    created: DateTime<Utc>
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            id: Uuid::new_v4(),
            created: Utc::now()
        }
    }
}

impl Meta {
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl Meta {
    pub fn to_neon_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let id_field = "id";
        let id = cx.string(self.id.to_string().as_str());
        obj.set(cx, id_field, id);


        let created_field = "created";
        // rfc3339 is Chrono's implementation of the standarized time.
        let created = cx.string(self.created.to_rfc3339());
        obj.set(cx, created_field, created);


        Ok(obj)
    }
}
