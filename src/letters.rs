use std::collections::HashMap;

type Letter = [&'static str; 8];

#[rustfmt::skip]
const LOWERCASE_A: Letter = [
    "        ",
    "@@@@@@@@",
    "      @@",
    "@@@@@@@@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_B: Letter = [
    "@@      ",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_C: Letter = [
    "        ",
    "@@@@@@@@",
    "@@      ",
    "@@      ",
    "@@      ",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_D: Letter = [
    "      @@",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_E: Letter = [
    "        ",
    "@@@@@@@@",
    "@@    @@",
    "@@@@@@@@",
    "@@      ",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_F: Letter = [
    "  @@",
    "@@  ",
    "@@@@",
    "@@  ",
    "@@  ",
    "@@  ",
    "    ",
    "    ",
];

#[rustfmt::skip]
const LOWERCASE_G: Letter = [
    "        ",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "      @@",
    "@@@@@@@@",
];

#[rustfmt::skip]
const LOWERCASE_H: Letter = [
    "@@      ",
    "@@@@@@  ",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_I: Letter = [
    "@@",
    "  ",
    "@@",
    "@@",
    "@@",
    "@@",
    "  ",
    "  ",
];

#[rustfmt::skip]
const LOWERCASE_J: Letter = [
    "  @@",
    "    ",
    "  @@",
    "  @@",
    "  @@",
    "  @@",
    "  @@",
    "@@  ",
];

#[rustfmt::skip]
const LOWERCASE_K: Letter = [
    "@@      ",
    "@@    @@",
    "@@@@@@  ",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_L: Letter = [
    "@@",
    "@@",
    "@@",
    "@@",
    "@@",
    "@@",
    "  ",
    "  ",
];

#[rustfmt::skip]
const LOWERCASE_M: Letter = [
    "          ",
    "@@@@@@@@  ",
    "@@  @@  @@",
    "@@  @@  @@",
    "@@  @@  @@",
    "@@  @@  @@",
    "          ",
    "          ",
];

#[rustfmt::skip]
const LOWERCASE_N: Letter = [
    "        ",
    "@@@@@@  ",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_O: Letter = [
    "        ",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_P: Letter = [
    "        ",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "@@      ",
    "@@      ",
];

#[rustfmt::skip]
const LOWERCASE_Q: Letter = [
    "        ",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "      @@",
    "      @@",
];

#[rustfmt::skip]
const LOWERCASE_R: Letter = [
    "      ",
    "@@@@@@",
    "@@    ",
    "@@    ",
    "@@    ",
    "@@    ",
    "      ",
    "      ",
];

#[rustfmt::skip]
const LOWERCASE_S: Letter = [
    "        ",
    "@@@@@@@@",
    "@@      ",
    "@@@@@@@@",
    "      @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_T: Letter = [
    "  @@  ",
    "@@@@@@",
    "  @@  ",
    "  @@  ",
    "  @@  ",
    "  @@  ",
    "      ",
    "      ",
];

#[rustfmt::skip]
const LOWERCASE_U: Letter = [
    "        ",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_V: Letter = [
    "        ",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    " @@  @@ ",
    "   @@   ",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_W: Letter = [
    "          ",
    "@@  @@  @@",
    "@@  @@  @@",
    "@@  @@  @@",
    "@@  @@  @@",
    "@@@@@@@@@@",
    "          ",
    "          ",
];

#[rustfmt::skip]
const LOWERCASE_X: Letter = [
    "        ",
    "@@    @@",
    "@@    @@",
    "  @@@@  ",
    "@@    @@",
    "@@    @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const LOWERCASE_Y: Letter = [
    "        ",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "      @@",
    "@@@@@@@@",
];

#[rustfmt::skip]
const LOWERCASE_Z: Letter = [
    "        ",
    "@@@@@@@@",
    "      @@",
    "  @@@@  ",
    "@@      ",
    "@@@@@@@@",
    "        ",
    "        ",
];

// Uppercase.

#[rustfmt::skip]
const UPPERCASE_A: Letter = [
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_B: Letter = [
    "@@@@@@  ",
    "@@  @@  ",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_C: Letter = [
    "@@@@@@@@",
    "@@      ",
    "@@      ",
    "@@      ",
    "@@      ",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_D: Letter = [
    "@@@@@@  ",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_E: Letter = [
    "@@@@@@@@",
    "@@      ",
    "@@@@@@  ",
    "@@      ",
    "@@      ",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_F: Letter = [
    "@@@@@@@@",
    "@@      ",
    "@@@@@@  ",
    "@@      ",
    "@@      ",
    "@@      ",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_G: Letter = [
    "@@@@@@@@",
    "@@      ",
    "@@      ",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_H: Letter = [
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_I: Letter = [
    "@@@@",
    " @@ ",
    " @@ ",
    " @@ ",
    " @@ ",
    "@@@@",
    "    ",
    "    ",
];

#[rustfmt::skip]
const UPPERCASE_J: Letter = [
    "      @@",
    "      @@",
    "      @@",
    "      @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_K: Letter = [
    "@@    @@",
    "@@    @@",
    "@@@@@@  ",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_L: Letter = [
    "@@      ",
    "@@      ",
    "@@      ",
    "@@      ",
    "@@      ",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_M: Letter = [
    "@@@@  @@@@",
    "@@  @@  @@",
    "@@  @@  @@",
    "@@      @@",
    "@@      @@",
    "@@      @@",
    "          ",
    "          ",
];

#[rustfmt::skip]
const UPPERCASE_N: Letter = [
    "@@      @@",
    "@@@@    @@",
    "@@  @@  @@",
    "@@  @@  @@",
    "@@    @@@@",
    "@@      @@",
    "          ",
    "          ",
];

#[rustfmt::skip]
const UPPERCASE_O: Letter = [
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_P: Letter = [
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "@@      ",
    "@@      ",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_Q: Letter = [
    "@@@@@@@@  ",
    "@@    @@  ",
    "@@    @@  ",
    "@@    @@  ",
    "@@    @@  ",
    "@@@@@@@@@@",
    "          ",
    "          ",
];

#[rustfmt::skip]
const UPPERCASE_R: Letter = [
    "@@@@@@  ",
    "@@    @@",
    "@@    @@",
    "@@@@@@  ",
    "@@    @@",
    "@@    @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_S: Letter = [
    "@@@@@@@@",
    "@@      ",
    "@@@@@@@@",
    "      @@",
    "      @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_T: Letter = [
    "@@@@@@@@",
    "   @@   ",
    "   @@   ",
    "   @@   ",
    "   @@   ",
    "   @@   ",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_U: Letter = [
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_V: Letter = [
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    " @@  @@ ",
    "   @@   ",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_W: Letter = [
    "@@   @@   @@",
    "@@   @@   @@",
    "@@   @@   @@",
    "@@   @@   @@",
    "@@   @@   @@",
    "@@@@@@@@@@@@",
    "            ",
    "            ",
];

#[rustfmt::skip]
const UPPERCASE_X: Letter = [
    "@@    @@",
    "@@    @@",
    "  @@@@  ",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_Y: Letter = [
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "   @@   ",
    "   @@   ",
    "        ",
    "        ",
];

#[rustfmt::skip]
const UPPERCASE_Z: Letter = [
    "@@@@@@@@",
    "      @@",
    "    @@  ",
    "  @@    ",
    "@@      ",
    "@@@@@@@@",
    "        ",
    "        ",
];

// Put here punctuation and symbols i.e. $, #, @ etc.
#[rustfmt::skip]
const SPACE: Letter = [
    "      ",
    "      ",
    "      ",
    "      ",
    "      ",
    "      ",
    "      ",
    "      ",
];

#[rustfmt::skip]
const PERIOD: Letter = [
    "  ",
    "  ",
    "  ",
    "  ",
    "@@",
    "@@",
    "  ",
    "  ",
];

#[rustfmt::skip]
const QUESTION_MARK: Letter = [
    "        ",
    "  @@@@  ",
    "@@    @@",
    "     @@ ",
    "   @@   ",
    "        ",
    "   @@   ",
    "        ",
];

#[rustfmt::skip]
pub const UNKNOWN: Letter = [
    "  @@@@@@  ",
    "@@@    @@@",
    "@  @@@@  @",
    "@@@@@@  @@",
    "@@@@  @@@@",
    "@@@@@@@@@@",
    "@@@@  @@@@",
    "  @@@@@@  ",
];

pub fn get_symbols() -> HashMap<char, Letter> {
    HashMap::from([
        //
        // Lowercase.
        //
        ('a', LOWERCASE_A),
        ('b', LOWERCASE_B),
        ('c', LOWERCASE_C),
        ('d', LOWERCASE_D),
        ('e', LOWERCASE_E),
        ('f', LOWERCASE_F),
        ('g', LOWERCASE_G),
        ('h', LOWERCASE_H),
        ('i', LOWERCASE_I),
        ('j', LOWERCASE_J),
        ('k', LOWERCASE_K),
        ('l', LOWERCASE_L),
        ('m', LOWERCASE_M),
        ('n', LOWERCASE_N),
        ('o', LOWERCASE_O),
        ('p', LOWERCASE_P),
        ('q', LOWERCASE_Q),
        ('r', LOWERCASE_R),
        ('s', LOWERCASE_S),
        ('t', LOWERCASE_T),
        ('u', LOWERCASE_U),
        ('v', LOWERCASE_V),
        ('w', LOWERCASE_W),
        ('x', LOWERCASE_X),
        ('y', LOWERCASE_Y),
        ('z', LOWERCASE_Z),
        //
        // Uppercase.
        //
        ('A', UPPERCASE_A),
        ('B', UPPERCASE_B),
        ('C', UPPERCASE_C),
        ('D', UPPERCASE_D),
        ('E', UPPERCASE_E),
        ('F', UPPERCASE_F),
        ('G', UPPERCASE_G),
        ('H', UPPERCASE_H),
        ('I', UPPERCASE_I),
        ('J', UPPERCASE_J),
        ('K', UPPERCASE_K),
        ('L', UPPERCASE_L),
        ('M', UPPERCASE_M),
        ('N', UPPERCASE_N),
        ('O', UPPERCASE_O),
        ('P', UPPERCASE_P),
        ('Q', UPPERCASE_Q),
        ('R', UPPERCASE_R),
        ('S', UPPERCASE_S),
        ('T', UPPERCASE_T),
        ('U', UPPERCASE_U),
        ('V', UPPERCASE_V),
        ('W', UPPERCASE_W),
        ('X', UPPERCASE_X),
        ('Y', UPPERCASE_Y),
        ('Z', UPPERCASE_Z),
        //
        // Punctuation & other glyphs.
        //
        (' ', SPACE),
        ('.', PERIOD),
        ('?', QUESTION_MARK),
    ])
}
