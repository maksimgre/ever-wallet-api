use std::str::FromStr;

use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;

use crate::models::*;
use crate::sqlx_client::*;

const DB_POOL_SIZE: u32 = 1;

pub async fn add_root_token(token_name: String, token_address: String) -> Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .context("The DATABASE_URL environment variable must be set")?;

    let pool = PgPoolOptions::new()
        .max_connections(DB_POOL_SIZE)
        .connect(&database_url)
        .await
        .expect("fail pg pool");

    let sqlx_client = SqlxClient::new(pool);
    let root_token = sqlx_client
        .create_root_token(TokenWhitelistFromDb {
            name: token_name,
            address: token_address,
        })
        .await?;

    println!("Root token {:?} has been added!", root_token);

    Ok(())
}

pub async fn create_api_service(service_name: String, service_id: Option<String>) -> Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .context("The DATABASE_URL environment variable must be set")?;

    let id = match service_id {
        Some(id) => ServiceId::from_str(&id)?,
        None => ServiceId::generate(),
    };

    let pool = PgPoolOptions::new()
        .max_connections(DB_POOL_SIZE)
        .connect(&database_url)
        .await
        .expect("fail pg pool");

    let sqlx_client = SqlxClient::new(pool);
    let api_service = sqlx_client.create_api_service(id, &service_name).await?;

    println!("Api service {:?} created successfully!", api_service);

    Ok(())
}

pub async fn create_api_service_key(
    service_id: String,
    service_key: String,
    service_secret: String,
) -> Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .context("The DATABASE_URL environment variable must be set")?;

    let service_id = ServiceId::from_str(&service_id)?;

    let pool = PgPoolOptions::new()
        .max_connections(DB_POOL_SIZE)
        .connect(&database_url)
        .await
        .expect("fail pg pool");

    let sqlx_client = SqlxClient::new(pool);
    let api_service_key = sqlx_client
        .create_api_service_key(service_id, &service_key, &service_secret)
        .await?;

    println!(
        "Api service key {:?} created successfully!",
        api_service_key
    );

    Ok(())
}
