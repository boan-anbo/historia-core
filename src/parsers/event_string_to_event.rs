
use std::str::Chars;
use regex::Regex;
use crate::models::event::Event;
use crate::models::parser_options::HisotriaOptions;
use crate::consts::regex::{
    CHINESE_REGEX_CAPTURE_YEAR_SINGLE,
    CHINESE_REGEX_CAPTURE_MONTH_SINGLE,
    CHINESE_REGEX_CAPTURE_DAY_SINGLE,
    CHINESE_REGEX_CAPTURE_IS_DECADE,
    CHINESE_REGEX_CAPTURE_IS_CENTURY
};
use crate::enums::languages::{Languages, MatchMode};
use crate::models::event_date::EventDate;
use crate::models::raw_string::RawStrings;

pub(crate) fn event_string_to_event_single(_raw_strings: RawStrings, _opt: &HisotriaOptions) -> Option<Event> {
    let ref _text = _raw_strings.raw_event_string.clone();
    Some(Event {
        raw_strings: _raw_strings,
        start_year: get_year(_text, _opt),
        start_month: get_month(_text, _opt),
        start_day: get_day(_text, _opt),
        is_start_decade: is_single_event_string_decade(_text, _opt),
        is_start_century: is_single_event_string_century(_text, _opt),
        ..Default::default()
    })
}

// tells if the string contains decade marker e.g. 年代
fn is_single_event_string_decade(_input: &str, _opt: &HisotriaOptions) -> Option<bool> {
    let is_decade_regex: &Regex;
    match _opt.language {
        Languages::CHINESE => {
            is_decade_regex = &CHINESE_REGEX_CAPTURE_IS_DECADE
        }
    }
    Some(is_decade_regex.is_match(_input))
}

// tells if the string contains decade marker e.g. 世纪
fn is_single_event_string_century(_input: &str, _opt: &HisotriaOptions) -> Option<bool> {
    let is_decade_regex: &Regex;
    match _opt.language {
        Languages::CHINESE => {
            is_decade_regex = &CHINESE_REGEX_CAPTURE_IS_CENTURY
        }
    }
    Some(is_decade_regex.is_match(_input))
}

fn get_year(_text: &str, _opt: &HisotriaOptions) -> Option<EventDate> {
    let year_regex: &Regex;
    match _opt.language {
        Languages::CHINESE => {
            match _opt.match_mode {
                MatchMode::SINGLE => {
                    year_regex = &CHINESE_REGEX_CAPTURE_YEAR_SINGLE
                }
                MatchMode::CONNECTED => {
                    year_regex = &CHINESE_REGEX_CAPTURE_YEAR_SINGLE
                }
            }
        }
    }
    year_regex
        .captures(_text)
        .map_or(
            None,
            |m| {
                Option::Some(
                    str_to_int(
                        m.get(1)?.as_str(), _opt,
                    )
                )
            },
        )
}


fn get_month(_text: &str, _opt: &HisotriaOptions) -> Option<EventDate> {
    let year_regex: &Regex;
    match _opt.language {
        Languages::CHINESE => {
            match _opt.match_mode {
                MatchMode::SINGLE => {
                    year_regex = &CHINESE_REGEX_CAPTURE_MONTH_SINGLE
                }
                MatchMode::CONNECTED => {
                    year_regex = &CHINESE_REGEX_CAPTURE_MONTH_SINGLE
                }
            }
        }
    }
    year_regex
        .captures(_text)
        .map_or(
            None,
            |m| {
                Option::Some(
                    str_to_int(
                        m.get(1)?.as_str(), _opt,
                    )
                )
            },
        )
}

fn get_day(_text: &str, _opt: &HisotriaOptions) -> Option<EventDate> {
    let year_regex: &Regex;
    match _opt.language {
        Languages::CHINESE => {
            match _opt.match_mode {
                MatchMode::SINGLE => {
                    year_regex = &CHINESE_REGEX_CAPTURE_DAY_SINGLE
                }
                MatchMode::CONNECTED => {
                    year_regex = &CHINESE_REGEX_CAPTURE_DAY_SINGLE
                }
            }
        }
    }
    year_regex
        .captures(_text)
        .map_or(
            None,
            |m| {
                Option::Some(
                    str_to_int(
                        m.get(1)?.as_str(), _opt,
                    )
                )
            },
        )
}

