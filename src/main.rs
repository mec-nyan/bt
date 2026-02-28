use crate::letters::{UNKNOWN, get_symbols};

mod letters;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        return;
    }

    let msg = &args[1];

    let key_map = get_symbols();
    let mut big_words = Vec::new();

    for ch in msg.chars() {
        big_words.push(key_map.get(&ch).unwrap_or(&UNKNOWN));
    }

    let spacing = "  ";

    for n in 0..big_words[0].len() {
        for letter in &big_words {
            print!("{}{}", letter[n].replace("@", "█"), spacing);
        }
        println!();
    }

    println!("::{msg}");
}
