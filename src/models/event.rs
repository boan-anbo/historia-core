use chrono::{NaiveDate, TimeZone, Utc};
use neon::context::{Context};
use ts_rs::{TS, export};

use neon::object::Object;
use neon::result::JsResult;
use neon::types::{JsNull, JsObject};
use crate::models::event_date::EventDate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::meta::Meta;
use crate::models::raw_string::RawStrings;

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct Event {
    pub meta: Meta,
    pub raw_strings: RawStrings,
    pub start_year: Option<EventDate>,
    pub start_month: Option<EventDate>,
    pub start_day: Option<EventDate>,
    pub is_start_decade: Option<bool>,
    pub is_start_century: Option<bool>,
    pub start_date: Option<NaiveDate>,
}

impl Default for Event {
    fn default() -> Self {
        Event {
            meta: Meta::default(),
            raw_strings: Default::default(),
            start_year: None,
            start_month: None,
            start_day: None,
            is_start_decade: None,
            is_start_century: None,
            start_date: None,
        }
    }
}

// date utility
impl Event {
    pub fn build_date(&mut self) {
        if self.start_year.is_some() && self.start_month.is_some() && self.start_day.is_some() {
            self.start_date = Some(
                Utc.ymd(
                    self.start_year.as_ref().unwrap().number as i32,
                    self.start_month.as_ref().unwrap().number.clone() as u32,
                    self.start_day.as_ref().unwrap().number.clone() as u32,
                ).naive_utc()
            )
        };
    }
}

impl Event {
    pub fn get_id(&self) -> &Uuid {
        self.meta.get_id()
    }

    pub fn get_start_year_number(&self) -> Option<isize> {
        match &self.start_year {
            None => None,
            Some(start_year) => {
                Some(start_year.number)
            }
        }
    }

    pub fn get_start_month_number(&self) -> Option<isize> {
        match &self.start_month {
            None => None,
            Some(start_month) => {
                Some(start_month.number)
            }
        }
    }

    pub fn get_start_day_number(&self) -> Option<isize> {
        match &self.start_day {
            None => None,
            Some(start_day) => {
                Some(start_day.number)
            }
        }
    }
}

impl Event {
    pub fn to_neon_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let raw_strings = self.raw_strings.to_neon_object(cx).unwrap();
        obj.set(cx, "rawStrings", raw_strings).err();


        let meta = self.meta.to_neon_object(cx).unwrap();
        obj.set(cx, "meta", meta).err();


        let start_month_field = "startYear";
        if let Some(start_year) = &self.start_year {
            let start_year_value = start_year.to_neon_object(cx).unwrap();
            obj.set(cx, start_month_field, start_year_value).err();
        } else {
            let null = JsNull::new(cx);
            obj.set(cx, start_month_field, null).err();
        }


        let start_month_field = "startMonth";
        if let Some(start_month) = &self.start_month {
            let value = start_month.to_neon_object(cx).unwrap();
            obj.set(cx, start_month_field, value).err();
        } else {
            let null = JsNull::new(cx);
            obj.set(cx, start_month_field, null).err();
        }


        let start_day_field = "startDay";
        if let Some(start_day) = &self.start_day {
            let value = start_day.to_neon_object(cx).unwrap();
            obj.set(cx, start_day_field, value).err();
        } else {
            let null = JsNull::new(cx);
            obj.set(cx, start_day_field, null).err();
        }

        let is_start_decade_field = "isStartDecade";
        if let Some(is_start_decade) = &self.is_start_decade {
            let value = cx.boolean(*is_start_decade);
            obj.set(cx, is_start_decade_field, value).err();
        } else {
            let null = JsNull::new(cx);
            obj.set(cx, is_start_decade_field, null).err();
        }

        let is_start_century_field = "isStartCentury";
        if let Some(is_start_century) = &self.is_start_century {
            let value = cx.boolean(*is_start_century);
            obj.set(cx, is_start_century_field, value).err();
        } else {
            let null = JsNull::new(cx);
            obj.set(cx, is_start_century_field, null).err();
        }


        let start_date_field = "startDate";
        if let Some(start_date) = &self.start_date {
            let value = cx.string(start_date.to_string());
            obj.set(cx, start_date_field, value).err();
        } else {
            let null = JsNull::new(cx);
            obj.set(cx, start_date_field, null).err();
        }


        Ok(obj)
    }
}

export! {
            // Event => "./ts_models/event.ts",
            // RawStrings => "./ts_models/rawstrings.ts",
            // EventDate => "./ts_models/eventdate.ts",
            // Meta => "./ts_models/meta.ts",
    Event, RawStrings, EventDate, Meta => "./ts_models/event.ts"
        }

#[cfg(test)]
mod test_event {
    use chrono::Datelike;
    use crate::parsers::text_to_events::text_to_events;
    use crate::tests::default_options::DEFAULT_CHINESE_OPT_SINGLE;
    use crate::models::event::Event;
    use ts_rs::export;

    #[test]
    fn generate_event_typescript_definitions() {}

    #[test]
    fn test_event_start_date() {
        let mut result = text_to_events("1959年6月1日", &DEFAULT_CHINESE_OPT_SINGLE);
        assert_eq!(result.iter().count(), 1);
        assert!(result.first().unwrap().start_date.is_none());
        result.first_mut().unwrap().build_date();
        assert!(&result.first().unwrap().start_date.is_some());
        assert_eq!(*&result.first().unwrap().start_date.unwrap().year(), 1959);
        assert_eq!(*&result.first().unwrap().start_date.unwrap().month(), 6);
        assert_eq!(*&result.first().unwrap().start_date.unwrap().day(), 1);
        // assert_eq!(*&first.start_date.unwrap().year(), 1957);
    }
}