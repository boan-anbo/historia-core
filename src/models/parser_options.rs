use crate::enums::languages::{Languages, MatchMode};
use crate::enums::sort::Sort;

#[derive(Deserialize, Serialize, Debug)]
pub struct HisotriaOptions {
    pub language: Languages,
    pub match_mode: MatchMode,
    pub left_context_margin: usize,
    pub right_context_margin: usize,
    pub sort: Sort,
}


impl Default for HisotriaOptions {
    fn default() -> Self {
        HisotriaOptions {
            language: Languages::CHINESE,
            match_mode: MatchMode::SINGLE,
            left_context_margin: 26,
            right_context_margin: 26,
            sort: Sort::None
        }
    }
}