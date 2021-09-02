use std::io;
//输入输出需要用到io库，io库在std标准库中
use rand::Rng;
// Rng是一个接口，会定义很多方法
use std::cmp::Ordering;

fn main() {
    // 提示语句
    println!("Guess a number!");

    // let:声明了一个变量
    // Rust中变量默认不可变，即immutable
    // 生成一个[1,101)的随机数
    let secret_number = rand::thread_rng().gen_range(1, 101); // 随机数生成器
    println!("Secret is: {}", secret_number);

    // 无限循环
    loop {
        // 想让变量可变需要mut(mutable)关键字
        // String::new()会创建一个字符串实例，两个冒号(::)代表new()是String类型的一个关联函数，即静态方法
        let mut guess = String::new();

        // stdin()用于处理终端的输入，将输入放在guess字符串中，&表示引用，即访问同一块内存，引用在rust中默认也不可变！
        // 方法的参数(read_line(args))按引用(&mut guess)进行传递，引用可以在程序的不同位置访问同一块内存
        // read_line()会返回一个Result类型数据，Result包含ok(运行成功)或者Err(运行失败以及原因)和expect(潜在错误处理)
        // expect()：当运行失败是，它截断当前程序并打印出msg；若成功则会打印出正确的结果
        io::stdin().read_line(&mut guess).expect("Can not read");

        // {}表示占位符 -> "{} {} ...", str1, str2, ...
        println!("The number you are guessing is: {}", guess);

        // shadow:类型遮蔽，即允许复用变量名称，隐藏之前的变量，下面的代码都将用这个guess，多用于类型转换
        // trim()去掉空白，回车等，parse()将字符串解析为数字类型，解析为u32类型(无符号整数)
        // guess: u32 - 显式声明变量类型
        // 使用match来处理Result是rust中的惯用手段
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // cmp(compare):与另一个值进行比较
        // 模式匹配 =>
        // secret_number被自动转换成u32类型
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big"),
        };
    }
}
