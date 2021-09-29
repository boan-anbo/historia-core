use regex::{Match, Regex};
use crate::enums::languages::{Languages, MatchMode};
use crate::models::parser_options::HisotriaOptions;
use crate::consts::regex::{CHINESE_REGEX_RAW_STRING_SINGLE, CHINESE_REGEX_RAW_STRING_CONNECTED, CHINESE_REGEX_ESSENTIAL_MARKERS};

use crate::models::raw_string::RawStrings;

pub(crate) fn text_to_raw_event_strings(_input: &str, _opt: &HisotriaOptions) -> Vec<RawStrings> {
    let raw_string_regex: &Regex;
    match _opt.language {
        Languages::CHINESE => {
            match _opt.match_mode {
                MatchMode::SINGLE => {
                    raw_string_regex = &CHINESE_REGEX_RAW_STRING_SINGLE
                }
                MatchMode::CONNECTED => {
                    raw_string_regex = &CHINESE_REGEX_RAW_STRING_CONNECTED
                }
            }
        }
    }
    raw_string_regex.find_iter(_input)
        .filter(|s| {
            has_essential_markers(&s.as_str(), _opt)
        })
        .map(move |m| {
            let raw_event_string = m.as_str().to_string();
            let raw_context_string = get_context_string(_input, m, _opt);
            RawStrings {
                raw_event_string,
                raw_context_string,
            }
        })
        .collect()
}


fn get_context_string(_passage: &str, m: Match, _opt: &HisotriaOptions) -> String {
    let passage_char_length = _passage.chars().count();

    let char_start = byte_to_char_index(_passage, m.start().clone());
    let left_margin = char_start.checked_sub(_opt.left_context_margin).unwrap_or(0);

    let char_end = byte_to_char_index(_passage, m.end().clone());
    let mut right_margin = char_end + _opt.right_context_margin;
    if right_margin > passage_char_length - 1 {
        right_margin = passage_char_length - 1
    }


    utf8_slice(_passage, left_margin, right_margin).unwrap_or(m.as_str()).to_string()
}

fn byte_to_char_index(_input: &str, byte_index: usize) -> usize {
    _input[0..(byte_index)].char_indices().count()
}

pub fn utf8_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    let mut iter = s.char_indices()
        .map(|(pos, _)| pos)
        .chain(Some(s.len()))
        .skip(start)
        .peekable();
    let start_pos = *iter.peek()?;
    for _ in start..end { iter.next(); }
    Some(&s[start_pos..*iter.peek()?])
}

fn has_essential_markers(_input: &str, _opt: &HisotriaOptions) -> bool {
    match _opt.language {
        Languages::CHINESE => {
            return CHINESE_REGEX_ESSENTIAL_MARKERS.is_match(_input);
        }
    }
}

#[cfg(test)]
mod parser_raw_string_test {
    use super::*;

    use test_case::test_case;
    use crate::tests::default_options::{DEFAULT_CHINESE_OPT_CONNECTED, DEFAULT_CHINESE_OPT_SINGLE};

    #[test_case("大宝和小宝", "大宝")]
    #[test_case("大宝和小宝", "宝和小")]
    fn test_utf_8_slice(input: &str, slice: &str) {
        let m = Regex::new(&*format!("{}", slice)).unwrap().find(input).unwrap();
        let start_char = byte_to_char_index(input, m.start());
        let end_char = byte_to_char_index(input, m.end());
        let actual = utf8_slice(input, start_char, end_char).unwrap();
        assert_eq!(slice, actual);
    }

    #[test_case("大宝和小包", 0, (0, '大'))]
    #[test_case("大宝和小包", 3, (1, '宝'))]
    fn test_byte_to_char_indices(input: &str, byte_start: usize, expected: (usize, char)) {
        let char_index = byte_to_char_index(input, byte_start);
        assert_eq!(expected.0, char_index)
    }

