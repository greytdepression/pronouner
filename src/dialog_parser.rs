use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};

use crate::{
    character::{CharacterCast, Title},
    verbs::Dictionary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogMacroType {
    VerbConjugate,
    Name,
    TitlePlusName,
    SubjectivePronoun,
    ObjectivePronoun,
    PossessiveDeterminer,
    PossessivePronoun,
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
    pub fn new(cast: CharacterCast, dict: Dictionary<'a>) -> Self {
        Self {
            cast,
            dictionary: dict,
        }
    }

    pub fn parse_and_compile(&self, src: &str) -> Result<String, crate::Error> {
        let mut output = String::new();

        let mut src_slice = src;

        if src_slice.is_empty() {
            return Ok("".to_string());
        }

        while !src_slice.is_empty() {
            // We expect there to be a next slice, otherwise src_slice should be empty!
            let mut split = src_slice.split('{');
            let mut next_slice = split.next().unwrap();

            // Check that any '}' is properly escaped
            if let Some(index) = next_slice.find('}') {
                // Brace was properly escaped
                if !matches!(next_slice.get(index..index + 2), Some("}}")) {
                    return Err(crate::Error::UnmatchedClosingBrace);
                }

                output.push_str(&next_slice[..index + 1]);
                src_slice = &src_slice[next_slice[..index + 2].as_bytes().len()..];
                next_slice = &next_slice[index + 2..];
            }

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

                let value = self.compile(macro_)?;

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
                }
                '}' if !in_string => {
                    counter -= 1;
                }
                '\\' if in_string => {
                    escape = !escape;
                }
                '"' if !in_string => {
                    in_string = true;
                }
                '"' if in_string && !escape => {
                    in_string = false;
                }
                _ => {
                    escape = false;
                }
            }

            bytes += c.len_utf8();

            if counter == 0 {
                break;
            }
        }

        &src[0..bytes]
    }

    pub fn compile(&self, macr: DialogMacro<'a>) -> Result<String, crate::Error> {
        let Some(person) = self.cast.get(macr.character_id) else {
            return Err(crate::Error::UnknownCharacterIdentifier);
        };

        // TODO: there's probably a bit too much logic in this function that should be put somewhere else
        let raw_string = match macr._type {
            DialogMacroType::VerbConjugate => {
                let Some(data) = macr.data else {
                    // TODO: don't return strings like that but make use of Result types
                    return Err(crate::Error::MissingMacroData);
                };

                self.dictionary.conjugate(data, person.conjugate_case())?
            }
            DialogMacroType::Name => person.name().to_string(),
            DialogMacroType::TitlePlusName => match person.title() {
                Some(title) if !matches!(title, &Title::NoTitle) => {
                    format!("{} {}", title.str(), person.name())
                }
                _ => person.name().to_string(),
            },
            DialogMacroType::SubjectivePronoun => person.subjective_pronoun(),
            DialogMacroType::ObjectivePronoun => person.objective_pronoun(),
            DialogMacroType::PossessiveDeterminer => person.possessive_determiner(),
            DialogMacroType::PossessivePronoun => person.possessive_pronoun(),
            DialogMacroType::PersonDescriptor => {
                if let Some(descriptor) = person.person_descriptor() {
                    descriptor.to_string()
                } else {
                    "person".to_string()
                }
            }
        };

        Ok(apply_mods(raw_string, &macr.mods))
    }
}

