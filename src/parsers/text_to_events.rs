

use crate::models::event::Event;
use crate::models::parser_options::HisotriaOptions;
use crate::parsers::{raw_to_event_string, event_string_to_event};
pub(crate) fn text_to_events(_input: &str, _opt: &HisotriaOptions) -> Vec<Event> {
    raw_to_event_string::text_to_raw_event_strings(_input, _opt)
        .into_iter()
        .map(
            move |s| {
                event_string_to_event::event_string_to_event_single(s, _opt)
            }
        ).filter(|e| {
        e.is_some()
    })
        .map(move |e| e.unwrap())
        .collect()
}


#[cfg(test)]
mod test_text_to_events {


    
    use super::*;

    use test_case::test_case;
    use crate::tests::default_options::{DEFAULT_CHINESE_OPT_SINGLE};
    

    #[test_case(
    r#"大貓熊（學名：Ailuropoda melanoleuca），也稱作大熊貓，一般稱為「貓熊」或「熊貓」，屬於食肉目熊科的一種哺乳動物，體色為黑白兩色。貓熊是中國特有物種，現存的主要棲息地是中國中西部四川盆地周邊的山區和陝西南部的秦嶺地區。全世界野生大貓熊現存大約有2060頭（2016年數據[2]）。2016年末，世界自然保護聯盟（IUCN）將大貓熊的受威脅等級從「瀕危級」降為「易危級」。[3]由於生育率低，大貓熊在中國瀕危動物紅皮書等級中評為瀕危物種，為中國大陸國寶。大貓熊被譽為生物界的活化石[4]。
大貓熊黑白相間的毛色和憨態可掬的外表使其深受人們喜愛，在全世界亦有大量粉絲。在1961年世界自然基金會成立時就以大貓熊為標誌，大貓熊儼然已成為了瀕危物種保護最重要的象徵；大貓熊也是中國在外交活動中向對方表示友好的重要代表，美國國際政治學者約瑟夫·奈爾更直言大貓熊被視為中國拓展軟實力的重要支柱，與英國的皇室家族有著異曲同工之妙[5]。"#,
    & 3)]
    fn test_to_raw_string_single(raw_text: &str, number_of_events: &usize) {
        let result = text_to_events(raw_text, &DEFAULT_CHINESE_OPT_SINGLE);
        assert_eq!(&result.iter().count(), number_of_events);
    }

    // #[test_case(
    // LONG_ARTICLE,
    // & 10
    // )]
    // fn test_to_raw_string_single_timed(raw_text: &str, number_of_events: &usize) {
    //
    //     let start = Instant::now();
    //
    //     let result = text_to_events(raw_text, &DEFAULT_CHINESE_OPT_SINGLE);
    //     let duration = start.elapsed();
    //     println!("Time elapsed in expensive_function() is: {:?}", duration);
    //
    //     println!("{}", serde_json::to_string_pretty(&result).unwrap());
    //     assert_eq!(&result.iter().count(), number_of_events);
    //     // result.iter().for_each(|e| {
    //     //
    //     //     println!("{}",serde_json::to_string(e).unwrap())
    //     // })
    // }

}

