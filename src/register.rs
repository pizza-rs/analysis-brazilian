use alloc::boxed::Box;
use alloc::vec;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use crate::stem::BrazilianStemFilter;
use crate::stop::BrazilianStopFilter;

pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("brazilian_stem", Box::new(BrazilianStemFilter::new()));
    factory.register_token_filter("brazilian_stop", Box::new(BrazilianStopFilter::new()));

    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(BrazilianStopFilter::new()),
        Box::new(BrazilianStemFilter::new()),
    ];

    factory.register_analyzer(
        "brazilian",
        Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("brazilian_stem").is_some());
        assert!(factory.get_token_filter("brazilian_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("brazilian").is_some());
    }
}
