pub fn run() {
    let s = "Hello world";
    let word = first_word(s);
    print!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
