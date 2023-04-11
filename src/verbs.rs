use std::{fmt::Display, collections::HashMap};

use serde::{Serialize, Deserialize};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Dictionary<'a> {
    #[serde(borrow)]
    map: HashMap<String, Verb<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn conjugate(&self, key: &str, person: ConjugatePerson) -> String {
        let fallback = || {
            format!("##MISSING '{key}'.'{person:?}'##")
        };

        let Some(verb) = self.map.get(key) else {
            return fallback();
        };

        let Some(conj_verb) = verb.get(person) else {
            return fallback();
        };

        conj_verb.to_string()
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
mod tests {
    use super::*;


    type SerdeResult = Result<(), serde_json::Error>;

    #[test]
    fn verb_test() -> SerdeResult {
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