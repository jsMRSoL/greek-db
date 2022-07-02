use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref CHARS: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert(' ', " ");
        m.insert('\"', "\"");
        m.insert('.', ".");
        m.insert('-', "-");
        m.insert('\'', "'");
        m.insert('/', "/");
        m.insert('Ἀ', "*)a");
        m.insert('Ἄ', "*)/a");
        m.insert('Ἑ', "*(e");
        m.insert('Ἕ', "*(/e");
        m.insert('ἀ', "a)");
        m.insert('ἂ', "a)\\");
        m.insert('ἄ', "a)/");
        m.insert('ᾀ', "a)|");
        m.insert('ᾂ', "a)\\|");
        m.insert('ᾄ', "a)/|");
        m.insert('ἆ', "a)=");
        m.insert('ᾆ', "a)=|");
        m.insert('ἁ', "a(");
        m.insert('ἃ', "a(\\");
        m.insert('ἅ', "a(/");
        m.insert('ᾃ', "a(\\|");
        m.insert('ᾅ', "a(/|");
        m.insert('ἇ', "a(=");
        m.insert('ᾇ', "a(=|");
        m.insert('ᾶ', "a=");
        // m.insert('ὰ', "a\\");
        m.insert('ὰ', "a/");
        m.insert('ά', "a/");
        m.insert('ά', "a/");
        m.insert('α', "a");
        // m.insert('ᾲ', "a\\|");
        m.insert('ᾲ', "a/|");
        m.insert('ᾴ', "a/|");
        m.insert('ᾳ', "a|");
        m.insert('ἐ', "e)");
        m.insert('ἒ', "e)\\");
        m.insert('ἔ', "e)/");
        m.insert('ἑ', "e(");
        m.insert('ἓ', "e(\\");
        m.insert('ἕ', "e(/");
        // m.insert('ὲ', "e\\");
        m.insert('ὲ', "e/");
        m.insert('έ', "e/");
        m.insert('έ', "e/");
        m.insert('ε', "e");
        m.insert('ἠ', "h)");
        m.insert('ἢ', "h)\\");
        m.insert('ἤ', "h)/");
        m.insert('ᾒ', "h)\\|");
        m.insert('ᾔ', "h)/|");
        m.insert('ἦ', "h)=");
        m.insert('ᾖ', "h)=|");
        m.insert('ἡ', "h(");
        m.insert('ἣ', "h(\\");
        m.insert('ἥ', "h(/");
        m.insert('ᾓ', "h(\\|");
        m.insert('ᾕ', "h(/|");
        m.insert('ἧ', "h(=");
        m.insert('ᾗ', "h(=|");
        m.insert('ῆ', "h=");
        // m.insert('ὴ', "h\\");
        m.insert('ὴ', "h/");
        m.insert('ή', "h/");
        m.insert('ή', "h/");
        m.insert('η', "h");
        m.insert('ῇ', "h=|");
        // m.insert('ῂ', "h\\|");
        m.insert('ῂ', "h/|");
        m.insert('ῄ', "h/|");
        m.insert('ῃ', "h|");
        m.insert('ἰ', "i)");
        m.insert('ἲ', "i)\\");
        m.insert('ἴ', "i)/");
        m.insert('ἶ', "i)=");
        m.insert('ἱ', "i(");
        m.insert('ἳ', "i(\\");
        m.insert('ἵ', "i(/");
        m.insert('ἷ', "i(=");
        // m.insert('ὶ', "i\\");
        m.insert('ὶ', "i/");
        m.insert('ϊ', "i+");
        m.insert('ῖ', "i=");
        m.insert('ί', "i/");
        m.insert('ί', "i/");
        m.insert('ι', "i");
        m.insert('ὀ', "o)");
        m.insert('ὂ', "o)\\");
        m.insert('ὄ', "o)/");
        m.insert('ὁ', "o(");
        m.insert('ὃ', "o(\\");
        m.insert('ὅ', "o(/");
        // m.insert('ὸ', "o\\");
        m.insert('ὸ', "o/");
        m.insert('ό', "o/");
        m.insert('ό', "o/");
        m.insert('ό', "o/");
        m.insert('ο', "o");
        m.insert('ὐ', "u)");
        m.insert('ὖ', "u)=");
        m.insert('ὒ', "u)\\");
        m.insert('ὔ', "u)/");
        m.insert('ὑ', "u(");
        m.insert('ὓ', "u(\\");
        m.insert('ὕ', "u(/");
        m.insert('ὗ', "u(=");
        m.insert('ῦ', "u=");
        // m.insert('ὺ', "u\\");
        m.insert('ὺ', "u/");
        m.insert('ύ', "u/");
        m.insert('ύ', "u/");
        m.insert('ϋ', "u+");
        m.insert('υ', "u");
        m.insert('ὠ', "w)");
        m.insert('ὢ', "w)\\");
        m.insert('ὤ', "w)/");
        m.insert('ᾢ', "w)\\|");
        m.insert('ᾤ', "w)/|");
        m.insert('ὦ', "w)=");
        m.insert('ᾦ', "w)=|");
        m.insert('ὡ', "w(");
        m.insert('ὣ', "w(\\");
        m.insert('ὥ', "w(/");
        m.insert('ᾣ', "w(\\|");
        m.insert('ᾥ', "w(/|");
        m.insert('ὧ', "w(=");
        m.insert('ᾧ', "w(=|");
        m.insert('ῶ', "w=");
        // m.insert('ὼ', "w\\");
        m.insert('ὼ', "w/");
        m.insert('ώ', "w/");
        m.insert('ώ', "w/");
        m.insert('ω', "w");
        m.insert('ῷ', "w=|");
        // m.insert('ῲ', "w\\|");
        m.insert('ῲ', "w/|");
        m.insert('ῴ', "w/|");
        m.insert('ῳ', "w|");
        m.insert('β', "b");
        m.insert('γ', "g");
        m.insert('δ', "d");
        m.insert('ζ', "z");
        m.insert('θ', "q");
        m.insert('κ', "k");
        m.insert('λ', "l");
        m.insert('μ', "m");
        m.insert('ν', "n");
        m.insert('ξ', "c");
        m.insert('π', "p");
        m.insert('ρ', "r");
        m.insert('ῥ', "r(");
        m.insert('σ', "s");
        m.insert('ς', "s");
        m.insert('τ', "t");
        m.insert('φ', "f");
        m.insert('χ', "x");
        m.insert('ψ', "y");
        m.insert('Β', "*b");
        m.insert('Γ', "*g");
        m.insert('Δ', "*d");
        m.insert('Ζ', "*z");
        m.insert('Θ', "*q");
        m.insert('Κ', "*k");
        m.insert('Λ', "*l");
        m.insert('Μ', "*m");
        m.insert('Ν', "*n");
        m.insert('Ξ', "*c");
        m.insert('Π', "*p");
        m.insert('Ρ', "*r");
        m.insert('Ῥ', "*r(");
        m.insert('Σ', "*s");
        m.insert('Τ', "*t");
        m.insert('Φ', "*f");
        m.insert('Χ', "*x");
        m.insert('Ψ', "*y");
        m
    };
}