fn str_to_int(_input: &str, _opt: &HisotriaOptions) -> EventDate {
    let _input_arabic = normalize_number_string(_input, _opt);
    EventDate {
        number_string_raw: _input.to_string(),
        number_string_arabic: _input_arabic.to_string(),
        number: _input_arabic.parse::<isize>().unwrap_or(0),
        ..Default::default()
    }
}


fn normalize_number_string(_input: &str, _opt: &HisotriaOptions) -> String {
    let anon_matcher: fn((usize, char), original: &usize) -> Vec<char>;
    match _opt.language {
        Languages::CHINESE => {
            anon_matcher = |(i, x), char_size| match x {
                '零' => vec!['0'],
                '一' => vec!['1'],
                '二' => vec!['2'],
                '三' => vec!['3'],
                '四' => vec!['4'],
                '五' => vec!['5'],
                '六' => vec!['6'],
                '七' => vec!['7'],
                '八' => vec!['8'],
                '九' => vec!['9'],
                '十' => {
                    // if "十" appears first, and...
                    if i == 0 {
                        // it's the only digit, e.g. “十".
                        if char_size == &1 {
                            vec!['1', '0']
                        } else {
                            // if there are more than one char, meanings it's "十五" etc.
                            vec!['1']
                        }
                        // e.g. “二十”， “三十”
                    } else if i == 1 && char_size == &2 {
                        vec!['0']
                    } else {
                        // e.g. "二十三"， “五十三”
                        vec![]
                    }
                }
                _ => vec![x]
            };
        }
    }

    let chars: &Chars = &_input.chars();
    let chars_size = &chars.clone().count();
    let s: String = chars.clone()
        .enumerate()
        .map(
            |(i, x)| anon_matcher((i, x), &chars_size)
        )
        .flatten()
        .collect();

    s
}

#[cfg(test)]
mod test_raw_string_to_event {
    use super::*;

    use test_case::test_case;
    use crate::tests::default_options::{DEFAULT_CHINESE_OPT_SINGLE};

    #[test_case("中华人民共和国成立于49年10月1日,而不是50年", Some(49), Some(10), Some(1))]
    #[test_case("中华人民共和国成立于四九年十月一日,而不是五零年", Some(49), Some(10), Some(1))]
    #[test_case("我4号到的北京", None, None, Some(4))]
    #[test_case("我4日的北京", None, None, Some(4))]
    #[test_case("八十年代的发展", Some(80), None, None)]
    #[test_case("二十一世纪最重要的是人才", Some(21), None, None)]
    fn test_event_string_to_event(event_string: &str, year: Option<isize>, month: Option<isize>, day: Option<isize>) {
        let input = RawStrings{raw_event_string: event_string.to_string(), ..Default::default()};
        let result = event_string_to_event_single(input, &DEFAULT_CHINESE_OPT_SINGLE);
        assert!(result.is_some());
        println!("{}", serde_json::to_string(&result.as_ref().unwrap()).unwrap());
        if year.is_some() {
            assert_eq!(&year.unwrap(), &result.as_ref().unwrap().start_year.as_ref().unwrap().number);
        } else {
            assert!(&result.as_ref().as_ref().unwrap().start_year.is_none())
        }

        if month.is_some() {
            assert_eq!(&month.unwrap(), &result.as_ref().unwrap().start_month.as_ref().unwrap().number);
        } else {
            assert!(&result.as_ref().as_ref().unwrap().start_month.is_none())
        }

        if day.is_some() {
            assert_eq!(&day.unwrap(), &result.as_ref().unwrap().start_day.as_ref().unwrap().number);
        } else {
            assert!(&result.as_ref().as_ref().unwrap().start_day.is_none())
        }
    }

    #[test_case("八十年代是中国最好的年代", Some(true))]
    #[test_case("那是中国最好的年代", Some(false))]
    #[test_case("40年代", Some(true))]
    fn test_is_decade(_input: &str, is_decade: Option<bool>) {
        let result = is_single_event_string_decade(_input, &DEFAULT_CHINESE_OPT_SINGLE);
        assert_eq!(is_decade.unwrap(), result.unwrap())
    }

