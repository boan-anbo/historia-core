use crate::models::event::Event;
use crate::models::parser_options::HisotriaOptions;
use crate::post_processors::complete_fields::complete;
use crate::post_processors::link_years_and_months::link_event_dates;
use crate::post_processors::sort::sort;

pub fn post_process(_events: Vec<Event>, _opt: &HisotriaOptions) -> Vec<Event> {

    let events_after_linking = link_event_dates(_events, _opt);

    let events_after_completion = complete(events_after_linking, _opt);

    let event_after_sorting = sort(events_after_completion, _opt);

    event_after_sorting
}