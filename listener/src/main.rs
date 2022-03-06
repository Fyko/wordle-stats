mod metrics;
mod common;

#[macro_use]
extern crate log;

use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;
use egg_mode::stream::StreamMessage;
use futures::TryStreamExt;
use parser::parse;
use prometheus::{
    Encoder, TextEncoder,
};
use std::env::var;
use metrics::{GAME_COUNTER_VEC, TWEET_COUNT};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

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

    let server = HttpServer::new(|| {
        App::new().service(metrics::serve_metrics).wrap(Logger::default())
    }).bind(("127.0.0.1", 2489)).unwrap().run();
    tokio::spawn(server);
    
    let stream = egg_mode::stream::filter()
        .track(&["Wordle"])
        .language(&["en"])
        .start(&token)
        .try_for_each(|m| {
            if let StreamMessage::Tweet(tweet) = m {
                TWEET_COUNT.inc();

                let parsed = parse(tweet.text.as_str());
                match parsed {
                    Ok(game) => {
                        info!("Parsed score for Day {}: {}/6", game.day, game.score);
                        GAME_COUNTER_VEC
                            .with_label_values(&[
                                &game.day.to_string(),
                                &game.score.to_string()
                            ])
                            .inc();
                    }
                    Err(_) => {}
                }
            } else {
                println!("{:?}", m);
            }
            futures::future::ok(())
        });

    ctrlc::set_handler(move || {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer));
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    
    if let Err(e) = stream.await {
        println!("Stream error: {}", e);
        println!("Disconnected")
    }
}
