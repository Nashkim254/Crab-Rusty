use std::collections::HashMap;

pub fn run() {
    let a = [1, 2, 3, 4, 5];
    let mut v: Vec<i32> = Vec::new();
    v.push(6);
    v.push(7);
    v.push(8);

    let v2 = vec![4, 5, 6, 7, 8, 9];
    let third = &v2[2];
    println!("the third element is:{}", third);
    match v2.get(2) {
        Some(t) => println!("the third element is:{}", t),
        None => println!("there is no third element "),
    }

    for i in &v2 {
        println!("{}", i)
    }

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(3.142),
        SpreadSheetCell::Text(String::from("Blue")),
    ];

    match &row[1] {
        SpreadSheetCell::Int(i) => println!("{}", i),
        _ => println!("No int"),
    }

    let blue = String::from("Blue");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    let team = String::from("Blue");
    let score = scores.get(&team);
    for (k, v) in &scores {
        println!("key {}, value {}", k, v)
    }
}