    #[test_case("十八世纪是个探险的时代", Some(true))]
    #[test_case("21世纪的人", Some(true))]
    #[test_case("二十一世纪的人", Some(true))]
    fn test_is_century(_input: &str, is_century: Option<bool>) {
        let result = is_single_event_string_century(_input, &DEFAULT_CHINESE_OPT_SINGLE);
        assert_eq!(is_century.unwrap(), result.unwrap())
    }

    #[test_case("中华人民共和国成立于49年10月1日,而不是50年","49", "49", 49; "test two raw strings")]
    #[test_case("1958年到1990年", "1958", "1958", 1958; "years only")]
    #[test_case("1958", "", "", 0; "exclude no year")]
    #[test_case("1961年", "1961", "1961", 1961)]
    #[test_case("二零三年", "二零三", "203", 203)]
    #[test_case("一九八五年", "一九八五", "1985", 1985)]
    #[test_case("零一年", "零一", "01", 1)]
    #[test_case("二零零三年", "二零零三", "2003", 2003)]
    #[test_case("五四三年", "五四三", "543", 543)]

    fn test_get_chinese_year_single(raw_text: &str, year_string_raw: &str, year_string_arabic: &str, year: isize) {
        let result = get_year(raw_text, &DEFAULT_CHINESE_OPT_SINGLE);
        if year == 0 {
            assert_eq!(true, result.is_none());
        } else {
            assert_eq!(year, result.as_ref().unwrap().number);
            assert_eq!(year_string_raw, result.as_ref().unwrap().number_string_raw.as_str());
            assert_eq!(year_string_arabic, result.as_ref().unwrap().number_string_arabic.as_str());
        };
    }


    #[test_case("中华人民共和国成立于1949年10月1日,而不是50年", 10)]
    #[test_case("93年6月我在", 6)]
    #[test_case("事件6月和7月间。", 6)]
    #[test_case("10月15日到11月30日", 10)]
    #[test_case("1958年到1990年", 0)]
    #[test_case("我7日到的美国，16日才安定下来", 0)]
    #[test_case("1970年3月开始一直到学习，中间经历了很多，一直到1989年八月", 3)]
    #[test_case("六月", 6)]
    #[test_case("十二月", 12)]
    #[test_case("十月", 10)]
    fn test_get_chinese_month_single(raw_text: &str, month: isize) {
        let result = get_month(raw_text, &DEFAULT_CHINESE_OPT_SINGLE);
        if month == 0 {
            assert_eq!(true, result.is_none());
        } else {
            assert_eq!(month, result.unwrap().number);
        };
    }


    #[test_case("中华人民共和国成立于1949年10月1日,而不是50年", 1)]
    #[test_case("93年6月我在", 0)]
    #[test_case("事件6月和7月间。", 0)]
    #[test_case("10月15日到11月30日", 15)]
    #[test_case("1958年到1990年", 0)]
    #[test_case("我7日到的美国，16日才安定下来", 7)]
    #[test_case("1970年3月开始一直到学习，中间经历了很多，一直到1989年八月", 0)]
    #[test_case("15到16.", 0; "should exclude strings without essential markers.")]
    #[test_case("五号", 5)]
    #[test_case("十日", 10)]
    #[test_case("十五日", 15)]
    #[test_case("二十日", 20)]
    #[test_case("二十五日", 25)]
    #[test_case("三十一日", 31)]
    fn test_get_chinese_day_single(raw_text: &str, day: isize) {
        let result = get_day(raw_text, &DEFAULT_CHINESE_OPT_SINGLE);
        if day == 0 {
            assert_eq!(true, result.is_none());
        } else {
            assert_eq!(day, result.unwrap().number);
        };
    }

    #[test_case("1915", "1915")]
    #[test_case("九三", "93")]
    #[test_case("二十三", "23")]
    #[test_case("二十", "20")]
    #[test_case("十", "10")]
    #[test_case("十九", "19")]
    #[test_case("零二", "02")]
    #[test_case("一九八五", "1985")]
    #[test_case("七", "7")]
    fn test_normalize_chinese_numbers(_input: &str, number: &str) {
        let result = normalize_number_string(_input, &DEFAULT_CHINESE_OPT_SINGLE);
        assert_eq!(number, result);
    }
}