// lazy_static! {
//     static ref FINALS: HashMap<char, &'static str> = {
//         let mut m = HashMap::new();
//         m.insert('ὰ', "a/");
//         m.insert('ᾲ', "a/|");
//         m.insert('ὲ', "e/");
//         m.insert('ὴ', "h/");
//         m.insert('ῂ', "h/|");
//         m.insert('ὶ', "i/");
//         m.insert('ὸ', "o/");
//         m.insert('ὺ', "u/");
//         m.insert('ὼ', "w/");
//         m.insert('ῲ', "w/|");
//         m
//     };
// }

pub fn transliterate(s: &str) -> String {
    let mut trans_wd: String = String::new();
    for gk_char in s.chars() {
        let gk_str = match CHARS.get(&gk_char) {
            Some(s) => s,
            None => {
                eprintln!("{} -> problem char: {}", s, gk_char);
                "?"
            },
        };
        trans_wd.push_str(gk_str);
    }
    trans_wd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transliterate() {
        let tests = [
            "μῆνιν",
            "ἄειδε",
            "θεά",
            "Πηληϊάδεω",
            "γίγνονται",
            "τελευτήν",
        ];

        let mut output: Vec<String> = Vec::new();

        for wd in tests.iter() {
            let trans_wd = transliterate(wd);
            output.push(trans_wd);
        }

        assert_eq!(
            output,
            vec![
                "mh=nin",
                "a)/eide",
                "qea/",
                "*phlhi+a/dew",
                "gi/gnontai",
                "teleuth/n"
            ]
        )
    }
}
