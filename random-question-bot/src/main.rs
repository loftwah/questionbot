use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::error::Error;
use std::fs;

use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Question {
    question: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let questions_file = fs::read_to_string(&env::var("QUESTIONS")?)?;
    let questions: Vec<Question> = serde_json::from_str(&questions_file)?;

    let mut rng = rand::thread_rng();
    let shuffled_questions = questions.choose_multiple(&mut rng, questions.len()).cloned().collect::<Vec<_>>();
    let question = &shuffled_questions[0];

    let question_text = &question.question;

    let json = json!({
        "content": question_text,
        "username": "Random Question Bot"
    });

    let client = reqwest::Client::new();
    let webhook_url = &env::var("WEBHOOK")?;
    let res = client.post(webhook_url)
        .json(&json)
        .send()
        .await?;
    println!("{:?}", res);

    Ok(())
}
