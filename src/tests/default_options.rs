use crate::enums::languages::{Languages, MatchMode};
use crate::enums::sort::Sort;
use crate::models::parser_options::HisotriaOptions;

pub static DEFAULT_CHINESE_OPT_SINGLE: HisotriaOptions = HisotriaOptions {
    language: Languages::CHINESE,
    match_mode: MatchMode::SINGLE,
    left_context_margin: 50,
    right_context_margin: 50,
    sort: Sort::None
};

pub static DEFAULT_CHINESE_OPT_CONNECTED: HisotriaOptions = HisotriaOptions {
    language: Languages::CHINESE,
    match_mode: MatchMode::CONNECTED,
    left_context_margin: 25,
    right_context_margin: 25,
    sort: Sort::None
};
