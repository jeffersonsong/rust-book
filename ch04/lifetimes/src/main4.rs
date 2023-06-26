fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = "a quick fox jump over a log";
    let first_word = first_word(s);
    println!("first word is '{}'", first_word);
}