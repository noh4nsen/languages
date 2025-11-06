use std::{thread, time::Duration};

pub fn expensive_closure() -> impl Fn(u64) -> u64 {
    |num: u64| -> u64 {
        println!("calculating slowly... {num}");
        thread::sleep(Duration::from_secs(num));
        num
    }
}

pub fn example_closure_1(num: u64) {
    let add_one_v1 = |x: u64| -> u64 { x + 1 };
    let add_one_v2 = |x| x + 1;
    let add_one_v3 = |x| x + 1;

    let a = add_one_v1(num);
    let b = add_one_v2(num);
    let c = add_one_v3(num);

    println!("{a} {b} {c}");
}

//Immutable borrow
pub fn example_closure_2(numbers: &Vec<u64>) {
    println!("Before defining closure: {:?}", numbers);

    let only_borrows = || println!("From closure: {:?}", numbers);

    println!("Before calling closure: {:?}", numbers);
    only_borrows();
    println!("After calling closure: {:?}", numbers);
}

//Mutable borrow
pub fn example_closure_3(numbers: &mut Vec<u64>) {
    println!("Before defining closure: {:?}", numbers);

    let mut borrows_mutably = || numbers.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", numbers);
}

pub fn example_closure_4(numbers: Vec<u64>) {
    println!("Before defining closure: {:?}", numbers);

    thread::spawn(move || println!("From thread: {:?}", numbers))
        .join()
        .unwrap();
}
