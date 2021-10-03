mod lib;
use Trait::NewsArticle;  // proj_name::struct
use Trait::Summary;  // proj_name::trait

fn main() {
    let newpaper = NewsArticle {
        headline: "UofA".to_string(),
        location: "Edmonton".to_string(),
        author: "stuLib".to_string(),
        content: "Hello".to_string(),
    };

    // 调用summarize这个trait
    println!("{}", newpaper.summarize());
}


