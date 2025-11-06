#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn constructor(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn soma_lados(&self) -> u32 {
        (self.width * 2) + (self.height * 2)
    }

    fn multiplica_altura(&self, i: u32) -> u32 {
        self.height * i
    }
}

fn main() {
    let width1: u32 = 30;
    let height1: u32 = 50;
    let rect1: (u32, u32) = (width1, height1);
    let rect2: Rectangle = Rectangle::constructor(dbg!(15 * width1), height1);
    let rect3: Rectangle = Rectangle::constructor(width1, height1);


    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    println!("The area of the rectangle is {} square pixels.", area_2(rect1));
    println!("The area of the rectangle is {} square pixels.", rect2.area());
    println!("Soma de lados {}", rect2.soma_lados());
    println!("Multiplica altura {} por {} = {}", rect2.height, 2, rect2.multiplica_altura(2));
    println!("rect2 is {:#?}", rect2);
    println!("rect3 is {:#?}", rect3);

    //macro dbg! printa no stderr
    dbg!(&rect2);
}

fn area(width: u32, height: u32) -> u32 { width * height }

fn area_2(dimensions: (u32, u32)) -> u32 { dimensions.0 * dimensions.1 }