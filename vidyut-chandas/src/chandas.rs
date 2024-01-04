use crate::akshara::{scan_lines, Akshara};
use crate::vrtta::{MatchType, Vrtta};
use crate::Jati;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Describes a result of classifying an input string with `Chandas`.
#[derive(Debug)]
pub struct Matches {
    matches: Vec<Match>,
    aksharas: Vec<Vec<Akshara>>,
}

impl Matches {
    /// Get the Aksharas of this match result
    pub fn aksharas(&self) -> &Vec<Vec<Akshara>> {
        &self.aksharas
    }

    /// Get the matches of this match result
    pub fn matches(&self) -> &Vec<Match> {
        &self.matches
    }
}

#[derive(Debug)]
pub struct Match {
    padya: Padya,
    match_type: MatchType,
    partial_match: usize,
}

/// Padya Struct
#[derive(Debug)]
pub enum Padya {
    /// Vrtta Struct
    Vrtta(Vrtta),
    /// Jati Struct
    Jati(Jati),
}

impl Match {
    /// The vrtta match for this query.
    pub fn padya(&self) -> &Padya {
        &self.padya
    }

    /// The match type for this query.
    pub fn match_type(&self) -> &MatchType {
        &self.match_type
    }
}

/// A metrical classifier.
///
///
/// ### Usage
///
/// ```
/// use vidyut_chandas::{Chandas, MatchType, Vrtta, Padya};
///
/// let vrttas: Vec<Vrtta> = vec![
///     "vasantatilakA\tvrtta\tGGLGLLLGLLGLGG".try_into().unwrap(),
///     "mandAkrAntA\tvrtta\tGGGGLLLLLGGLGGLGG".try_into().unwrap(),
///     "puzpitAgrA\tvrtta\tLLLLLLGLGLGG/LLLLGLLGLGLGG".try_into().unwrap(),
///     "udgatA\tvrtta\tLLGLGLLLGL/LLLLLGLGLG/GLLLLLLGLLG/LLGLGLLLGLGLG".try_into().unwrap()
/// ];
/// let chandas = Chandas::new(vrttas);
///
/// let classify_result = chandas.classify("mAtaH samastajagatAM maDukEwaBAreH");
/// let result = &classify_result.matches()[0];
/// let name: &str = match result.padya() {
///     Padya::Vrtta(v) => v.name(),
///     Padya::Jati(j) => j.name(),
/// };
/// assert_eq!(name, "vasantatilakA");
/// assert_eq!(result.match_type(), &MatchType::Pada);
/// ```

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Chandas {
    vrttas: Vec<Vrtta>,
}

impl Chandas {
    /// Creates a new `Chandas` instance.
    pub fn new(vrttas: Vec<Vrtta>) -> Chandas {
        Self { vrttas }
    }

    /// Creates a new `Chandas` instance by defining meters from the given text data.
    ///
    /// We recommend using this constructor when the program does not have access to the
    /// filesystem, e.g. when using this code in WebAssembly.
    pub fn from_text(data: &str) -> Result<Self, Box<dyn Error>> {
        let vrttas: Result<Vec<_>, _> = data.lines().map(Vrtta::try_from).collect();
        Ok(Self::new(vrttas?))
    }

    /// Creates a new classifier from the given data path.
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let path = PathBuf::from(path).join(path);
        let data = fs::read_to_string(path)?;
        let vrttas: Result<Vec<_>, _> = data.lines().map(Vrtta::try_from).collect();

