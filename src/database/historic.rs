// use chrono::{Local, DateTime};

// use indexmap::IndexMap;

// #[derive(Serialize)]
// pub struct HistoryStateProbe {
//     pub label: String,
//     pub nodes: IndexMap<String, HistoryStateNode>,
// }

// #[derive(Serialize)]
// pub struct HistoryStateNode {
//     pub label: String,
//     pub day: IndexMap<String, HistoryStateDay>,
// }

// #[derive(Serialize)]
// pub struct HistoryStateDay {
//     pub number: u32,
//     pub status: u32,
//     pub outages: IndexMap<String,  >,
// }

// #[derive(Serialize)]
// pub struct HistoryStateOutages {
//     pub label: String,
//     pub noticedate: Vec<DateTime<Local>>,
// }