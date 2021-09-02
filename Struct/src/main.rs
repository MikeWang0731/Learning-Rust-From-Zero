// struct是结构体，可以自定义数据类型，并打包，形成有意义的组合
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// fn area(width: u32, length: u32) -> u32 {
//     width * length
// }

struct Rectangle {
    width: u32,
    length: u32,
}

// struct的方法:impl + fn(self)
// 这样下面不需要给area传递参数，只需要实例化一个Rectangle类就可以了，area会自动拿到实例化后的参数
impl Rectangle {
    // implement from *Rectangle*
    fn area(&self) -> u32 {  // self表示拿到struct Rectangle中的属性
        self.length * self.width  // fn方法返回值：长*宽(计算面积)
    }
    fn can_contain(&self, other: &Rectangle) -> bool {  // 两个大参数，一个是"本身"的，一个是"对照组"的
        // 当前矩形是否能容纳另外一个？
        self.length > other.length && self.width > other.width
    }
    // 关联函数：不把self作为传入参数，但也是impl里的一部分
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size}
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}

fn main() {

    // 实例化User，若要在之后更新user1的内容，则需要设定user1为可变(mut)
    let mut user1 = User {
        username: String::from("Tom"),
        email: String::from("123@126.com"),
        active: true,
        sign_in_count: 5,
    };

    user1.email = String::from("456@126.com");
    // struct的更新
    let user2 = User {
        username: String::from("Amy"),
        email: String::from("123@163.com"),
        ..user1  // ..user1就是除了上面两行不一样的(重新指定的)，其他都跟user1一样
    };

    // struct例子：计算面积
    let rect = Rectangle { width: 30, length: 50 };
    println!("Area: {}", area(&rect));  // 引用方式传参，area不会拿到rect的所有权，area执行完毕之后rect不会被销毁
    println!("Area: {}", rect.area()); // 使用struct写成的方法impl，这里直接调用，不需要传递参数(因为是self)

    let rect2 = Rectangle { width: 10, length: 11 };
    if rect.can_contain(&rect2) { println!("1 can contain 2."); } else { println!("Can't contain 2"); }

    // 关联函数->Struct::method
    let square = Rectangle::square(3);
    println!("Square of rectangle: {}", square.area());
}
