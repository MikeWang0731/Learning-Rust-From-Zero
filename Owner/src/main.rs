fn main() {
    // 所有权规则：
    // 每个值只能有一个变量，该变量是该值的所有者
    // 每个值同时只能有一个所有者
    // 当所有者超出作用域时，值被删除

    // Stack堆内存, Heap栈内存
    // e.g. String类型的指针，长度和容量存放在stack上，而字符串内容在heap上
    let mut s = String::from("Hello");
    s.push_str(",World!");
    println!("{}", s);  // Output: Hello World

    // Move: 浅拷贝s1的内容，但s1超出作用域，回收机制使得s1失效
    // let s1 = String::from("Hello");
    // let s2 = s1;
    // println!("{}", s1); // 错误的使用方法：value borrowed here after move

    // Clone: 数据深拷贝
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{},{}", s1, s2);

    // 对于一个存放在Stack上的数据来说，实现了copy之后旧数据仍然可用
    // 若实现了drop，那么copy就不可用了

    // 引用: &,允许引用某些值而不取得所有权
    let hello = String::from("Hello");
    // 借用：以引用作为函数参数的行为叫做借用
    // 不可以修改借用的东西！
    let len = cal_length(&hello);  // 这里我们没有复制，而是引用了hello
    println!("The length of hello is {}", len);
}

fn cal_length(s: &String) -> usize {
    // s在离开自己的作用域的时候并不会销毁自己指向的数据(指hello)，因为这是引用，并没有拿到所有权
    s.len()  // 不能有分号，因为要作为方法的返回值！返回传入字符串的长度
}
