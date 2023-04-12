# pronouner

A small crate to help write pronoun agnostic dialog for video games.

Issues, comments, and pull requests are very much appreciated! If you have an idea how pronouner could be made better feel free to tell me or open a PR :)

## :warning: Warning

***pronouner is very much a work-in-progress. Expect breaking changes to the API! I'm not happy with the current naming of things and will likely change most struct/enum names some time in the future.***

## :computer: Usage

You build a `DialogMacroCompiler` using a cast of characters and a dictionary of verb conjugations. The compiler then replaces DialogMacros within the XYR dialog with the corresponding words.

See example folder for simple usage examples. You can also try the example with `cargo run --example simple`.

DialogMacros have the form of a JSON dictionary and consist of the following key-value pairs

```javascript
{
// The identifier of the character in the `CharacterCast`
"character_id": "id",

// A list of types and explanations is given below
"type_": "type",

// String or `null`. Identifies the verb if type is `VerbConjugate`
"data": "data",

// A list of modifiers for the generated string
"mod": [],
}
```

**`type_`** The type of the macro. One of the following:
Identifier | Description
--- | ---
`VerbConjugate` | Conjugate the verb
`Name` | The name of the character
`TitlePlusName` | The character's title and their name (Mx. Y)
`PronounSubjective` | The character's subjective pronoun (they)
`PronounObjective` | The character's objective pronoun (them)
`PronounPossessive` | The character's possessive pronoun (their)
`PersonDescriptor` | The character's descriptor as a person (man/woman/person)

**`mod`** A list of modifications for the output string. The options are:
Identifier | Description
--- | ---
`Capitalized` | Make the first letter uppercase and leave the rest
`LowerCase` | Make every character lowercase
`UpperCase` | Make every character uppercase

## :checkered_flag: Goals

- [x] XYR dialog syntax
- [x] XYR parser + compiler
- [x] Character and dictionary context
- [x] Serializing and deserializing of context
- [ ] Multiple pronouns?
- [ ] Command line tool to help write XYR
- [ ] Potentially: VSCode plugin to help write XYR (third party)

## :heavy_exclamation_mark: Known issues/limitations
