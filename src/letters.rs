use std::collections::HashMap;

type Symbol = [&'static str; 8];

#[rustfmt::skip]
const LOWERCASE_A: Symbol = [
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
const LOWERCASE_B: Symbol = [
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
const LOWERCASE_C: Symbol = [
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
const LOWERCASE_D: Symbol = [
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
const LOWERCASE_E: Symbol = [
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
const LOWERCASE_F: Symbol = [
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
const LOWERCASE_G: Symbol = [
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
const LOWERCASE_H: Symbol = [
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
const LOWERCASE_I: Symbol = [
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
const LOWERCASE_J: Symbol = [
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
const LOWERCASE_K: Symbol = [
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
const LOWERCASE_L: Symbol = [
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
const LOWERCASE_M: Symbol = [
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
const LOWERCASE_N: Symbol = [
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
const LOWERCASE_O: Symbol = [
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
const LOWERCASE_P: Symbol = [
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
const LOWERCASE_Q: Symbol = [
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
const LOWERCASE_R: Symbol = [
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
const LOWERCASE_S: Symbol = [
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
const LOWERCASE_T: Symbol = [
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
const LOWERCASE_U: Symbol = [
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
const LOWERCASE_V: Symbol = [
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
const LOWERCASE_W: Symbol = [
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
const LOWERCASE_X: Symbol = [
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
const LOWERCASE_Y: Symbol = [
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
const LOWERCASE_Z: Symbol = [
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
const UPPERCASE_A: Symbol = [
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
const UPPERCASE_B: Symbol = [
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
const UPPERCASE_C: Symbol = [
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
const UPPERCASE_D: Symbol = [
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
const UPPERCASE_E: Symbol = [
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
const UPPERCASE_F: Symbol = [
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
const UPPERCASE_G: Symbol = [
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
const UPPERCASE_H: Symbol = [
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
const UPPERCASE_I: Symbol = [
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
const UPPERCASE_J: Symbol = [
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
const UPPERCASE_K: Symbol = [
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
const UPPERCASE_L: Symbol = [
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
const UPPERCASE_M: Symbol = [
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
const UPPERCASE_N: Symbol = [
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
const UPPERCASE_O: Symbol = [
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
const UPPERCASE_P: Symbol = [
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
const UPPERCASE_Q: Symbol = [
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
const UPPERCASE_R: Symbol = [
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
const UPPERCASE_S: Symbol = [
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
const UPPERCASE_T: Symbol = [
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
const UPPERCASE_U: Symbol = [
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
const UPPERCASE_V: Symbol = [
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
const UPPERCASE_W: Symbol = [
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
const UPPERCASE_X: Symbol = [
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
const UPPERCASE_Y: Symbol = [
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
const UPPERCASE_Z: Symbol = [
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
const SPACE: Symbol = [
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
const PERIOD: Symbol = [
    "  ",
    "  ",
    "  ",
    "  ",
    "  ",
    "@@",
    "  ",
    "  ",
];

#[rustfmt::skip]
const COMMA: Symbol = [
    "  ",
    "  ",
    "  ",
    "  ",
    "  ",
    "@@",
    " @",
    "  ",
];

#[rustfmt::skip]
const COLON: Symbol = [
    "  ",
    "  ",
    "@@",
    "  ",
    "  ",
    "@@",
    "  ",
    "  ",
];

#[rustfmt::skip]
const SEMICOLON: Symbol = [
    "  ",
    "  ",
    "@@",
    "  ",
    "  ",
    "@@",
    " @",
    "  ",
];

#[rustfmt::skip]
const QUESTION_MARK: Symbol = [
    "@@@@@@@@",
    "@@    @@",
    "      @@",
    "   @@@  ",
    "        ",
    "   @@   ",
    "        ",
    "        ",
];

#[rustfmt::skip]
const EXCLAMATION_MARK: Symbol = [
    "@@",
    "@@",
    "@@",
    "@@",
    "  ",
    "@@",
    "  ",
    "  ",
];

#[rustfmt::skip]
const APOSTROPHE: Symbol = [
    "@@",
    "@@",
    "  ",
    "  ",
    "  ",
    "  ",
    "  ",
    "  ",
];

#[rustfmt::skip]
const QUOTES: Symbol = [
    "@@  @@",
    "@@  @@",
    "      ",
    "      ",
    "      ",
    "      ",
    "      ",
    "      ",
];

#[rustfmt::skip]
const OPENING_BRACKET: Symbol = [
    "@@@@",
    "@@  ",
    "@@  ",
    "@@  ",
    "@@  ",
    "@@  ",
    "@@@@",
    "    ",
];

#[rustfmt::skip]
const CLOSING_BRACKET: Symbol = [
    "@@@@",
    "  @@",
    "  @@",
    "  @@",
    "  @@",
    "  @@",
    "@@@@",
    "    ",
];

#[rustfmt::skip]
const OPENING_CURLY_BRACE: Symbol = [
    "    @@",
    "  @@  ",
    "  @@  ",
    "@@    ",
    "  @@  ",
    "  @@  ",
    "    @@",
    "        ",
];

#[rustfmt::skip]
const CLOSING_CURLY_BRACE: Symbol = [
    "@@    ",
    "  @@  ",
    "  @@  ",
    "    @@",
    "  @@  ",
    "  @@  ",
    "@@    ",
    "      ",
];

#[rustfmt::skip]
const OPENING_PARENTHESIS: Symbol = [
    "  @@",
    "@@  ",
    "@@  ",
    "@@  ",
    "@@  ",
    "@@  ",
    "  @@",
    "    ",
];

#[rustfmt::skip]
const CLOSING_PARENTHESIS: Symbol = [
    "@@  ",
    "  @@",
    "  @@",
    "  @@",
    "  @@",
    "  @@",
    "@@  ",
    "    ",
];

#[rustfmt::skip]
const ZERO: Symbol = [
    "  @@@@  ",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "  @@@@  ",
    "        ",
    "        ",
];

#[rustfmt::skip]
const ONE: Symbol = [
    "  @@",
    "@@@@",
    "  @@",
    "  @@",
    "  @@",
    "  @@",
    "    ",
    "    ",
];

#[rustfmt::skip]
const TWO: Symbol = [
    "@@@@@@@@",
    "      @@",
    "      @@",
    "  @@@@  ",
    "@@      ",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const THREE: Symbol = [
    "@@@@@@@@",
    "      @@",
    "@@@@@@@@",
    "      @@",
    "      @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const FOUR: Symbol = [
    "@@    @@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "      @@",
    "      @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const FIVE: Symbol = [
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
const SIX: Symbol = [
    "@@      ",
    "@@      ",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const SEVEN: Symbol = [
    "@@@@@@@@",
    "      @@",
    "      @@",
    "      @@",
    "      @@",
    "      @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const EIGHT: Symbol = [
    "@@@@@@@@",
    "@@    @@",
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "        ",
    "        ",
];

#[rustfmt::skip]
const NINE: Symbol = [
    "@@@@@@@@",
    "@@    @@",
    "@@    @@",
    "@@@@@@@@",
    "      @@",
    "      @@",
    "        ",
    "        ",
];

#[rustfmt::skip]
pub const UNKNOWN: Symbol = [
    "        ",
    "@@@@@@@@",
    "@      @",
    "@      @",
    "@      @",
    "@      @",
    "@@@@@@@@",
    "        ",
];

pub fn get_symbols() -> HashMap<char, Symbol> {
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
        (',', COMMA),
        (':', COLON),
        (';', SEMICOLON),
        ('?', QUESTION_MARK),
        ('!', EXCLAMATION_MARK),
        ('\'', APOSTROPHE),
        ('"', QUOTES),
        ('[', OPENING_BRACKET),
        (']', CLOSING_BRACKET),
        ('(', OPENING_PARENTHESIS),
        (')', CLOSING_PARENTHESIS),
        ('{', OPENING_CURLY_BRACE),
        ('}', CLOSING_CURLY_BRACE),
        ('0', ZERO),
        ('1', ONE),
        ('2', TWO),
        ('3', THREE),
        ('4', FOUR),
        ('5', FIVE),
        ('6', SIX),
        ('7', SEVEN),
        ('8', EIGHT),
        ('9', NINE),
    ])
}

pub struct Glyph {
    symbol: Vec<String>,
}

pub enum XOffset {
    Zero,
    One,
    Two,
}

pub enum YOffset {
    Zero,
    One,
}

impl Glyph {
    pub fn new(symbol: Symbol) -> Glyph {
        let mut lines = Vec::new();
        for line in symbol {
            lines.push(String::from(line));
        }
        Glyph { symbol: lines }
    }

    pub fn len(&self) -> usize {
        self.symbol.len()
    }

    pub fn get(&self, n: usize) -> String {
        if n >= self.symbol.len() {
            panic!("!!!")
        } else {
            self.symbol[n].replace("@", "█").replace(".", "░")
        }
    }

    pub fn with_shadow(&self, x_offset: XOffset, y_offset: YOffset) -> Glyph {
        // For now: x can be 0, 1 or 2; y can be either 0 or 1.
        let y;
        let x;

        match x_offset {
            XOffset::Zero => x = 0,
            XOffset::One => x = 1,
            XOffset::Two => x = 2,
        }

        match y_offset {
            YOffset::Zero => y = 0,
            YOffset::One => y = 1,
        }

        let mut map: Vec<Vec<char>> = Vec::new();

        for line in &self.symbol {
            let mut chars = Vec::new();
            for c in line.chars() {
                chars.push(c);
            }
            map.push(chars);
        }

        if x != 0 {
            for line in &mut map {
                line.push(' ');
                line.push(' ');
            }
        }

        for row in 0..map.len() - 1 {
            for col in 0..map[0].len() - 2 {
                if map[row][col] == '@' {
                    let shadow_x = col + x;
                    let shadow_y = row + y;
                    if map[shadow_y][shadow_x] == ' ' {
                        map[shadow_y][shadow_x] = '.';
                    }
                }
            }
        }

        let mut lines = Vec::new();
        for line in map {
            let mut s = String::new();
            for ch in line {
                s.push(ch);
            }
            lines.push(s);
        }

        Glyph { symbol: lines }
    }

    pub fn to_small_glyph(&self) -> Glyph {
        const TOP_LEFT: &str = "@   ";
        const TOP_RIGHT: &str = " @  ";
        const BOT_LEFT: &str = "  @ ";
        const BOT_RIGHT: &str = "   @";
        const TOP_LEFT_AND_BOT_RIGHT: &str = "@  @";
        const TOP_RIGHT_AND_BOT_LEFT: &str = " @@ ";
        const TOP_HALF: &str = "@@  ";
        const BOT_HALF: &str = "  @@";
        const LEFT_HALF: &str = "@ @ ";
        const RIGHT_HALF: &str = " @ @";
        const FULL_BLOCK: &str = "@@@@";
        const EMPTY_BLOCK: &str = "    ";
        const MINUS_TOP_LEFT: &str = " @@@";
        const MINUS_TOP_RIGHT: &str = "@ @@";
        const MINUS_BOT_LEFT: &str = "@@ @";
        const MINUS_BOT_RIGHT: &str = "@@@ ";

        let block_map = HashMap::from([
            (TOP_LEFT, '▘'),
            (TOP_RIGHT, '▝'),
            (BOT_LEFT, '▖'),
            (BOT_RIGHT, '▗'),
            (TOP_LEFT_AND_BOT_RIGHT, '▚'),
            (TOP_RIGHT_AND_BOT_LEFT, '▞'),
            (TOP_HALF, '▀'),
            (BOT_HALF, '▄'),
            (LEFT_HALF, '▌'),
            (RIGHT_HALF, '▐'),
            (FULL_BLOCK, '█'),
            (EMPTY_BLOCK, ' '),
            (MINUS_TOP_LEFT, '▟'),
            (MINUS_TOP_RIGHT, '▙'),
            (MINUS_BOT_LEFT, '▜'),
            (MINUS_BOT_RIGHT, '▛'),
        ]);

        let mut lines = Vec::new();

        for i in (0..self.symbol.len()).step_by(2) {
            let line1: Vec<char> = self.symbol[i].chars().collect();
            let line2: Vec<char> = self.symbol[i + 1].chars().collect();

            let mut next_line = String::new();

            for j in (0..line1.len()).step_by(2) {
                let mut key = String::new();
                key.push(line1[j]);
                key.push(line1[j + 1]);
                key.push(line2[j]);
                key.push(line2[j + 1]);

                next_line.push(block_map[key.as_str()]);
            }

            lines.push(next_line);
        }

        Glyph { symbol: lines }
    }
}
