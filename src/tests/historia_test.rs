#[cfg(test)]
mod historia_test {
    use crate::enums::languages::LANGUAGES;
    use crate::models::parser_options::ParserOptions;
    use crate::parser::from_text;

    static DEFAULT_CHINESE_OPT: ParserOptions = ParserOptions {
        language: LANGUAGES::CHINESE
    };

    #[test]
    fn it_works() {
        let result = from_text("hola1", &DEFAULT_CHINESE_OPT);
        assert_eq!(false, result.is_empty());
    }
}
