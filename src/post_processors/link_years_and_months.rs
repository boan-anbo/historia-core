use crate::models::event::Event;
use crate::models::parser_options::HisotriaOptions;

// inferring missing years and months based on previous dates.
pub fn link_event_dates<'a>(mut events: Vec<Event>, _opt: &'a HisotriaOptions) -> Vec<Event> {
    let candidate_events = &events.clone();
    for (pos, current_event) in events.iter_mut().enumerate() {

        // if without a day, there is no sense of linking month.
        if pos > 0 && current_event.start_day.is_some() && current_event.start_month.is_none() {
            *current_event = find_previous_event_with_month(candidate_events, pos, current_event).clone();
        }

        if pos > 0 && current_event.start_year.is_none() {
            *current_event = find_previous_event_with_year(candidate_events, pos, current_event).clone();
        }
    }
    events
}

pub fn find_previous_event_with_year<'a>(candidates: &Vec<Event>, current_event_pos: usize, current_event: &'a mut Event) -> &'a mut Event {
    let mut look_up_pos = current_event_pos.clone();
    loop {
        look_up_pos = look_up_pos.checked_sub(1).unwrap_or(0);

        match &candidates[look_up_pos] {
            candidate => {
                if candidate.start_year.is_some() {
                    current_event.start_year = candidate.start_year.clone();
                    current_event.start_year.as_mut().unwrap().is_inferred = true;
                    current_event.start_year.as_mut().unwrap().inferred_from = Option::from(candidate.get_id().clone());
                }
            }
        }
        if current_event.start_year.is_some() || current_event_pos == 0 || look_up_pos == 0 {
            break;
        }
    }
    current_event
}


pub fn find_previous_event_with_month<'a>(candidates: &Vec<Event>, current_event_pos: usize, current_event: &'a mut Event) -> &'a mut Event {
    let mut look_up_pos = current_event_pos.clone();
    loop {
        look_up_pos = look_up_pos.checked_sub(1).unwrap_or(0);

        match &candidates[look_up_pos] {
            candidate => {
                if candidate.start_month.is_some() {
                    current_event.start_month = candidate.start_month.clone();
                    current_event.start_month.as_mut().unwrap().is_inferred = true;
                    current_event.start_month.as_mut().unwrap().inferred_from = Option::from(candidate.get_id().clone());
                }
            }
        }
        if current_event.start_month.is_some() || current_event_pos == 0 || look_up_pos == 0 {
            break;
        }
    }
    current_event
}

#[cfg(test)]
mod test_link_years {
    
    
    use super::*;

    use test_case::test_case;
    use crate::parsers::text_to_events::text_to_events;
    use crate::tests::default_options::{DEFAULT_CHINESE_OPT_SINGLE};
    

    #[test_case("一九九一年五月2日到十月八日", 1991)]
    fn test_link_year(raw_text: &str, _second_event_year: usize) {
        let results = text_to_events(raw_text, &DEFAULT_CHINESE_OPT_SINGLE);
        // first check
        assert_eq!(results.iter().count(), 2);
        let old_first = results.get(0).unwrap().clone();
        assert!(old_first.start_year.is_some());
        let old_second = results.get(1).unwrap().clone();
        assert!(old_second.start_year.is_none());

        // link
        let newresults = link_event_dates(results, &DEFAULT_CHINESE_OPT_SINGLE);
        assert_eq!(newresults.iter().count(), 2);

        // re-check
        let new_second = newresults.get(1).unwrap();
        assert!(new_second.start_year.is_some());
        assert_eq!(
            new_second.start_year.as_ref().unwrap().number_string_arabic,
            old_first.start_year.as_ref().unwrap().number_string_arabic
        );
        // print
        println!("{}", serde_json::to_string_pretty(&newresults).unwrap());
    }
}