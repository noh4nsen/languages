// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T, U2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![43, 342, 234, 645, 123, 345];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'm', 'q', 'k'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    println!("integer.x : {} integer.y : {}", integer.x, integer.y);

    let float: Point<f32, f32> = Point { x: 1.0, y: 4.0 };
    println!("float.x : {} float.y : {}", float.x, float.y);
    println!("Distance from origin {}", float.distance_from_origin());

    let mixed = Point { x: 'A', y: 3 };
    println!("mixed.x : {} mixed.y : {}", mixed.x(), mixed.y());

    let mixed = mixed.mixup(float);
    println!("mixed.x : {} mixed.y : {}", mixed.x(), mixed.y());
}
