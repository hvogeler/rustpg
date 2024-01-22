
use std::env;
use sqlx::{postgres::PgPoolOptions, types::time::{PrimitiveDateTime, Date, Time}};

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect("postgres://dec:dec@L15-50:5434/dec").await?;

    // let rows = sqlx::query(r#"SELECT request_id, entity_schema, agent_name FROM request"#).fetch_all(&pool).await?;
    // rows.iter().for_each(|r| {
    //     println!("{:?}", r.get::<&str, _>("agent_name"));
    // });

    let requests: Vec<Request> = 
    sqlx::query!(
        r#"SELECT request_id, entity_schema, agent_name, ts_in 
          FROM request
          WHERE ts_in < $1
          ORDER BY ts_in ASC"#,
          PrimitiveDateTime::new(Date::from_calendar_date(2024, time::Month::January, 17).unwrap(), Time::from_hms(11, 17, 12).unwrap()))
    .fetch_all(&pool)
    .await?
    .iter()
    .map(|r| Request {
        request_id: r.request_id.clone(),
        entity_schema: r.entity_schema.clone(),
        agent_name: r.agent_name.clone(),
        ts_in: r.ts_in,
    })
    .collect();
    
    requests.iter().for_each(|request| log::info!("{:?}", request));

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Request {
    pub request_id: String,
    pub entity_schema: String,
    pub agent_name: String,
    pub ts_in: PrimitiveDateTime,
}

// request_id VARCHAR(36) NOT NULL,
// entity_schema VARCHAR(255) NOT NULL,
// agent_name VARCHAR(255) NOT NULL,
// agent_version INT NOT NULL,
// entity_id VARCHAR(255) NOT NULL,
// choice_cnt INT NOT NULL,
// -- source of the request
// src VARCHAR(64),
// ts_in TIMESTAMP NOT NULL,
// ts_out TIMESTAMP,
// request_params JSONB NOT NULL,