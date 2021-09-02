fn main() {
    // if语句
    // 如果语句多于一个else if，则推荐用match重构代码
    let number = 6;
    if number % 4 == 0 {
        println!("Divided by 4");
    } else if number % 3 == 0 {
        println!("Divided by 3")
    } else { println!("Other case"); }

    let condition = true;
    // 5和6是if-else函数中最后一个表达式返回的值,所以可以被let接收
    let number = if condition { 5 } else { 6 };  // true:5, false:6
    println!("The number is {}", number);

    // 三种循环：loop, while, for

    // loop:无限执行的循环，直到满足特定停止条件
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // 切断循环并返回值counter*2给result
        }
    };
    println!("Result for loop {}", result);

    // while: 每次循环都判断一次条件，不满足条件时停止运行
    let mut while_count = 3;
    while while_count != 0 {
        println!("This round, the while count is {}", while_count);
        while_count = while_count - 1;
    };
    println!("While count is zero!");

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {  // iter()会返回一个可以便遍历的东西
        println!("The element in a is {}", element);
    };
    // for+range: range可以生成一个范围[start,end)，rev方法可以反转这个范围(123->321)
    // 实现3,2,1,发射！
    for number in (1..4).rev() {
        println!("{}!", number);
    };
    println!("Launch!")
}
