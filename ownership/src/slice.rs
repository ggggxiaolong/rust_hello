pub fn test_slice(){
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..=10];
    println!("{}, {}", hello, world)
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}