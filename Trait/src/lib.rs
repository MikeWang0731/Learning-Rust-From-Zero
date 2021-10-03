use std::fmt::Display;

// 只有方法签名，没有具体实现，每个方法占一行，以分号结尾
pub trait Summary {
    fn summarize(&self) -> String;
    // 默认实现：即在trait中直接定义好，就不需要在单独实现了
    // 默认实现的方法也可以调用trait中其他的方法
    fn which_source(&self) {
        println!("From a newspaper.");
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为struct结构体实现trait：impl trait_name for struct_name
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
}

// Newspaper和Tweet是两个不同的社交媒体，但他们都实现了同样的功能Summary()，这就是trait的意义
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} pushed {}", self.username, self.content)
    }
}

// 将summary这个trait当作参数传入，即这个方法可以使用summary的东西
pub fn notify(message: impl Summary) {
    println!("Big message: {}", message.summarize());
}

// Trait Bound
pub fn notify1<T: Summary>(item: T) {
    println!("Big message: {}", item.summarize());
}

// Trait Bound + 多个trait (同时实现Summary和Display)
pub fn notify2<T: Summary + Display>(item: T) {
    println!("Big message: {}", item.summarize());
}



/*

impl<T> Pair<T> {
    fn new(x: T, y: T) ->Self<>{
        Self { x, y }
    }
}
如果说T类型可以实现Display和PartialOrd，那么就可以用下面这个trait
impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x > self.y{
            println!("{} is bigger", self.x);
        } else { println!("It is y.") }
    }
}
 */