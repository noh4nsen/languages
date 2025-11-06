fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    v.push(1);
    v2.push(8);

    let third = &v2[2];
    println!("The third element is {third}");

    let first = v.get(0);
    match first {
        Some(first) => println!("The third element is {first}!"),
        None => println!("There is no third element!"),
    }


    for i in &v {
        println!("{i}");
    }
    for i in &v2 {
        println!("{i}")
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}      {:p}", v, &v);

    enum SpreadsheeelCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheeelCell::Int(3),
        SpreadsheeelCell::Text(String::from("blue")),
        SpreadsheeelCell::Float(10.12),
    ];

    {
        let v = vec![100, 32, 57];
        println!("{:?}      {:p}", v, &v);
    }
    println!("{:?}      {:p}", v, &v);

}
