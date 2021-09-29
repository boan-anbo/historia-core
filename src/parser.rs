use regex::{Captures};
use crate::models::event::Event;


use crate::models::parser_options::HisotriaOptions;
use crate::parsers::text_to_events::text_to_events;


pub fn from_text(input: &str, _opt: &HisotriaOptions) -> Vec<Event> {
    let _clean_text = input.trim();
    text_to_events(input, _opt)
}



