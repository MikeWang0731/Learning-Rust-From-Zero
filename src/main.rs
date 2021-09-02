// fn main() 是定义了一个主函数，它是Rust中最先执行的代码，main()的括号里无参数
fn main() {
    // 缩进是4个空格，而不是tab
    // println! 是一个Rust Macro(Rust宏)，如果是函数的话就没有感叹号(!) -> 即:println
    println!("hello world!");  // 代码以';'结尾
}

// rustc file_name -> 编译，编译之后还会生成一个.pdb文件，包含调试信息
// Rust是ahead-of-time，即预先编译的语言，可以将编译好的文件交给别人直接运行
// rustc只适合简单文件的编译