fn main() {
    let s1 = String::from("Teste tamanho de string abluble!");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
