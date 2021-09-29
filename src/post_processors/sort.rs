use crate::enums::sort::Sort;
use crate::models::event::Event;
use crate::models::parser_options::HisotriaOptions;

pub fn sort(mut _events: Vec<Event>, _opt: &HisotriaOptions) -> Vec<Event> {
    match _opt.sort {
        // return events directly
        Sort::None => _events,
        _ => {
            _events.sort_by(|a, b| {
                return match _opt.sort {
                    Sort::None => panic!("Cannot sort on none."),
                    Sort::Asc => {
                        let first = a.get_start_year_number().partial_cmp(&b.get_start_year_number());
                        let second = a.get_start_month_number().partial_cmp(&b.get_start_month_number());
                        let third = a.get_start_day_number().partial_cmp(&b.get_start_day_number());

                        first.unwrap().then(second.unwrap()).then(third.unwrap())
                    },
                    Sort::Desc => {
                        let first = b.get_start_year_number().partial_cmp(&a.get_start_year_number());
                        let second = b.get_start_month_number().partial_cmp(&a.get_start_month_number());
                        let third = b.get_start_day_number().partial_cmp(&a.get_start_day_number());

                        first.unwrap().then(second.unwrap()).then(third.unwrap())
                    }
                }
            });
            // return sorted events
            _events
        }
    }
}

#[cfg(test)]
mod test_sort {
    use crate::enums::sort::Sort;
    use crate::models::parser_options::HisotriaOptions;
    use crate::parsers::text_to_events::text_to_events;
    use crate::post_processors::sort::sort;
    use crate::tests::default_options::DEFAULT_CHINESE_OPT_SINGLE;

    #[test]
    fn test_sort() {
        let results = text_to_events("8月和6月和七月", &DEFAULT_CHINESE_OPT_SINGLE);
        let results_nature = sort(results, &DEFAULT_CHINESE_OPT_SINGLE);
        assert_eq!(results_nature.iter().count(), 3);
        assert_eq!(results_nature.first().unwrap().start_month.as_ref().unwrap().number, 8);

        let asc_sort_option = HisotriaOptions {
            sort: Sort::Asc,
            ..Default::default()
        };

        let results_asc = sort(results_nature, &asc_sort_option);

        assert_eq!(results_asc.first().unwrap().start_month.as_ref().unwrap().number, 6);
    }


    #[test]
    fn test_sort_conditions() {
        let results = text_to_events("1992年2月和1991年4月和1991年5月和1991年3月和10月", &DEFAULT_CHINESE_OPT_SINGLE);
        let results_nature = sort(results, &DEFAULT_CHINESE_OPT_SINGLE);
        assert_eq!(results_nature.iter().count(), 5);
        assert_eq!(results_nature.first().unwrap().start_year.as_ref().unwrap().number, 1992);
        assert_eq!(results_nature.first().unwrap().start_month.as_ref().unwrap().number, 2);

        let asc_sort_option = HisotriaOptions {
            sort: Sort::Asc,
            ..Default::default()
        };

        let results_asc = sort(results_nature, &asc_sort_option);

        // second None 10, because None on year is "smaller" than say 1991.
        assert_eq!(results_asc.first().unwrap().start_month.as_ref().unwrap().number, 10);
        assert!(results_asc.first().unwrap().start_year.is_none());

        // second 1991 3
        assert_eq!(results_asc.get(1).unwrap().start_year.as_ref().unwrap().number, 1991);
        assert_eq!(results_asc.get(1).unwrap().start_month.as_ref().unwrap().number, 3);


        // last 1992 2
        assert_eq!(results_asc.last().unwrap().start_year.as_ref().unwrap().number, 1992);
        assert_eq!(results_asc.last().unwrap().start_month.as_ref().unwrap().number, 2);


        let desc_sort_option = HisotriaOptions {
            sort: Sort::Desc,
            ..Default::default()
        };

        let results_desc = sort(results_asc, &desc_sort_option);

        // first 1992 2
        assert_eq!(results_desc.first().unwrap().start_month.as_ref().unwrap().number, 2);
        assert_eq!(results_desc.first().unwrap().start_year.as_ref().unwrap().number, 1992);

        // second 1991 3
        assert_eq!(results_desc.get(1).unwrap().start_year.as_ref().unwrap().number, 1991);
        assert_eq!(results_desc.get(1).unwrap().start_month.as_ref().unwrap().number, 5);


        // last None 10
        assert!(results_desc.last().unwrap().start_year.is_none());
        assert_eq!(results_desc.last().unwrap().start_month.as_ref().unwrap().number, 10);
    }
}
