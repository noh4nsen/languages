struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-Like Structs, para implementacao somente
//de comportamentos usando traits
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.sign_in_count = 2;
    print_user(&user1);

    let user2 = build_user(String::from("email@email.com"), String::from("newUser123"));
    print_user(&user2);

    let user3 = User {
        email: String::from("new_email@example.com"),
        ..user1
    };
    print_user(&user3);
    //print_user(&user1);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let end = Point(1, 2, 3);

    println!("{} {} {}", black.0, origin.0, end.0);
    println!("{} {} {}", black.1, origin.1, end.1);
    println!("{} {} {}", black.2, origin.2, end.2);

    let _subject = AlwaysEqual;
    //println!("{}", subject);
}

fn print_user(user: &User) {
    println!("===============================================");
    println!("email         ::  {}", user.email);
    println!("username      ::  {}", user.username);
    println!("active        ::  {}", user.active);
    println!("sign in count ::  {}", user.sign_in_count);
    println!("===============================================");
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
