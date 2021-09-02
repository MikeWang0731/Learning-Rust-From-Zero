fn main() {

    // 变量默认不可变，immutable
    // 编译器自动将x推断为i32类型数据
    let x = 5;

    //使用println!宏输出x的值
    println!("The number of x is {}", x);

    //x = 6;  // x重新赋值为6会报错 -> Cannot assign twice to immutable variable

    // mut: 使变量可变
    let mut change_x = 5;
    change_x = 7;  // x重新赋值为7

    // const声明常量，类型必须标注清楚，常量不可变，不可被mut
    const MAX_POINTS: u32 = 100000;

    // shadow:遮蔽，覆盖以前声明的同名变量
    let x = 6; // 之前x是5，从这一行开始x被定义为6
    // e.g. let x = 5, let x = x+1, let x = x*2 -> x = (5+1)*2 = 12

    // 使用let的shadow可以和之前的类型不同(见下方)
    // mut的话必须是同类型数据，否则报错
    let space = "    ";  //str
    let space = space.len();  //usize
}
