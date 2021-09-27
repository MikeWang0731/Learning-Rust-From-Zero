fn main() {
    let v: Vec<i32> = Vec::new();  // i32类型的数据，但不是必须声明
    let mut v = vec![1, 2, 3];  // 使用vec宏来声明
    v.push(1); // 向v中添加一个元素

    let third_e = &v[2];
    println!("The 3rd element is {}", third_e);

    match v.get(2) {
        Some(third) => println!("The 3rd element is {}", third),
        None => println!("There is no 3rd element"),
    };

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell{
    Int(i32), Float(f64),Text(String),
}
