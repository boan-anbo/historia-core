
use lazy_static::lazy_static;
use regex::{Regex};


 lazy_static! {
        pub static ref CHINESE_REGEX_RAW_STRING_CONNECTED: Regex = Regex::new(
         r"\d{1,4}[年世]?(?:(?:\d{1,2})?月?)?(?:(?:\d{1,2})?日?)?.{0,8}\d{1,4}年?(?:(?:\d{1,2})?月?)?(?:(?:\d{1,2})?日?)?"
     ).unwrap();
            //式 is there to capture "世纪"
        pub static ref CHINESE_REGEX_RAW_STRING_SINGLE: Regex = Regex::new(
         r"(?:[\d一二三四五六七八九十零]{1,2}世纪)?[\d一二三四五六七八九十零]{1,4}[年世]?[纪代]?(?:(?:[1-3一二三十]{0,1}[0-9一二三四五六七八九十]{1})?月?)?(?:(?:[1-3一二三十]{0,1}[0-9一二三四五六七八九十]{1}[一二三四五六七八九]{0,1})?[日号]?)?"
     ).unwrap();
        pub static ref CHINESE_REGEX_CAPTURE_YEAR_SINGLE: Regex = Regex::new(r"([\d零一二三四五六七八九十]{1,4})[年世]").unwrap();
        pub static ref CHINESE_REGEX_CAPTURE_MONTH_SINGLE: Regex = Regex::new(r"([1-3一二三十]{0,1}[0-9一二三四五六七八九十]{1})月").unwrap();
        pub static ref CHINESE_REGEX_CAPTURE_DAY_SINGLE: Regex = Regex::new(r"([1-3一二三十]{0,1}[0-9一二三四五六七八九十]{1}[一二三四五六七八九]{0,1})[日号]").unwrap();
        pub static ref CHINESE_REGEX_CAPTURE_IS_DECADE: Regex = Regex::new(r"[\d零一二三四五六七八九十]{1}年代").unwrap();
        pub static ref CHINESE_REGEX_CAPTURE_IS_CENTURY: Regex = Regex::new(r"[\d零一二三四五六七八九十]{1}世纪").unwrap();
        // tells whether a "suspect" raw string is indeed a match. in CHinese case, it must has at least 年，月，or 日.
        pub static ref CHINESE_REGEX_ESSENTIAL_MARKERS: Regex = Regex::new(r"[年月日号]").unwrap();
    }

