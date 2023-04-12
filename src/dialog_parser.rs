use std::str::Chars;

use serde::{Serialize, Deserialize};
use smallvec::{SmallVec, smallvec};

use crate::{character::{CharacterCast, GrammaticalCharacter, Title}, verbs::Dictionary};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogMacroType {
    VerbConjugate,
    Name,
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

    pub fn parse_and_compile(&self, src: &str) -> Result<String, serde_json::Error> {
        let mut output = String::new();

        let mut src_slice = &src[..];

        if src_slice.is_empty() {
            return Ok("".to_string());
        }


        while !src_slice.is_empty() {

            // We expect there to be a next slice, otherwise src_slice should be empty!
            let mut split = src_slice.split('{');
            let next_slice = split.next().unwrap();

            // Push the slice onto the output string and shrink src_slice
            output.push_str(next_slice);
            src_slice = &src_slice[next_slice.as_bytes().len()..];

            let Some(next) = split.next() else {
                break;
            };

            // If the next slice is empty, this means the string was "{{"
            if next.is_empty() {
                output.push('{');
                src_slice = &src_slice["{{".as_bytes().len()..];
            } else {
                let macro_str = self.delimit_macro(src_slice);

                src_slice = &src_slice[macro_str.as_bytes().len()..];

                let macro_: DialogMacro = serde_json::from_str(macro_str)?;

                let value = self.compile(macro_);

                output.push_str(&value);
            }
        }

        Ok(output)
    }

    fn delimit_macro<'b>(&self, src: &'b str) -> &'b str {
        assert!(src.starts_with('{'));

        let mut counter = 0;

        let mut bytes = 0;

        let mut in_string = false;
        let mut escape = false;

        for c in src.chars() {
            match c {
                '{' if !in_string => {
                    counter += 1;
                },
                '}' if !in_string => {
                    counter -= 1;
                },
                '\\' if in_string => {
                    escape = !escape;
                },
                '"' if !in_string => {
                    in_string = true;
                },
                '"' if in_string && !escape => {
                    in_string = false;
                }
                _ => {
                    escape = false;
                },
            }

            bytes += c.len_utf8();

            if counter == 0 {
                break;
            }
        }

        &src[0..bytes]
    }

    pub fn compile(&self, macr: DialogMacro<'a>) -> String {

        // TODO: do something more efficient
        let default_char = GrammaticalCharacter::default();

        let person = self.cast.get(macr.character_id).unwrap_or(&default_char);

        // TODO: there's probably a bit too much logic in this function that should be put somewhere else
        match macr._type {
            DialogMacroType::VerbConjugate => {
                let Some(data) = macr.data else {
                    // TODO: don't return strings like that but make use of Result types
                    return "##MISSING VerbConjugate data##".to_string();
                };

                self.dictionary.conjugate(data, person.conjugate_case())
            },
            DialogMacroType::Name => person.name().to_string(),
            DialogMacroType::TitlePlusName => {
                match person.title() {
                    Some(title) if !matches!(title, &Title::NoTitle) => {
                        format!("{} {}", title.str(), person.name())
                    }
                    _ => person.name().to_string(),
                }
            },
            DialogMacroType::PronounSubjective => person.subjective_pronoun(),
            DialogMacroType::PronounObjective => person.objective_pronoun(),
            DialogMacroType::PronounPossessive => person.possessive_pronoun(),
            DialogMacroType::PersonDescriptor => {
                if let Some(descriptor) = person.person_descriptor() {
                    descriptor.to_string()
                } else {
                    "person".to_string()
                }
            },
        }
    }
}

// TODO: this could probably be moved to its own file
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

    #[test]
    fn full_compiler_test() -> SerdeResult {
        // TODO: write a full test

        Ok(())
    }

}
