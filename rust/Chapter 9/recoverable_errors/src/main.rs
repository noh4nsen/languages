use std::{
    fs::{File, self},
    io::{Error, ErrorKind, Read},
};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = check_file(greeting_file_result);

    //Closure, reduz quantidade de match necessarios
    let greeting_file = File::open("hello2.txt")
                                .unwrap_or_else(|error| { check_with_closure(error) });

    let greeting_file = File::open("hello3.txt")
        .expect("hello3.txt doesn't exist");

    // Error propagation
    let username_result = read_username_from_file("hello.txt");

    // Error propagation using the ?
    let username_result = read_username_from_file_2("teste.txt");

    // Using fs
    let username_result = fs::read_to_string("hello.txt");

    println!("{:?}", last_char_of_first_line(""));
    println!("{:?}", last_char_of_first_line("teste pegar ultimo caractere"));
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_username_from_file_2(file_name: &str) -> Result<String, Error> {
    let mut username = String::new();

    File::open(file_name)?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file(file_name: &str) -> Result<String, Error> {
    let username_file_result = File::open(file_name);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn check_with_closure(erro: Error) -> File {
    if erro.kind() == ErrorKind::NotFound {
        File::create("hello2.txt").unwrap_or_else(|erro| {
            panic!("Problem creating the file: {:?}", erro);
        })
    } else {
        panic!("Problem opening the file: {:?}", erro);
    }
}

fn check_file(file: Result<File, Error>) -> File {
    match file {
        Ok(file) => file,
        Err(error) => check_file_notfound(error.kind()),
    }
}

fn check_file_notfound(kind: ErrorKind) -> File {
    match kind {
        ErrorKind::NotFound => check_file_creation(File::create("hello.txt")),
        other_error => panic!("Problem opening the file: {:?}", other_error),
    }
}

fn check_file_creation(file_result: Result<File, Error>) -> File {
    match file_result {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
    }
}
