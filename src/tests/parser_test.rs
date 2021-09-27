#[cfg(test)]
mod parser_test {
    use crate::enums::languages::LANGUAGES;
    use crate::models::parser_options::ParserOptions;
    use crate::parser::from_text;
    use crate::tests::sample_texts::CHINESE_ONE_EVENT;

    static DEFAULT_CHINESE_OPT: ParserOptions = ParserOptions {
        language: LANGUAGES::CHINESE
    };

    #[test]
    fn should_extract_one_event() {
        let result = from_text(CHINESE_ONE_EVENT, &DEFAULT_CHINESE_OPT);
        assert_eq!(false, result.is_empty());
        assert_eq!("1949", result.first().unwrap().start_year_str?)
    }
}
