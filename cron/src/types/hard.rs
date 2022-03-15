#[derive(Debug, Serialize, Deserialize)]
pub struct FetchHardRespose {
    pub status: String,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "resultType")]
    pub result_type: String,
    pub result: Vec<MetricResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricResult {
    pub metric: Metric,
    pub value: (f64, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    #[serde(rename = "__name__")]
    pub name: String,
    pub game: String,
    pub instance: String,
    pub job: String,
}
