# Table of Contents

*Rust Learning - Mike Wang*

1. src/Hello_World
    - println!
2. Hello_Cargo
    - Cargo和普通文件的区别
3. Guess_game
    - 生成随机数
    - match (switch控制语句)
    - use/let/loop
    - .expect()/.expect() with match
4. Variable
    - 变量可变/不可变，即mut关键字
    - 遮蔽机制: shadow
    - const: 常量类型
5. Data_type
    - 整数类型 & 浮点类型 & 字符类型
    - 四则运算
    - 布尔类型
    - 元组tuple & 数组array
6. Function
    - 命名规范
    - 语句与表达式/返回值
    - paras & args
7. Control_flow
    - if, loop, while, for
8. Owner
    - 所有权和作用域
    - 堆和栈内存
    - move/clone/copy
    - 引用和借用，&符号
9. Slice
    - 字符串切片
    - 字符串切片的引用 &str
    - 普通数组的切片
10. Struct
    - 初始化，且可作为返回值，可变性，实例化
    - Tuple Struct, Unit-like Struct
    - impl, 关联函数
    - 匿名Struct
11. Enum 枚举
    - 创建，变体，枚举值
    - 将数据附加到枚举的变体中
    - impl为枚举定义方法
    - Option枚举
    - Match：模式匹配和枚举/绑定值的模式
    - if-let: 处理只关心一种匹配而忽略其他匹配情况 
12. Package, Library, Module, Path
    - 包含关系，定义
    - Path的绝对路径：从crate root开始
    - Path的相对路径：从模块开始，使用self或者super
    - 私有边界：默认所有的条目都是私有的，外界访问不到，根级别除外
    - pub struct/enum: s是公共的，s下的字段是私有的！e本身就公共
    - 需要单独设置pub才能变为公有
    - 将模块放在其他文件
13. use关键字，as关键字
    - 引用包，模块
    - as：类似于python -> import A as alpha
    - use也可以加上pub关键字
    - 嵌套路径引入包：use::std::{cmp::Ordering,io}
    - '*'使用通配符引入，需要谨慎
14. Vector
    - 由标准库提供，存储多个值，存相同类型数据，内存中连续存放
    - 添加、删除(作用域内有效)、读取
    - Vec和enum
15. String
    - Rust的核心层面只提供了一个字符串，就是str类型
    - String来自于标准库，而不是核心
    - 初始化
    - 按索引访问，切片(需要按边界切割)
16. HashMap
    - <key,value>,key为同一种数据类型,value为同一种数据类型
    - 创建，添加，访问，遍历
    - 使用collect方法创建
    - 如i32这种copy trait的类型，值会复制到hashmap中
    - 如String这种拥有所有权的，值会移动，所有权转移给hashmap