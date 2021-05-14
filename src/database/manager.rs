use postgres::{Client, NoTls, Error};
// use postgres:::row::Row;
// use chrono::format::ParseError;
use chrono::{Local, DateTime, Datelike};
use crate::prober::status::Status;
use crate::prober::states::{ServiceStatesProbeNode, HistoryDaysOutages };
// use crate::prober::status::Status;
// use chrono::{Duration, DateTime, Utc, Local, TimeZone};
// use std::collections::HashMap;
// use std::env;

// const AGGREGATE_INTERVAL_SECONDS: u64 = 10;

// struct BumpedStates {
//     status: Status,
//     replicas: Vec<String>,
//     changed: bool,
//     startup: bool,
// }
struct HistoryStates {
    status: i32,
    probe_id: String,
    node_id: String,
    time: String,
    dayofyear: String,
}
// let db = env::var("DB_URL")
pub fn init_db() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:example@localhost:5432/postgres", NoTls)?;
    info!("Initialisation de la base de données postgres");
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS historic (
            id           SERIAL PRIMARY KEY,
            probe_id     VARCHAR NOT NULL,
            node_id      VARCHAR NOT NULL,
            status       INT NOT NULL,
            noticedate   VARCHAR NOT NULL, 
            dayofyear    VARCHAR NOT NULL
            )
    ")?;
    
    Ok(())

}
pub fn insert_in_db(
    my_probe_id: String, 
    my_node_id: String, 
    my_status: i32
   ) -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:example@localhost:5432/postgres", NoTls)?;
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS historic (
            id           SERIAL PRIMARY KEY,
            probe_id     VARCHAR NOT NULL,
            node_id      VARCHAR NOT NULL,
            status       INT NOT NULL,
            noticedate   VARCHAR NOT NULL, 
            dayofyear    VARCHAR NOT NULL
            )
    ")?;
    info!("datastoring in postgres");
    
    let local: DateTime<Local> = Local::now();
    let my_time = local.to_string();
    let dy = Datelike::ordinal(&local).to_string();
    info!("Day of year: {}", dy);
    // .format("%Y-%m-%d %H:%M:%S");
    let probe = HistoryStates {
        status: my_status ,
        probe_id: my_probe_id.to_string(),
        node_id: my_node_id.to_string(),
        time: my_time,
        dayofyear: dy,
    };
    let rows_updated = client.execute(
                "INSERT INTO historic (probe_id, node_id, status, noticedate, dayofyear) VALUES ($1, $2, $3, $4, $5)",
                &[
                    &probe.probe_id,
                    &probe.node_id,
                    &probe.status,
                    &probe.time,
                    &probe.dayofyear
                ],
        )?;
    println!("{:?} rows updated", rows_updated);

    Ok(())

}
pub fn get_days() -> (u32, u32) {
    let local: DateTime<Local> = Local::now();
    let b = Datelike::ordinal(&local);
    let a = b - 89;
    return (a, b);
}
// pub fn get_outages(dayofyear: String, node_id: String) -> (Vec<String>, Status) {
//     let mut outage: Vec<String> = Vec::new();
//     let mut status: Status;
//     let rs = outages(dayofyear, node_id, &mut outage, &mut status);
//     println!("Outages results: {:?}", rs);
//     return (outage, status);
// }

pub fn get_outages(dayofyear: String, node_id: String, find: &mut bool, probe_node: &mut ServiceStatesProbeNode, n: u32) -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:example@localhost:5432/postgres", NoTls)?;
    let mut outages: Vec<String> = Vec::new();
    // let dy = dayofyear.to_string();
    let my_status: Status;
    let my_rows = client.query(
        "SELECT noticedate, status FROM historic WHERE node_id = $1 AND dayofyear = $2",
        &[
            &node_id,
            &dayofyear
        ],
    )?;
    *find = my_rows.is_empty();
    if !my_rows.is_empty() {
        let mut code = 1;
        for row in my_rows {
            let my_s:i32 = row.get(1);
            let my_not = row.get(0);
            if my_s == 2 {
                code = my_s;
            }
            outages.push(my_not);
        }
        my_status = match code {
            0 => Status::Healthy,
            1 => Status::Sick,
            _ => Status::Dead,          
        };
        
    } else {
        my_status = Status::Healthy;   
    }
    // noticedates = &mut outages;
    // let mut status = Status::Healthy;
    // return (outages, status);
    probe_node.days.insert(
            n.to_string(),
            HistoryDaysOutages {
                status: my_status,
                daynum: n,
                noticedate: outages ,
            },   
        );
    Ok(())

}