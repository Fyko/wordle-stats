#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod fetch;
pub mod types;
pub mod media;

use anyhow::Result;
use num_format::{Locale, ToFormattedString};
use egg_mode::tweet::DraftTweet;
use parser::{GuessChar, Wordle};
use std::env::var;
use chrono::prelude::*;

fn calc_percentage_string(num: u32, den: u32) -> f64 {
    let percentage = (num as f64 / den as f64) * 100.0;
    return percentage.round();
}

fn calc_squares(decimal: f64) -> f64 {
    return (20_f64 * decimal).round();
}

// needs to run at 7am EST every day, fetch previous day's data
// if today was 261, at 5am EST, we display 260

#[tokio::main]
async fn main() -> Result<()> {
    let token = egg_mode::Token::Access {
        consumer: egg_mode::KeyPair::new(
            var("TWITTER_CONSUMER_KEY").unwrap(),
            var("TWITTER_CONSUMER_SECRET").unwrap(),
        ),
        access: egg_mode::KeyPair::new(
            var("TWITTER_ACCESS_KEY").unwrap(),
            var("TWITTER_ACCESS_SECRET").unwrap(),
        ),
    };

    let day: u16 = 261;
    let now_timestamp = Utc::now().timestamp_millis();
    println!("{}", now_timestamp);

    let client = reqwest::Client::new();
    let req = fetch::fetch_scores(&day, &client).await?;
    let total: u32 = req
        .data
        .result
        .iter()
        .fold(0, |acc, x| acc + x.value.1.parse::<u32>().unwrap());

    let hardreq = fetch::fetch_hard_mode(&day, &client).await?;
    let hard = hardreq.data.result[0].value.1.parse::<u32>().unwrap();

    let darkreq = fetch::fetch_dark_mode(&day, &client).await?;
    let dark = darkreq.data.result[0].value.1.parse::<u32>().unwrap();

    let mut lines = vec![
        format!("#Wordle - Day {}", day),
        format!(
            "{} results captured",
            total.to_formatted_string(&Locale::en)
        ),
        format!("{} hard mode users", hard.to_formatted_string(&Locale::en)),
        format!("{} dark mode users", dark.to_formatted_string(&Locale::en)),
    ];

    for r in req.data.result {
        let score = r.metric.score;
        let (_, val) = r.value;
        let parsed_val = val.parse::<u32>().unwrap();

        let percent = calc_percentage_string(parsed_val, total);
        let emojis = "🟩".repeat(calc_squares(percent / 100_f64) as usize);

        lines.push(format!(
            "{}: {} {} ({}%)",
            score,
            emojis,
            parsed_val.to_formatted_string(&Locale::en),
            percent
        ));
    }

    let content = lines.join("\n");
    println!("{}", content);

    let media_id = fetch::fetch_panel_image(&day, &now_timestamp, &token, &client).await?;
    let mut tweet = DraftTweet::new(content);
    tweet.add_media(media_id);

    println!("{:#?}", tweet);

    Ok(())
}
