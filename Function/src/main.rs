// 声明函数使用fn关键字
// 针对函数和变量名，rust使用snake case命名规范，即所有字母为小写，下划线分隔
fn main() {
    println!("Hello, world!");
    another_function();  // func_name(paras); -> 调用函数
    print_a_num(5);
    println!("6 + 5 = {}", plus_five(6));

    // 语句与表达式：语句是执行动作的指令，表达式会计算一个值。函数的定义是语句！语句不包含返回值！所以不能把语句给let！
    // 语句返回的是一个空的() -> empty tuple
    let x = {
        let y = 1;
        y + 3  // 注意！这里没有逗号！这里是一个表达式，可以把表达式给let！
    };  // x(y=4) -> x=4
}

// parameter 和 argument 的区别：para为方法设计时设定的参数(形参)，arg为调用方法时传进去的参数(实参)
fn another_function() {
    println!("Another function!")
}

// 若有参数，必须指定参数类型！
fn print_a_num(x: i32) {
    println!("The num you pass in is {}", x);
}

// 函数的返回值是函数体里面最后一个表达式的值，若提前返回则需要return关键字
// 在->后面声明函数返回值的类型
fn plus_five(x: i32) -> i32 {  // -> i32: 返回i32类型的值
    x + 5  // 注意！这里没有逗号，否则就变成了语句，不是表达式了
}


