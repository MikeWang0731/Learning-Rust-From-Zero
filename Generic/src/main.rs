fn main() {
    let p1 = Point { x: 1, y: 2 };
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    };
    largest
}

struct Point<T> {
    // x和y必须是相同类型的，若想实现不同类型，则可以写Point<T, U>
    x: T,
    y: T,
}

// 把T放在impl关键字后面，表示在类型T上实现方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum Option<T> {
    Some(T),
    None,
}