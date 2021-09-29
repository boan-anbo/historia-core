use crate::models::event::Event;
use crate::models::parser_options::HisotriaOptions;

// complete fields like "date" which can only be generated after other data are in place.
pub fn complete(mut _events: Vec<Event>, _opt: &HisotriaOptions) -> Vec<Event> {
    _events.iter_mut().for_each(|mut e| e.build_date());

    _events
}
