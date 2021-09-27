use std::borrow::Borrow;
use std::ops::Deref;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use crate::models::event::Event;
use crate::models::parser_options::ParserOptions;

pub fn from_text(input: &str, _opt: &ParserOptions) -> Vec<Event> {
    let clean_text = input.trim();
    extract_events(input, _opt)
}

lazy_static! {
        static ref CHINESE_REGEX: Regex = Regex::new(r"(\d{4})").unwrap();
    }
fn extract_events(input: &str, opt: &ParserOptions) -> Vec<Event> {
    CHINESE_REGEX.captures_iter(input).map( move |cap| {
        capture_to_event(cap, opt)
    }).collect()
}

fn capture_to_event(caps: Captures, _opt: &ParserOptions) -> Event {
    let raw_string = caps.get(0).unwrap().as_str().to_string();
    let start_year = caps.get(1).map(|m| m.as_str().to_string());
    Event {
        raw_string,
        start_year_str: start_year
    }

}