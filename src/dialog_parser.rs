use serde::{Serialize, Deserialize};
use smallvec::{SmallVec, smallvec};

use crate::{character::{CharacterCast, GrammaticalCharacter}, verbs::Dictionary};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogMacroType {
    VerbConjugate,
    Name,
    FullName,
    TitlePlusName,
    PronounSubjective,
    PronounObjective,
    PronounPossessive,
    PersonDescriptor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogMacroMod {
    Capitalized,
    UpperCase,
    LowerCase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogMacro<'a> {
    character_id: &'a str,
    _type: DialogMacroType,
    data: Option<&'a str>,
    mods: Vec<DialogMacroMod>,
}

pub struct DialogMacroCompiler<'a> {
    cast: CharacterCast,
    dictionary: Dictionary<'a>,
}

impl<'a> DialogMacroCompiler<'a> {
    pub fn compile(&self, macr: DialogMacro<'a>) -> String {

        // TODO: do something more efficient
        let default_char = GrammaticalCharacter::default();

        let person = self.cast.get(macr.character_id).unwrap_or(&default_char);

        match macr._type {
            DialogMacroType::VerbConjugate => todo!(),//self.dictionary.conjugate(macr.data, person),
            DialogMacroType::Name => todo!(),
            DialogMacroType::FullName => todo!(),
            DialogMacroType::TitlePlusName => todo!(),
            DialogMacroType::PronounSubjective => todo!(),
            DialogMacroType::PronounObjective => todo!(),
            DialogMacroType::PronounPossessive => todo!(),
            DialogMacroType::PersonDescriptor => todo!(),
        }
    }
}

fn apply_mods(mut input: String, mods: &[DialogMacroMod]) -> String {
    for _mod in mods {
        match _mod {
            DialogMacroMod::Capitalized => {
                let mut first = true;

                input = input.chars()
                    .flat_map(|x| {

                        // Unicode to_uppercase may turn one character into multiple ones. For this reason
                        // we need to provide a vector for the ToUppercase iterator to write into.
                        // Use SmallVec to avoid heap allocations.
                        let y: SmallVec<[char; 2]> = if first {
                            x.to_uppercase().collect()
                        } else {
                            smallvec![x]
                        };

                        first = false;

                        y
                    })
                    .collect();
            },
            DialogMacroMod::UpperCase => {
                input = input.to_uppercase();
            },
            DialogMacroMod::LowerCase => {
                input = input.to_lowercase();
            },
        }
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    type SerdeResult = Result<(), serde_json::Error>;

    #[test]
    fn print_macro() -> SerdeResult {
        let dm = DialogMacro {
            character_id: "pidge",
            _type: DialogMacroType::PronounSubjective,
            data: None,
            mods: vec![],
        };

        println!("{}", serde_json::to_string(&dm)?);

        let verb_dm = DialogMacro {
            character_id: "pidge",
            _type: DialogMacroType::VerbConjugate,
            data: Some("to be"),
            mods: vec![],
        };

        println!("{}", serde_json::to_string(&verb_dm)?);

        Ok(())
    }

}
