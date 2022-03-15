mod common;
mod metrics;

#[macro_use]
extern crate log;

use actix_web::{middleware::Logger, App, HttpServer};
use egg_mode::stream::StreamMessage;
use env_logger::Env;
use futures::TryStreamExt;
use metrics::{GAME_COUNTER_VEC, HARD_MODE, TWEET_COUNT};
use parser::{GuessChar, Wordle};
use prometheus::{Encoder, TextEncoder};
use std::env::var;

use crate::metrics::DARK_MODE;

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
		App::new()
			.service(metrics::serve_metrics)
			.wrap(Logger::default())
	})
	.bind(("0.0.0.0", 2489))
	.unwrap()
	.run();
	tokio::spawn(server);

	let stream = egg_mode::stream::filter()
		.track(&["Wordle"])
		.language(&["en"])
		.start(&token)
		.try_for_each(|m| {
			if let StreamMessage::Tweet(tweet) = m {
				TWEET_COUNT.inc();

				let parsed = Wordle::try_from(tweet.text.as_str());
				match parsed {
					Ok(game) => {
						info!("Parsed score for Day {}: {}/6", game.day, game.score);

						if game.hard {
							info!("Game was in hard mode");
							HARD_MODE.with_label_values(&[&game.day.to_string()]).inc();
						}

						if game.guesses[0].contains(&GuessChar::Black) {
							info!("User is in dark mode!");
							DARK_MODE.with_label_values(&[&game.day.to_string()]).inc();
						}

						GAME_COUNTER_VEC
							.with_label_values(&[&game.day.to_string(), &game.score.to_string()])
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
