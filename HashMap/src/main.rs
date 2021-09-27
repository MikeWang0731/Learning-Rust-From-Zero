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
}
