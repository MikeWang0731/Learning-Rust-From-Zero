mod front_of_house{

    pub mod hosting{
        pub fn add_to_waitlist() {}
        pub fn seat_at_table(){}
    }

    mod serving{
        fn take_order(){}
        fn serve_order(){
            // 使用super关键字访问上级架构中的东西
            super::hosting::seat_at_table();
        }
        fn take_payment(){}
    }
}

// use关键字将hosting带入当前作用域
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
    // 因为use，这里才可以直接使用hosting
    hosting::seat_at_table();
}

mod track_food; // 直接以分号结尾就代表这是从另一个名叫track_food的文件引入的