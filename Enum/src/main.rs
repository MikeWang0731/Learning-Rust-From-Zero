use crate::IpAddrKind::V4;

fn main() {

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // IpAddr - Struct的实例化
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // 数据附加到枚举后调用
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    // option Enum -> enum Option<T> {Some(T), Option<T>, None,}
    // Rust没有Null的这个设计，若想使用Option中的T，则必须将它转化为T，避免null泛滥
    let some_number = Some(5);
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;

    let c = Coin::Quarter(CanState::Alberta);
    println!("{}", value_in_cents(c));

    let v = Some(0u8);
    // 只针对some(3)，但也放弃了穷举，
    if let Some(3) = v {
        println!("three");
    } else { println!("Other"); };
}

fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 数据附加到枚举：不需要额外使用struct，每个变体可以拥有不同的类型和数据量
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 枚举和模式匹配
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(CanState),
}

// 绑定值的模式匹配
#[derive(Debug)]
enum CanState {
    Alberta,
    Vancouver,
}


fn value_in_cents(coin: Coin) -> u8 {
    // match表达式的结果就会作为函数的结果返回
    match coin {
        // 如果第一个匹配不上就会匹配第二个，最终返回匹配上的那个分支的代码块
        Coin::Penny => 1,  // 分支1
        Coin::Nickel => 5,  // 分支2
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The state is:{:?}", state);
            25
        }
    }
}