        Ok(Self::new(vrttas?))
    }

    /// The vrttas available to this classifier.
    pub fn vrttas(&self) -> &Vec<Vrtta> {
        &self.vrttas
    }

    /// Classifies the input string against an internal list of meters.
    ///
    /// Currently, this function supports only simple samavrtta.
    pub fn classify(&self, text: impl AsRef<str>) -> Matches {
        self.classify_all(text.as_ref(), 1)
    }
    /// Classifies the input string against an internal list of meters and returns the best 'n' partial matches
    ///
    /// Currently, this function supports only simple samavrtta.
    pub fn classify_all(&self, text: impl AsRef<str>, n: usize) -> Matches {
        let aksharas = scan_lines(text.as_ref().lines());
        let mut matches: Matches = Matches {
            matches: vec![],
            aksharas: aksharas.clone(),
        };

        for (_i, vrtta) in self.vrttas.iter().enumerate() {
            let (partial_match, match_type) = vrtta.try_match(&aksharas);
            matches.matches.push(Match {
                padya: Padya::Vrtta(vrtta.clone()),
                match_type: match_type,
                partial_match: partial_match,
            });
        }
        matches.matches.sort_by(|a, b| {
            let match_type_order = |match_type: MatchType| -> u8 {
                match match_type {
                    MatchType::Full => 0,
                    MatchType::Pada => 1,
                    MatchType::Prefix => 2,
                    MatchType::None => 3,
                }
            };

            let a_type = match_type_order(a.match_type);
            let b_type = match_type_order(b.match_type);

            a_type
                .cmp(&b_type)
                .then_with(|| a.partial_match.cmp(&b.partial_match))
        });

        eprintln!("{:?}", matches.matches);
        matches.matches.truncate(n);
        matches.matches.shrink_to_fit();
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_has_vrtta(c: &Chandas, text: &str, expected: &str) {
        let res = &c.classify(text).matches[0].padya;
        let name: &str = match res {
            Padya::Vrtta(v) => v.name(),
            Padya::Jati(j) => j.name(),
        };
        assert_eq!(name, expected);
    }

    fn new_chandas() -> Chandas {
        Chandas::new(vec![
            "vasantatilakA\tvrtta\tGGLGLLLGLLGLGG".try_into().unwrap(),
            "mandAkrAntA\tvrtta\tGGGGLLLLLGGLGGLGG".try_into().unwrap(),
            "puzpitAgrA\tvrtta\tLLLLLLGLGLGG/LLLLGLLGLGLGG"
                .try_into()
                .unwrap(),
            "udgatA\tvrtta\tLLGLGLLLGL/LLLLLGLGLG/GLLLLLLGLLG/LLGLGLLLGLGLG"
                .try_into()
                .unwrap(),
        ])
    }

    #[test]
    fn classify_samavrtta_single_pada() {
        let c = new_chandas();
        assert_has_vrtta(&c, "mAtaH samastajagatAM maDukEwaBAreH", "vasantatilakA");
        assert_has_vrtta(&c, "mAtaH\nsamastajagatAM\nmaDukEwaBAreH", "vasantatilakA");
    }

    #[test]
    fn classify_samavrtta_full_verse() {
        let c = new_chandas();
        assert_has_vrtta(
            &c,
            "kaScitkAntAvirahaguruRA svADikArapramattaH
            zApenAstaMgamitamahimA varzaBogyeRa BartuH .
            yakzaScakre janakatanayAsnAnapuRyodakezu
            snigDacCAyAtaruzu vasatiM rAmagiryASramezu .. 1 ..",
            "mandAkrAntA",
        );
        assert!(c.classify("mo mo go go vidyunmAlA").matches[0].match_type == MatchType::None);
    }

    #[test]
    fn classify_ardhasamavrtta_pada_1() {
        let c = new_chandas();
        assert_has_vrtta(&c, "aTa madanavaDUrupaplavAntaM", "puzpitAgrA");
    }

    #[test]
    fn classify_ardhasamavrtta_half() {
        let c = new_chandas();
        assert_has_vrtta(
            &c,
            "aTa madanavaDUrupaplavAntaM vyasanakfSA paripAlayAmbaBUva",
            "puzpitAgrA",
        );
        assert_has_vrtta(
            &c,
            "aTa\nmadanavaDUrupaplavAntaM\nvyasanakfSA\nparipAlayAmbaBUva",
            "puzpitAgrA",
        );
    }

    #[test]
    fn classify_ardhasamavrtta_full_verse() {
        let c = new_chandas();
        assert_has_vrtta(
            &c,
            "aTa madanavaDUrupaplavAntaM vyasanakfSA paripAlayAmbaBUva |
                SaSina iva divAtanasya leKA kiraRaparikzayaDUsarA pradozam ||",
            "puzpitAgrA",
        );
    }

    #[test]
    fn classify_vishamavrtta_pada_1() {
        let c = new_chandas();
        assert_has_vrtta(&c, "aTa vAsavasya vacanena", "udgatA");
    }

    #[test]
    fn classify_vishamavrtta_full_verse() {
        let c = new_chandas();
        assert_has_vrtta(
            &c,
            "aTa vAsavasya vacanena ruciravadanastrilocanam |
                klAntirahitamaBirADayituM viDivattapAMsi vidaDe DanaMjayaH ||",
            "udgatA",
        );
    }
}
