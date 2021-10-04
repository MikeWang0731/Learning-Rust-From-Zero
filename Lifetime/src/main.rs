fn main() {
    let string1 = String::from("abcd");

    // &str的生命周期是全局静态，即一直有效
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest is {}", result);
}

/*
声明周期以`开头，放在&符号的后面，和类型名称以空格相隔 -> &'a i32(生命周期为`a的i32类型数据)
它描述的是生命周期之间的关系
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // x和y和返回值拥有一样的生命周期
    // longest的`a得到的生命周期是x和y之间比较短的那一个
    // 若a的2-9,b是5-7，则`a最终拿到的是5-7行的生命周期
    if x.len() > y.len() {
        x
    } else { y }
}
