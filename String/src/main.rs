fn main() {
    // 初始化方式
    let s = String::new();
    let s = "New String".to_string();
    let s = String::from("New String");

    // push_str()
    let mut foo = String::from("foo");
    foo.push_str("bar");
    println!("{}",foo);  //foobar

    // push()
    let mut lo = String::from("lo");
    lo.push('l');  // 这里是char!是单引号
    println!("{}", lo);

    // +
    let part_a = "Hello ".to_string();
    let part_b = "world".to_string();
    let part_c = part_a + &part_b;  // 第二部分是引用，这里part_a会失效
    println!("All is {}", part_c);

    // format!
    let a = "Hello ".to_string();
    let b = "world".to_string();
    let c = format!("{} and {}", a, b);
    println!(c);
}
