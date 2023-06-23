use futures::StreamExt;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgPool;
use futures::TryStreamExt;

use swpc_psql::{
    swpc::{filtered_solar_wind_data, payload_to_solarwind, solar_wind_payload, SolarWind}
};

async fn insert_solar_wind(pool: &PgPool, solar_wind: Vec<SolarWind>) -> Result<(), sqlx::Error> {
    for sw in solar_wind {
        sqlx::query(
            "INSERT INTO solarwind (time_stamp, time_tag, speed, density, temperature, bt, bz) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(sw.timestamp)
        .bind(sw.time_tag)
        .bind(sw.speed)
        .bind(sw.density)
        .bind(sw.temperature)
        .bind(sw.bt)
        .bind(sw.bz)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn max_solar_wind_timestamp(pool: &Pool<Postgres>, table: String) -> i64 {
    let mut max_timestamp = 0;
    let mut conn = pool.acquire().await.unwrap();
    let query = format!("SELECT COALESCE(MAX(time_stamp), 1682916954) FROM {}", table);
    let mut rows = sqlx::query(&query)
        .fetch(&mut conn);
    while let Some(row) = rows.next().await {
        let row = row.unwrap();
        max_timestamp = row.get(0);
    }
    max_timestamp
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let database_url = std::env::var("SWPC_PGDB").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let timestamp = max_solar_wind_timestamp(&pool, "solarwind".to_string()).await;

    let solar_wind = filtered_solar_wind_data(timestamp, payload_to_solarwind(solar_wind_payload().await)).await;

    if solar_wind.len() > 0 {

        insert_solar_wind(&pool, solar_wind).await?;

    }

    Ok(())
}


