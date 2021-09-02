fn main() {
    let s = String::from("Hello World");
    let word_index = first_word(&s);

    println!("Word Index is: {}", word_index);

    // 字符串切片：&str_name[startIndex..endIndex] -> [前闭,后开)
    let hello = &s[0..5];  //同：[..5]
    let world = &s[6..11];  // 同：[6..]

    let whole = &s[..];  // 选取全部
    println!("Whole is {}, 1 is {}, 2 is {}", whole, hello, world);
}

fn first_word(s: &Str) -> &str {
    let bytes = s.as_bytes();  // 字符串转换为字节数组
    // iter()转换为可迭代，enumerate()会对结果进行包装，包装形式为(index,value)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }  // 如果找到了空格，就截取这个index之前的东西
    }
    &s[..] // 方法的返回值为整个字符串
}
