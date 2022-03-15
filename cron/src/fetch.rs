use anyhow::Result;
use reqwest::Client;
use std::{collections::HashMap};
use std::env::var;
use crate::media::upload_image;
use std::io::copy;
use std::fs::File;
use crate::types::{
	hard::FetchHardRespose,
	scores::FetchScoresResponse,
	mode::FetchModeResponse,
};

pub async fn generic_get(q: String, client: &Client) -> Result<reqwest::Response> {
	let mut query: HashMap<&str, &str> = HashMap::new();
    query.insert("query", &q);

    let url = format!(
        "https://{}/api/datasources/proxy/1/api/v1/query",
        var("GRAFANA_DOMAIN").unwrap()
    );

    let res =  client
        .get(&url)
        .query(&query)
        .header(
            "Authorization",
            format!("Bearer {}", var("GRAFANA_BEARER_TOKEN").unwrap()),
        )
        .send()
        .await?;

	return Ok(res)
}


pub async fn fetch_scores(game: &u16, client: &Client) -> Result<FetchScoresResponse> {
	let q = format!("wordle_stats_games{{ game= \"{}\" }}", game);
	let res = generic_get(q, client).await?;
	let json: FetchScoresResponse = res.json().await?;

	return Ok(json)
}

pub async fn fetch_hard_mode(game: &u16, client: &Client) -> Result<FetchHardRespose> {
	let q = format!("wordle_stats_hard_mode{{ game= \"{}\" }}", game);
	let res = generic_get(q, client).await?;
	let json: FetchHardRespose = res.json().await?;

	return Ok(json)
}

pub async fn fetch_dark_mode(game: &u16, client: &Client) -> Result<FetchModeResponse> {
	let q = format!("wordle_stats_dark_mode{{ game= \"{}\" }}", game);
	let res = generic_get(q, client).await?;
	let json: FetchModeResponse = res.json().await?;

	return Ok(json)
}

pub async fn fetch_panel_image(game: &u16, now: &i64, token: &egg_mode::Token, client: &Client) -> Result<egg_mode::media::MediaId> {
	let mut query: HashMap<&str, &str> = HashMap::new();
	query.insert("orgId", "1");
	let g = &game.to_string();
	query.insert("var-game", g);
	query.insert("var-instance", "localhost:2489");

	// remove exactly 24 hours from now in milliseconds
	let yesterday = (now - (24 * 60 * 60 * 1000)).to_string();
	let now = now.to_string();
	query.insert("from", &yesterday);
	query.insert("to", &now);

	query.insert("panelId", "17");
	query.insert("width", "1000");
	query.insert("height", "500");
	query.insert("tz", "America/Denver");

	let url = format!(
		"https://{}/render/d-solo/v9UbITYnz/wordle-statistics",
		var("GRAFANA_DOMAIN").unwrap()
	);
	println!("{}", url);

	let req =  client
		.get(&url)
		.query(&query)
		.header(
			"Authorization",
			format!("Bearer {}", var("GRAFANA_BEARER_TOKEN").unwrap()),
		);
	println!("{:?}", req);
	let res = req.send().await?;
	println!("{:?}", res);
	let data = res.text().await?;

	let mut dest = File::create("./image.png")?;
    copy(&mut data.as_bytes(), &mut dest)?;

	// take the response and upload it to twitter with media::upload_media
	let upload = upload_image(data.as_bytes(), token).await?;

	return Ok(upload)
}