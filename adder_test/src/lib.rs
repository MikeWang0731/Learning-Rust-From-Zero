#[cfg(test)]
mod tests {
    #[test]  // test attribute
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);  // 判断result是不是4
    }

    #[test]
    fn another() {
        panic!("Loss")  // 触发panic就会失败
    }
}

/*
assert!：用来确定某个状态是否为true->true即通过,false即panic!
assert_eq!和assert_ne!：相当于==和!=，当断言失败的时候，会打印出两个参数的值，类似于失败的原因
 */

pub fn greeting(name: &str) -> String {
    format!("hello")
}

#[cfg(test)]
mod test2 {
    use super::*;
    #[test]
    // thread 'test2::greeting_contain' panicked at 'Greeting not contain hello'
    fn greeting_contain() {
        let result = greeting("Carol");
        // assert!(test_info, customized_information)
        assert!(result.contains("Carol"), "Greeting not contain {}", result);
    }
}

