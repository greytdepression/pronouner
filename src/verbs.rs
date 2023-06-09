use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Dictionary<'a> {
    #[serde(borrow)]
    map: HashMap<String, Verb<'a>>,
}

// TODO: find a better name for this
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ConjugatePerson {
    FirstSingular,
    SecondSingular,
    ThirdSingular,
    FirstPlural,
    SecondPlural,
    ThirdPlural,
}

impl<'a> Dictionary<'a> {
    pub fn insert(&mut self, key: String, verb: Verb<'a>) -> Option<Verb> {
        self.map.insert(key, verb)
    }

    pub fn remove(&mut self, key: &str) -> Option<Verb> {
        self.map.remove(key)
    }

    pub fn conjugate(&self, key: &str, person: ConjugatePerson) -> Result<String, Error> {
        let Some(verb) = self.map.get(key) else {
            return Err(Error::UnknownVerbKey);
        };

        let Some(conj_verb) = verb.get(person) else {
            return Err(Error::UndefinedVerbCase);
        };

        Ok(conj_verb.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verb<'a> {
    debug_ident: &'a str,
    infinitive: Option<&'a str>,
    singular1: Option<&'a str>,
    singular2: Option<&'a str>,
    singular3: Option<&'a str>,
    plural1: Option<&'a str>,
    plural2: Option<&'a str>,
    plural3: Option<&'a str>,
}

impl<'a> Verb<'a> {
    pub fn get(&self, person: ConjugatePerson) -> Option<&'a str> {
        match person {
            ConjugatePerson::FirstSingular => self.singular1,
            ConjugatePerson::SecondSingular => self.singular2,
            ConjugatePerson::ThirdSingular => self.singular3,
            ConjugatePerson::FirstPlural => self.plural1,
            ConjugatePerson::SecondPlural => self.plural2,
            ConjugatePerson::ThirdPlural => self.plural3,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        debug_ident: &'a str,
        infinitive: Option<&'a str>,
        singular1: Option<&'a str>,
        singular2: Option<&'a str>,
        singular3: Option<&'a str>,
        plural1: Option<&'a str>,
        plural2: Option<&'a str>,
        plural3: Option<&'a str>,
    ) -> Self {
        Self {
            debug_ident,
            infinitive,
            singular1,
            singular2,
            singular3,
            plural1,
            plural2,
            plural3,
        }
    }
}

impl<'a> Display for Verb<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "\"{}\" -- to {}:\n  I {}.\n  You {}.\n  He/she/it {}.\n  We {}.\n  You {}.\n  They {}.\n",
                self.debug_ident,
                self.infinitive.unwrap_or("N/A"),
                self.singular1.unwrap_or("N/A"),
                self.singular2.unwrap_or("N/A"),
                self.singular3.unwrap_or("N/A"),
                self.plural1.unwrap_or("N/A"),
                self.plural2.unwrap_or("N/A"),
                self.plural3.unwrap_or("N/A"),
            )
        )
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    type Res = Result<(), crate::Error>;

    pub(crate) fn gen_dict() -> Dictionary<'static> {
        let to_be = Verb::new(
            "to be",
            Some("be"),
            Some("am"),
            Some("are"),
            Some("is"),
            Some("are"),
            Some("are"),
            Some("are"),
        );

        let to_have = Verb::new(
            "to have",
            Some("have"),
            Some("have"),
            Some("have"),
            Some("has"),
            None,
            None,
            None,
        );

        let mut dict = Dictionary::default();

        dict.insert("to be".to_string(), to_be);
        dict.insert("to have".to_string(), to_have);

        dict
    }

    #[test]
    fn verb_test() -> Res {
        let foo = Verb {
            debug_ident: "to be",
            infinitive: None,
            singular1: Some("am"),
            singular2: Some("are"),
            singular3: Some("is"),
            plural1: None,
            plural2: Some("are"),
            plural3: Some("are"),
        };

        println!("{}", serde_json::to_string(&foo)?);
        println!("{}", foo);

        Ok(())
    }
}
