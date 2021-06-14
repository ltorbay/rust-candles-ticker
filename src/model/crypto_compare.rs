use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Histohour {
    pub response: String,
    pub has_warning: bool,
    pub data: Histogram,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Histogram {
    pub time_from: i64,
    pub time_to: i64,
    pub data: Vec<HistoData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoData {
    pub time: i64,
    pub high: f32,
    pub low: f32,
    pub open: f32,
    pub close: f32,
    pub conversion_type: String,
    pub conversion_symbol: String,
}