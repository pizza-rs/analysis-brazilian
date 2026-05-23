//! Brazilian Portuguese stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Brazilian Portuguese stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "a",
    "ao",
    "aos",
    "aquela",
    "aquelas",
    "aquele",
    "aqueles",
    "aquilo",
    "as",
    "até",
    "com",
    "como",
    "da",
    "das",
    "de",
    "dela",
    "delas",
    "dele",
    "deles",
    "depois",
    "do",
    "dos",
    "e",
    "ela",
    "elas",
    "ele",
    "eles",
    "em",
    "entre",
    "era",
    "eram",
    "essa",
    "essas",
    "esse",
    "esses",
    "esta",
    "estamos",
    "estas",
    "estava",
    "estavam",
    "este",
    "esteja",
    "estejam",
    "estejamos",
    "estes",
    "esteve",
    "estive",
    "estivemos",
    "estiver",
    "estivera",
    "estiveram",
    "estiverem",
    "estivermos",
    "estivesse",
    "estivessem",
    "estivéramos",
    "estivéssemos",
    "estou",
    "está",
    "estávamos",
    "estão",
    "eu",
    "foi",
    "fomos",
    "for",
    "fora",
    "foram",
    "forem",
    "formos",
    "fosse",
    "fossem",
    "fui",
    "fôramos",
    "fôssemos",
    "haja",
    "hajam",
    "hajamos",
    "havemos",
    "hei",
    "houve",
    "houvemos",
    "houver",
    "houvera",
    "houveram",
    "houverei",
    "houverem",
    "houveremos",
    "houveria",
    "houveriam",
    "houvermos",
    "houverá",
    "houverão",
    "houveríamos",
    "houvesse",
    "houvessem",
    "houvéramos",
    "houvéssemos",
    "há",
    "hão",
    "isso",
    "isto",
    "já",
    "lhe",
    "lhes",
    "mais",
    "mas",
    "me",
    "mesmo",
    "meu",
    "meus",
    "minha",
    "minhas",
    "muito",
    "na",
    "nas",
    "nem",
    "no",
    "nos",
    "nossa",
    "nossas",
    "nosso",
    "nossos",
    "num",
    "numa",
    "não",
    "nós",
    "o",
    "os",
    "ou",
    "para",
    "pela",
    "pelas",
    "pelo",
    "pelos",
    "por",
    "qual",
    "quando",
    "que",
    "quem",
    "se",
    "seja",
    "sejam",
    "sejamos",
    "sem",
    "serei",
    "seremos",
    "seria",
    "seriam",
    "será",
    "serão",
    "seríamos",
    "seu",
    "seus",
    "somos",
    "sou",
    "sua",
    "suas",
    "são",
    "só",
    "também",
    "te",
    "tem",
    "temos",
    "tenha",
    "tenham",
    "tenhamos",
    "tenho",
    "terei",
    "teremos",
    "teria",
    "teriam",
    "terá",
    "terão",
    "teríamos",
    "teu",
    "teus",
    "teve",
    "tinha",
    "tinham",
    "tive",
    "tivemos",
    "tiver",
    "tivera",
    "tiveram",
    "tiverem",
    "tivermos",
    "tivesse",
    "tivessem",
    "tivéramos",
    "tivéssemos",
    "tu",
    "tua",
    "tuas",
    "tém",
    "tínhamos",
    "um",
    "uma",
    "você",
    "vocês",
    "vos",
    "à",
    "às",
    "éramos",
    ];
    words.iter().copied().collect()
});

/// Removes Brazilian Portuguese stop words from the token stream.
#[derive(Clone, Debug)]
pub struct BrazilianStopFilter {
    stop_words: HashSet<String>,
}

impl Default for BrazilianStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl BrazilianStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for BrazilianStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 203);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = BrazilianStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = BrazilianStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = BrazilianStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
