use std::mem::size_of_val;

fn main() {
    let data = "initial contents";
    let mut s = data.to_string();

    let mut s2 = "initial contents".to_string();

    let s3 = String::from("initial contents");

    print_formated("data",data);
    print_formated("s",&s);
    print_formated("s2",&s2);
    print_formated("s3",&s3);

    s.push_str(" + append");
    print_formated("s",&s);

    s2.push_str(&s3);
    print_formated("s2",&s2);
    print_formated("s3",&s3);

    s.push('A');
    print_formated("s",&s);

    let s4 = String::from("Hello, ");
    print_formated("s4", &s4);
    println!("s4 block {:p}", &s4);
    let s5 = String::from("world!");
    println!("s5 block {:p}", &s5);
    let s6 = s4 + &s5; // s4 movido e nao pode mais ser usado
    println!("s6 block {:p}", &s6);

    //print_formated("s4", &s4);
    print_formated("s5", &s5);
    print_formated("s6", &s6);

    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let s_aux = s7.to_string();
    let s10 = s7 + "-" + &s8 + "-" + &s9;
    print_formated("s_aux", &s_aux);
    print_formated("s10", &s10);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    print_formated("s", &s);

    let hello = "Здравствуйте";
    for c in hello.chars() {
        print!("{c} ");
    }
    println!();
    for b in hello.bytes() {
        print!("{b} ");
    }
}

fn print_formated(name: &str, s: &str) {
    println!("-----------------------------------------------------");
    println!("Name          | {}", name);
    println!("Content       | {}", s);
    println!("Memory Size   | {}", size_of_val(&*s));
    // dentro da funcao esse print de memory block nao representa mais
    // o bloco real da variavel por ser uma referencia
    //println!("Memory Block  | {:p}", &s);
    println!("-----------------------------------------------------");
}
