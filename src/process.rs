use std::fs;
use crate::models::event::Event;
use crate::models::parser_options::HisotriaOptions;
use crate::parsers::text_to_events::text_to_events;
use crate::post_processors::post_process::post_process;

pub fn process(_input_text: &str, _opt: &HisotriaOptions) -> Vec<Event> {
    let events = text_to_events(_input_text, _opt);

    let events_after_post_process = post_process(events, _opt);
    events_after_post_process
}

pub fn process_file(file_path: &str, _opt: &HisotriaOptions) -> Vec<Event> {
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    process(&contents, _opt)
}


#[cfg(test)]
mod test_process {
    use crate::enums::languages::{Languages, MatchMode};
    use crate::enums::sort::Sort;
    use crate::models::parser_options::HisotriaOptions;
    use crate::process::process;
    
    
    use crate::utils::write_file::write_to_file;

    // #[test_case( LONG_ARTICLE )]
    fn test_process(_input: &str) {
        let opt = HisotriaOptions {
            language: Languages::CHINESE,
            match_mode: MatchMode::SINGLE,
            left_context_margin: 100,
            right_context_margin: 100,
            sort: Sort::Asc
        };
        let result = process(_input, &opt);

        assert!(!result.is_empty());
        write_to_file(&result, "test.json").err();

    }
}