// TODO: this could probably be moved to its own file
fn apply_mods(mut input: String, mods: &[DialogMacroMod]) -> String {
    for _mod in mods {
        match _mod {
            DialogMacroMod::Capitalized => {
                let mut first = true;

                input = input
                    .chars()
                    .flat_map(|x| {
                        // Unicode to_uppercase may turn one character into multiple ones. For this reason
                        // we need to provide a vector for the ToUppercase iterator to write into.
                        // Think e.g. ß -> SS.
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
            }
            DialogMacroMod::UpperCase => {
                input = input.to_uppercase();
            }
            DialogMacroMod::LowerCase => {
                input = input.to_lowercase();
            }
        }
    }

    input
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{character, verbs};

    use super::*;

    type Res = Result<(), crate::Error>;

    fn gen_compiler() -> DialogMacroCompiler<'static> {
        DialogMacroCompiler {
            cast: character::tests::gen_cast(),
            dictionary: verbs::tests::gen_dict(),
        }
    }

    #[test]
    fn test_mods() -> Res {
        // DialogMacroMod::Capitalized
        assert_eq!(
            apply_mods("fooBar".to_string(), &[DialogMacroMod::Capitalized]),
            "FooBar"
        );

        // DialogMacroMod::UpperCase
        assert_eq!(
            apply_mods("fooBar".to_string(), &[DialogMacroMod::UpperCase]),
            "FOOBAR"
        );

        // DialogMacroMod::LowerCase
        assert_eq!(
            apply_mods("fooBar".to_string(), &[DialogMacroMod::LowerCase]),
            "foobar"
        );

        // Chaining
        assert_eq!(
            apply_mods(
                "fooBar".to_string(),
                &[DialogMacroMod::LowerCase, DialogMacroMod::Capitalized]
            ),
            "Foobar"
        );

        Ok(())
    }

    #[test]
    fn print_macro() -> Res {
        let dm = DialogMacro {
            character_id: "pidge",
            _type: DialogMacroType::SubjectivePronoun,
            data: None,
            mods: vec![],
        };

        assert_eq!(
            serde_json::to_string(&dm)?,
            r#"{"character_id":"pidge","_type":"SubjectivePronoun","data":null,"mods":[]}"#
        );

        let verb_dm = DialogMacro {
            character_id: "pidge",
            _type: DialogMacroType::VerbConjugate,
            data: Some("to be"),
            mods: vec![],
        };

        assert_eq!(
            serde_json::to_string(&verb_dm)?,
            r#"{"character_id":"pidge","_type":"VerbConjugate","data":"to be","mods":[]}"#
        );

        Ok(())
    }

    #[test]
    fn compile_test() -> Res {
        let pidge_possessive = DialogMacro {
            character_id: "pidge",
            _type: DialogMacroType::PossessivePronoun,
            data: None,
            mods: vec![],
        };

        let tupo_objective = DialogMacro {
            character_id: "tupo",
            _type: DialogMacroType::ObjectivePronoun,
            data: None,
            mods: vec![DialogMacroMod::Capitalized],
        };

        let compiler = gen_compiler();

        assert_eq!(compiler.compile(pidge_possessive)?, "theirs");

        assert_eq!(compiler.compile(tupo_objective)?, "Xem");

        Ok(())
    }

    #[test]
    fn full_compiler_test() -> Res {
        let source = r#"Do you know {"character_id":"pidge","_type":"Name","data":null,"mods":[]}? {"character_id":"pidge","_type":"SubjectivePronoun","data":null,"mods":["Capitalized"]} {"character_id":"pidge","_type":"VerbConjugate","data":"to be","mods":[]} super smart! I love {"character_id":"pidge","_type":"ObjectivePronoun","data":null,"mods":[]}! Have you seen {"character_id":"pidge","_type":"PossessiveDeterminer","data":null,"mods":[]} sentient robot?"#;
        let expected = "Do you know Pidge? They are super smart! I love them! Have you seen their sentient robot?";

        let compiler = gen_compiler();

        let output = compiler.parse_and_compile(source)?;

        assert_eq!(&output, expected,);

        Ok(())
    }

    #[test]
    fn error_tests() -> Res {
        let unknown_verb = r#"{"character_id":"pidge","_type":"VerbConjugate","data":"to be or not to be","mods":[]}"#;
        let unknown_character =
            r#"{"character_id":"edward elrich","_type":"VerbConjugate","data":"to be","mods":[]}"#;
        let null_character =
            r#"{"character_id":null,"_type":"VerbConjugate","data":"to be","mods":[]}"#;
        let unknown_type = r#"{"character_id":"pidge","_type":"Alchemy","data":null,"mods":[]}"#;
        let unknown_mod = r#"{"character_id":"pidge","_type":"VerbConjugate","data":"to be","mods":["Supercalifragilisticexpialidocious"]}"#;

        let compiler = gen_compiler();

        // Unknown Verb
        assert!(matches!(
            compiler.parse_and_compile(unknown_verb),
            Err(crate::Error::UnknownVerbKey),
        ));

        // Unknown Character
        assert!(matches!(
            compiler.parse_and_compile(unknown_character),
            Err(crate::Error::UnknownCharacterIdentifier),
        ));

        // Null Character
        assert!(matches!(
            compiler.parse_and_compile(null_character),
            Err(crate::Error::Serde(_)),
        ));

        // Unknown Type
        assert!(matches!(
            compiler.parse_and_compile(unknown_type),
            Err(crate::Error::Serde(_)),
        ));

        // Unknown Mod
        assert!(matches!(
            compiler.parse_and_compile(unknown_mod),
            Err(crate::Error::Serde(_)),
        ));

        Ok(())
    }

    #[test]
    fn parse_escapes() -> Res {
        let compiler = gen_compiler();

        let escaped_brace = "Do you ever just public static void main(String[] args) {{}}?";

        assert_eq!(
            compiler.parse_and_compile(escaped_brace)?,
            "Do you ever just public static void main(String[] args) {}?"
        );

        let mixed_escaped_and_macro = r#"{{{"character_id":"pidge","_type":"ObjectivePronoun","data":null,"mods":["UpperCase"]}}}"#;

        assert_eq!(
            compiler.parse_and_compile(mixed_escaped_and_macro)?,
            "{THEM}"
        );

        let missing_closing_escape = "Oh no! This closing } is not escaped D:";

        assert!(matches!(
            compiler.parse_and_compile(missing_closing_escape),
            Err(crate::Error::UnmatchedClosingBrace)
        ));

        Ok(())
    }
}
