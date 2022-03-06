use actix_web::{get, HttpResponse, Responder};
use hyper::{
    header::CONTENT_TYPE
};
use lazy_static::lazy_static;
use prometheus::{
    register_counter, register_counter_vec,
    Counter, CounterVec
};
use prometheus::{Encoder, TextEncoder};

lazy_static! {
    pub static ref TWEET_COUNT: Counter = register_counter!(
        "wordle_stats_tweet_count",
        "Total amount of tweets the service has received."
    ).unwrap();

    pub static ref HARD_MODE: Counter = register_counter!(
        "wordle_stats_hard_mode",
        "Total amount of hard mode games parsed."
    ).unwrap();

    pub static ref GAME_COUNTER_VEC: CounterVec = register_counter_vec!(
        "wordle_stats_games",
        "Wordle Game Statistics.",
        &["game", "score"]
    )
    .unwrap();
}

#[get("/metrics")]
pub async fn serve_metrics() -> impl Responder {
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .append_header((CONTENT_TYPE, encoder.format_type()))
        .body(buffer)
}