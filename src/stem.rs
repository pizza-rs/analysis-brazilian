use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Brazilian Portuguese stemmer based on the RSLP (Removedor de Sufixos da
/// Lingua Portuguesa) algorithm adapted for Brazilian Portuguese.
#[derive(Clone, Debug, Default)]
pub struct BrazilianStemFilter;

impl BrazilianStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for BrazilianStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 4 {
            return (false, None);
        }

        let stemmed = stem_brazilian(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_brazilian(word: &str) -> String {
    // Step 1: Plural reduction
    let mut s = reduce_plural(word);

    // Step 2: Feminine reduction
    s = reduce_feminine(&s);

    // Step 3: Remove common adverb suffix
    if s.ends_with("mente") && s.len() > 8 {
        s = s[..s.len() - 5].to_string();
        return s;
    }

    // Step 4: Augmentative/Diminutive
    s = reduce_augmentative(&s);

    // Step 5: Noun suffixes
    s = reduce_noun_suffix(&s);

    s
}

fn reduce_plural(word: &str) -> String {
    if word.ends_with("ões") || word.ends_with("ães") {
        let base = &word[..word.len() - "ões".len()];
        if base.len() >= 3 {
            return format!("{}ão", base);
        }
    }
    if word.ends_with("ais") && word.len() > 4 {
        return format!("{}al", &word[..word.len() - 3]);
    }
    if word.ends_with("éis") && word.len() > 4 {
        return format!("{}el", &word[..word.len() - "éis".len()]);
    }
    if word.ends_with("eis") && word.len() > 4 {
        return format!("{}el", &word[..word.len() - 3]);
    }
    if word.ends_with("is") && word.len() > 4 {
        return format!("{}il", &word[..word.len() - 2]);
    }
    if word.ends_with('s') && word.len() > 3 {
        return word[..word.len() - 1].to_string();
    }
    word.to_string()
}

fn reduce_feminine(word: &str) -> String {
    if word.ends_with("osa") && word.len() > 5 {
        return format!("{}oso", &word[..word.len() - 3]);
    }
    if word.ends_with("ica") && word.len() > 5 {
        return format!("{}ico", &word[..word.len() - 3]);
    }
    if word.ends_with("ada") && word.len() > 5 {
        return format!("{}ado", &word[..word.len() - 3]);
    }
    if word.ends_with("ida") && word.len() > 5 {
        return format!("{}ido", &word[..word.len() - 3]);
    }
    if word.ends_with('a')
        && word.len() > 3
        && word.as_bytes()[word.len() - 2].is_ascii_alphabetic()
    {
        return word[..word.len() - 1].to_string();
    }
    word.to_string()
}

fn reduce_augmentative(word: &str) -> String {
    let suffixes: &[(&str, &str)] = &[
        ("inho", ""),
        ("inha", ""),
        ("zinho", ""),
        ("zinha", ""),
        ("ão", ""),
        ("ona", ""),
    ];

    for &(suffix, _) in suffixes {
        if word.ends_with(suffix) && word.len() - suffix.len() >= 3 {
            return word[..word.len() - suffix.len()].to_string();
        }
    }
    word.to_string()
}

fn reduce_noun_suffix(word: &str) -> String {
    let suffixes: &[&str] = &[
        "amento", "imento", "adora", "ência", "ância", "mente", "ador", "ição", "ação", "eza",
        "oso", "osa", "ico", "ica", "ivo", "iva",
    ];

    for suffix in suffixes {
        if word.ends_with(suffix) && word.len() - suffix.len() >= 3 {
            return word[..word.len() - suffix.len()].to_string();
        }
    }
    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_plural() {
        let filter = BrazilianStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("gatos"),
            start_offset: 0,
            end_offset: 5,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "gato");
    }

    #[test]
    fn test_stem_feminine() {
        let filter = BrazilianStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("bonita"),
            start_offset: 0,
            end_offset: 6,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "bonit");
    }
}
