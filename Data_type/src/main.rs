fn main() {

    // 当可能的类型比较多时，必须对类型进行标注！
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // i: integer  u: unsigned integer
    // 8-bit: i8/u8  16-bit: i16/u16  32-bit: i32/u32 64&128 ... arch: isize/usize(由计算机决定)

    // 整数溢出: u8(0-255)，假设把u8设置为256,那么：
    // 在编译模式下，程序会panic
    // 在发布模式下(--release)，会执行环绕，即256变成0，257变成1，此时不会panic

    // 浮点类型
    let x = 2.0; // 默认f64
    let x: f32 = 3.0;  // 显式更改f32

    // 数值操作
    let sum = 5 + 1;
    let difference = 95.5 - 3.2;
    let product = 4 * 2;
    let quotient = 53.1 / 2.2;  //整数不能除以浮点，否则no implementation for `{integer} / {float}`
    let reminder = 54 % 5;

    println!("{},{},{},{},{}", sum, difference, product, quotient, reminder);

    // 布尔类型
    let flag = false;

    // 字符类型 -> ''是char，""是str！
    let chr = 'z';
    let emoji = '😭';
    println!("{},{}", chr, emoji);

    // 复合类型：元组 Tuple
    // 可以将多个类型的多个值放在一个类型里，Tuple的长度是固定的，声明后无法改变
    // 在小括号里，用逗号隔开
    let tup = (4, 2.2, 'z');
    println!("tuple have {},{},{}", tup.0, tup.1, tup.2);
    // 解构：获取tuple的元素
    let (x, y, z) = tup; // tup有三个值，这里xyz与他们一一对应
    println!("{},{},{}", x, y, z);

    // 复合类型：数组
    // 也可以将多个值放在一个类型里，但是元素类型必须相同！长度固定！
    // 数组存放在栈内存(stack)上而不是堆(heap)，但不如Vector灵活，一般都是用Vector
    let list = [1, 2, 3, 4, 5]; // [Type; Length] -> [i32; 5]
    // 加入数组里的每个元素都相同，那么let a = [3;5]-> let a = [3,3,3,3,3] [init_val;len]

    // list[index]
    println!("list第一个元素:{}", list[0])
}
