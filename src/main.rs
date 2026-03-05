use crate::letters::{Glyph, UNKNOWN, XOffset, YOffset, get_symbols};

mod letters;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let msg;
    if args.len() == 1 {
        msg = "Hello, world!";
    } else {
        msg = &args[1];
    }

    let key_map = get_symbols();
    let mut big_words = Vec::new();

    for ch in msg.chars() {
        big_words.push(Glyph::new(*key_map.get(&ch).unwrap_or(&UNKNOWN)));
    }

    println!();
    println!();

    let spacing = "";

    for n in 0..big_words[0].len() {
        for glyph in &big_words {
            print!(
                "{}{}",
                glyph.with_shadow(XOffset::Two, YOffset::One).get(n),
                spacing
            );
        }
        println!();
    }

    println!();
    println!();

    let spacing = "  ";

    for n in 0..big_words[0].len() {
        for glyph in &big_words {
            print!("{}{}", glyph.get(n), spacing);
        }
        println!();
    }

    println!();
    println!();

    let mut small_words = Vec::new();
    for ch in msg.chars() {
        small_words.push(Glyph::new(*key_map.get(&ch).unwrap_or(&UNKNOWN)).to_small_glyph());
    }

    let spacing = " ";

    for n in 0..small_words[0].len() {
        for glyph in &small_words {
            print!("{}{}", glyph.get(n), spacing);
        }
        println!();
    }

    println!("You've printed: '{msg}'");
}
