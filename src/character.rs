use std::{collections::HashMap, };

use serde::{Serialize, Deserialize};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CharacterCast {
    map: HashMap<String, GrammaticalCharacter>,
}

impl CharacterCast {
    pub fn get(&self, key: &str) -> Option<&GrammaticalCharacter> {
        self.map.get(key)
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
        possessive: String,
    },
}

impl Pronouns {
    pub fn custom(
        subjective: String,
        objective: String,
        possessive: String,
    ) -> Self {
        Self::Custom { subjective, objective, possessive }
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

    pub fn subjective_pronoun(&self) -> String {
        match &self.pronouns {
            Pronouns::HeHim => "he",
            Pronouns::SheHer => "she",
            Pronouns::ItIts => "it",
            Pronouns::TheyThem => "they",
            Pronouns::Name => &self.name,
            Pronouns::XeXyr => "xe",
            Pronouns::Custom { subjective, .. } => &subjective,
        }.to_string()
    }

    pub fn objective_pronoun(&self) -> String {
        match &self.pronouns {
            Pronouns::HeHim => "him",
            Pronouns::SheHer => "her",
            Pronouns::ItIts => "it",
            Pronouns::TheyThem => "them",
            Pronouns::Name => &self.name,
            Pronouns::XeXyr => "xem",
            Pronouns::Custom { objective, .. } => &objective,
        }.to_string()
    }

    pub fn possessive_pronoun(&self) -> String {
        match &self.pronouns {
            Pronouns::HeHim => "his".to_string(),
            Pronouns::SheHer => "her".to_string(),
            Pronouns::ItIts => "its".to_string(),
            Pronouns::TheyThem => "their".to_string(),
            Pronouns::Name => {
                let ends_in_s = matches!(
                    self.name
                        .chars()
                        .last()
                        .map(|c| c.to_ascii_lowercase()),
                    Some('s')
                );

                let name = &self.name;
                let end_char = if ends_in_s { "" } else { "s" };

                format!("{name}'{end_char}")
            },
            Pronouns::XeXyr => "xyr".to_string(),
            Pronouns::Custom { possessive, .. } => possessive.to_string(),
        }
    }
}

impl Default for GrammaticalCharacter {
    fn default() -> Self {
       Self::new("##MISSING CHARACTER#".to_string(), Pronouns::default(), None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type SerdeResult = Result<(), serde_json::Error>;

    fn get_characters() -> [GrammaticalCharacter; 3] {
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

        [pidge, alfons, tupo]
    }

    #[test]
    fn serde_test() -> SerdeResult {
        let [pidge, alfons, tupo] = get_characters();

        println!("{}", serde_json::to_string(&pidge)?);
        println!("{}", serde_json::to_string(&alfons)?);
        println!("{}", serde_json::to_string(&tupo)?);

        Ok(())
    }

    #[test]
    fn no_pronoun_test() {
        let [pidge, alfons, tupo] = get_characters();

        assert_eq!(
            format!(
                "{} is super smart! I love {}! Have you seen {} sentient robot?",
                &pidge.name,
                pidge.objective_pronoun(),
                pidge.possessive_pronoun(),
            ),
            "Pidge is super smart! I love them! Have you seen their sentient robot?"
        );
        assert_eq!(
            format!(
                "{} lion has been lost.",
                alfons.possessive_pronoun(),
            ),
            "Alfons' lion has been lost.",
        );
        assert_eq!(
            format!(
                "{} is still going through puberty. Give {} some time to grow into {} legs.",
                &tupo.name,
                tupo.objective_pronoun(),
                tupo.possessive_pronoun(),
            ),
            "Tupo is still going through puberty. Give xem some time to grow into xyr legs."
        );
    }
}
