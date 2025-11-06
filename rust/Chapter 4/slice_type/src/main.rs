fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("{}", word);

    //String slices
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let slice = &s[0..3];
    println!("{}", slice);
    let slice = &s[..3];
    println!("{}", slice);

    let len = s.len();
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);

    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);

    let slice = first_word_slice(&s);
    println!("{}", slice);

    s.clear();
    println!("{}", s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    let slice = &a[0..3];
    assert_eq!(slice, &[1, 2, 3]);
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
