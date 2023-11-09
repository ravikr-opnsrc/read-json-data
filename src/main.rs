extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

fn main() {
    let json_str = r#"
    {
        "article": "How to work with JSON in Rust",
        "author": "Mr. Anonymous",
        "paragraph": [
            {
                "name": "starting sentence"
            },
            {
                "name": "body of paragraph"
            },
            {
                "name": "end of the paragraph"
            }
        ]
    }
    "#;

    // Deserialize JSON into a Rust data structure
    let article: Article = serde_json::from_str(json_str).unwrap();

    println!("Article: {}", article.article);
    println!("Author: {}", article.author);

    for paragraph in article.paragraph {
        println!("Paragraph: {}", paragraph.name);
    }


    let article2 = Article {
        article: "How to work with JSON in Rust".to_string(),
        author: "Ravikant Kumar".to_string(),
        paragraph: vec![
            Paragraph { name: "starting sentence".to_string() },
            Paragraph { name: "body of paragraph".to_string() },
            Paragraph { name: "end of the paragraph".to_string() },
        ],
    };

    let json_str = serde_json::to_string(&article2).unwrap();

    println!("\n\n{}", json_str);
}
