use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    // insert(key,value)
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    // 在元素类型为tuple的vector上使用collect方法创建hashmap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let ini_scores = vec![10, 50];
    // zip()类似于拉链，将teams.iter()和ini_scores.iter()结合在了一起
    let scores: HashMap<_, _> = teams.iter().zip(ini_scores.iter()).collect();
    // 注意所有权的变化！！

    // get()
    let find_name = "Blue".to_string();
    let find_score = scores.get(&find_name);
    match find_score {
        Some(score) => println!("{}", score),
        None=>println!("No Team.")
    }

    // 遍历
    for (k, v) in &scores {
        println!("{}:{}", k, v);
    };


    let mut new_scores = HashMap::new();
    new_scores.insert(String::from("Blue"), 10);
    // 更新
    // 替换现有的：new_scores.insert(String::from("Blue"), 10);
    new_scores.insert(String::from("Blue"), 15);

    // 在key不对应任何值的情况下，插入v -> 检查Yellow是否有对应的Value，若没有则加入"50"
    new_scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", new_scores);

    // 基于现有的value来更新value
    let text = "Hello World Wonderful World";
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        // 将当前单词计入map，若第一次出现，则value是0
        let count = text_map.entry(word).or_insert(0);  // 等号右边对应的是value
        // 将当前value值+1
        *count += 1;
    }
    println!("{:?}", text_map);


}
