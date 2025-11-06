const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x: u32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //Constants
    println!("Three hours are equal to {THREE_HOURS_IN_SECONDS} seconds");

    //Shadowing
    let y = 5;
    let y = y + 1;

    println!("The value of x is: {y}");
    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }
    println!("The value of x is: {y}");
}