    #[test_case("中华人民共和国成立于1949年10月1日,而不是50年", "1949年10月1日"; "test two raw strings")]
    #[test_case("事件6月和7月间。", "6月"; "months only")]
    #[test_case("10月15日到11月30日", "10月15日"; "month and day only")]
    #[test_case("1958年到1990年", "1958年"; "years only")]
    #[test_case("我7日到的美国，16日才安定下来", "7日"; "connected days")]
    #[test_case("一九九五年二月十一日凌晨", "一九九五年二月十一日")]
    #[test_case("零三年十一月3日凌晨", "零三年十一月3日")]
    #[test_case("2月十一日", "2月十一日")]
    #[test_case("十九世纪80年代", "十九世纪80年代")]
    fn test_to_raw_string_single_event(raw_text: &str, match_string: &str) {
        let result = text_to_raw_event_strings(raw_text, &DEFAULT_CHINESE_OPT_SINGLE);
        assert_eq!(false, result.is_empty());
        assert_eq!(match_string, result.first().as_ref().unwrap().raw_event_string.as_str());
    }


    #[test_case(
    r#"大貓熊（學名：Ailuropoda melanoleuca），也稱作大熊貓，一般稱為「貓熊」或「熊貓」，屬於食肉目熊科的一種哺乳動物，體色為黑白兩色。貓熊是中國特有物種，現存的主要棲息地是中國中西部四川盆地周邊的山區和陝西南部的秦嶺地區。全世界野生大貓熊現存大約有2060頭（2016年數據[2]）。2016年末，世界自然保護聯盟（IUCN）將大貓熊的受威脅等級從「瀕危級」降為「易危級」。[3]由於生育率低，大貓熊在中國瀕危動物紅皮書等級中評為瀕危物種，為中國大陸國寶。大貓熊被譽為生物界的活化石[4]。
大貓熊黑白相間的毛色和憨態可掬的外表使其深受人們喜愛，在全世界亦有大量粉絲。在1961年世界自然基金會成立時就以大貓熊為標誌，大貓熊儼然已成為了瀕危物種保護最重要的象徵；大貓熊也是中國在外交活動中向對方表示友好的重要代表，美國國際政治學者約瑟夫·奈爾更直言大貓熊被視為中國拓展軟實力的重要支柱，與英國的皇室家族有著異曲同工之妙[5]。"#,
    & 3)]
    fn test_raw_text_to_raw_strings_single_event(_input: &str, _result_size: &usize) {
        let result = text_to_raw_event_strings(_input, &DEFAULT_CHINESE_OPT_SINGLE);
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
        assert_eq!(&result.iter().count(), _result_size)
    }

    #[test_case("中华人民共和国成立于1949年10月1日,而不是50年", "1949年10月1日,而不是50年"; "test two raw strings")]
    #[test_case("93年6月我在", "93年6月"; "no day")]
    #[test_case("事件6月和7月间。", "6月和7月"; "months only")]
    #[test_case("10月15日到11月30日", "10月15日到11月30日"; "month and day only")]
    #[test_case("1958年到1990年", "1958年到1990年"; "years only")]
    #[test_case("我7日到的美国，16日才安定下来", "7日到的美国，16日"; "connected days")]
    #[test_case("1970年3月开始一直到学习，中间经历了很多，一直到1989年八月", "1970年3月"; "should exclude far away one")]
    #[test_case("15到16.", ""; "should exclude strings without essential markers.")]
    fn test_to_raw_string_connected(raw_text: &str, match_string: &str) {
        let result = text_to_raw_event_strings(raw_text, &DEFAULT_CHINESE_OPT_CONNECTED);
        if match_string.is_empty() {
            assert_eq!(true, result.is_empty());
        } else {
            assert_eq!(false, result.is_empty());
            assert_eq!(match_string, result.first().as_ref().unwrap().raw_event_string.as_str());
        }
    }

    #[test_case("15到16.", false; "should exclude strings without essential markers1.")]
    #[test_case("1986", false; "should exclude strings without essential markers2.")]
    #[test_case("86年.", true; "should accept strings without essential markers1.")]
    #[test_case("86月.", true; "should accept strings without essential markers2.")]
    #[test_case("86日.", true; "should accept strings without essential markers3.")]
    fn should_test_if_has_essentail_markers(raw_text: &str, has_markers: bool) {
        let result = has_essential_markers(raw_text, &DEFAULT_CHINESE_OPT_SINGLE);
        assert_eq!(has_markers, result);
    }
}
