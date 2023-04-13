use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::verbs::ConjugatePerson;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CharacterCast {
    map: HashMap<String, GrammaticalCharacter>,
}

impl CharacterCast {
    pub fn get(&self, key: &str) -> Option<&GrammaticalCharacter> {
        self.map.get(key)
    }

    pub fn insert(
        &mut self,
        key: String,
        value: GrammaticalCharacter,
    ) -> Option<GrammaticalCharacter> {
        self.map.insert(key, value)
    }

    pub fn remove(&mut self, key: &str) -> Option<GrammaticalCharacter> {
        self.map.remove(key)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pronouns {
    HeHim,
    SheHer,
    ItIts,
    #[default]
    TheyThem,
    Name,
    XeXyr,
    Custom {
        subjective: String,
        objective: String,
        possessive_determiner: String,
        possessive: String,
        reflexive: String,
        conjugate_case: ConjugatePerson,
    },
}

impl Pronouns {
    pub fn custom(
        subjective: String,
        objective: String,
        possessive: String,
        possessive_determiner: String,
        reflexive: String,
        conjugate_case: ConjugatePerson,
    ) -> Self {
        Self::Custom {
            subjective,
            objective,
            possessive_determiner,
            possessive,
            reflexive,
            conjugate_case,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Title {
    Mr,
    Ms,
    Mrs,
    #[default]
    Mx,
    NoTitle,
    Custom(String),
}

impl Title {
    pub fn custom(title: String) -> Self {
        Self::Custom(title)
    }

    pub fn str(&self) -> &str {
        match self {
            Title::Mr => "Mr.",
            Title::Ms => "Ms.",
            Title::Mrs => "Mrs.",
            Title::Mx => "Mx.",
            Title::NoTitle => "",
            Title::Custom(value) => value,
        }
    }
}

impl ToString for Title {
    fn to_string(&self) -> String {
        self.str().to_string()
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GrammaticalCharacter {
    name: String,
    pronouns: Pronouns,
    title: Option<Title>,
    person_descriptor: Option<String>,
}

impl GrammaticalCharacter {
    pub fn new(
        name: String,
        pronouns: Pronouns,
        title: Option<Title>,
        person_descriptor: Option<String>,
    ) -> Self {
        Self {
            name,
            pronouns,
            title,
            person_descriptor,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn title(&self) -> Option<&Title> {
        self.title.as_ref()
    }

    pub fn person_descriptor(&self) -> Option<&String> {
        self.person_descriptor.as_ref()
    }

    pub fn subjective_pronoun(&self) -> String {
        match &self.pronouns {
            Pronouns::HeHim => "he",
            Pronouns::SheHer => "she",
            Pronouns::ItIts => "it",
            Pronouns::TheyThem => "they",
            Pronouns::Name => &self.name,
            Pronouns::XeXyr => "xe",
            Pronouns::Custom { subjective, .. } => subjective,
        }
        .to_string()
    }

    pub fn objective_pronoun(&self) -> String {
        match &self.pronouns {
            Pronouns::HeHim => "him",
            Pronouns::SheHer => "her",
            Pronouns::ItIts => "it",
            Pronouns::TheyThem => "them",
            Pronouns::Name => &self.name,
            Pronouns::XeXyr => "xem",
            Pronouns::Custom { objective, .. } => objective,
        }
        .to_string()
    }

    pub fn possessive_determiner(&self) -> String {
        match &self.pronouns {
            Pronouns::HeHim => "his".to_string(),
            Pronouns::SheHer => "her".to_string(),
            Pronouns::ItIts => "its".to_string(),
            Pronouns::TheyThem => "their".to_string(),
            Pronouns::Name => {
                let name_ends_in_s = matches!(
                    self.name.chars().last().map(|c| c.to_ascii_lowercase()),
                    Some('s')
                );

                let name = &self.name;
                let end_char = if name_ends_in_s { "" } else { "s" };

                format!("{name}'{end_char}")
            }
            Pronouns::XeXyr => "xyr".to_string(),
            Pronouns::Custom {
                possessive_determiner,
                ..
            } => possessive_determiner.to_string(),
        }
    }

    pub fn possessive_pronoun(&self) -> String {
        match &self.pronouns {
            Pronouns::HeHim => "his".to_string(),
            Pronouns::SheHer => "hers".to_string(),
            Pronouns::ItIts => "its".to_string(),
            Pronouns::TheyThem => "theirs".to_string(),
            Pronouns::Name => {
                let name_ends_in_s = matches!(
                    self.name.chars().last().map(|c| c.to_ascii_lowercase()),
                    Some('s')
                );

                let name = &self.name;
                let end_char = if name_ends_in_s { "" } else { "s" };

                format!("{name}'{end_char}")
            }
            Pronouns::XeXyr => "xyrs".to_string(),
            Pronouns::Custom { possessive, .. } => possessive.to_string(),
        }
    }

    pub fn reflexive_pronoun(&self) -> String {
        match &self.pronouns {
            Pronouns::HeHim => "himself".to_string(),
            Pronouns::SheHer => "herself".to_string(),
            Pronouns::ItIts => "itself".to_string(),
            Pronouns::TheyThem => "themself".to_string(),
            Pronouns::Name => {
                let name_ends_in_s = matches!(
                    self.name.chars().last().map(|c| c.to_ascii_lowercase()),
                    Some('s')
                );

                let name = &self.name;
                let end_char = if name_ends_in_s { "" } else { "s" };

                // FIXME: find a good solution for this case
                format!("{name}'{end_char} self")
            }
            Pronouns::XeXyr => "xyrself".to_string(),
            Pronouns::Custom { reflexive, .. } => reflexive.to_string(),
        }
    }

    pub fn conjugate_case(&self) -> ConjugatePerson {
        use ConjugatePerson::*;
        use Pronouns::*;
        match &self.pronouns {
            HeHim | SheHer | ItIts | XeXyr | Name => ThirdSingular,
            TheyThem => ThirdPlural,
            Pronouns::Custom { conjugate_case, .. } => *conjugate_case,
        }
    }
}

impl Default for GrammaticalCharacter {
    fn default() -> Self {
        Self::new(
            "##MISSING CHARACTER#".to_string(),
            Pronouns::default(),
            None,
            None,
        )
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    type Res = Result<(), crate::Error>;

    pub(crate) fn get_characters() -> [GrammaticalCharacter; 4] {
        let pidge = GrammaticalCharacter {
            name: "Pidge".into(),
            pronouns: Pronouns::TheyThem,
            title: Some(Title::NoTitle),
            person_descriptor: Some("Person".to_string()),
        };

        let alfons = GrammaticalCharacter {
            name: "Alfons".into(),
            pronouns: Pronouns::Name,
            title: Some(Title::Custom("King".into())),
            person_descriptor: Some("Man".to_string()),
        };

        let tupo = GrammaticalCharacter {
            name: "Tupo".to_string(),
            pronouns: Pronouns::XeXyr,
            title: Some(Title::NoTitle),
            person_descriptor: Some("Laru".to_string()),
        };

        let hunk = GrammaticalCharacter {
            name: "Hunk".to_string(),
            pronouns: Pronouns::HeHim,
            title: Some(Title::Mr),
            person_descriptor: Some("Man".to_string()),
        };

        [pidge, alfons, tupo, hunk]
    }

    pub(crate) fn gen_cast() -> CharacterCast {
        let mut cast = CharacterCast::default();

        let [pidge, alfons, tupo, hunk] = crate::character::tests::get_characters();

        cast.insert("pidge".to_string(), pidge);
        cast.insert("alfons".to_string(), alfons);
        cast.insert("tupo".to_string(), tupo);
        cast.insert("hunk".to_string(), hunk);

        cast
    }

    #[test]
    fn no_pronoun_test() {
        let [pidge, alfons, tupo, _] = get_characters();

        assert_eq!(
            format!(
                "{} is super smart! I love {}! Have you seen {} sentient robot?",
                &pidge.name,
                pidge.objective_pronoun(),
                pidge.possessive_determiner(),
            ),
            "Pidge is super smart! I love them! Have you seen their sentient robot?"
        );
        assert_eq!(
            format!("{} lion has been lost.", alfons.possessive_determiner(),),
            "Alfons' lion has been lost.",
        );
        assert_eq!(
            format!(
                "{} is still going through puberty. Give {} some time to grow into {} legs.",
                &tupo.name,
                tupo.objective_pronoun(),
                tupo.possessive_determiner(),
            ),
            "Tupo is still going through puberty. Give xem some time to grow into xyr legs."
        );
    }
}
