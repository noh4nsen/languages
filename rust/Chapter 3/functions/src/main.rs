fn main() {
    another_function(2);
    print_labeled_measurement(5, 'h');

    //Statements and Expressions
    let y = {
        let x = 3;
        let w = 2;
        x + w
    };
    println!("The value of y is: {y}");

    //functions with return values
    println!(
        "Teste funcao com retorno {}, {}",
        five() + five(),
        plus_one(3)
    );

    //Control flow
    let number = -1;
    if number < 5 && number > 1 {
        println!("condition was true");
    } else if number != 0 {
        println!("number is not zero");
    } else {
        println!("condition was false");
    }

    let number_2 = if 3 == 3 { 'A' } else { 'B' };
    println!("{number_2}");

    //loops
    // loop {
    //     println!("again!");
    // }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//functions with return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
