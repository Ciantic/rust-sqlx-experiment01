use chrono::NaiveDateTime;
use sqlx::prelude::*;
use sqlx::{Decode, error::BoxDynError, migrate::Migrator, sqlite::{SqlitePoolOptions, SqliteTypeInfo}};
use std::fs::File;
use serde::{Serialize, Deserialize};
// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[derive(
    PartialEq, Eq, Hash, Debug, Serialize, Deserialize
)]
pub struct ArticleId(pub uuid::Uuid);

impl From<String> for ArticleId {
    fn from(s: String) -> Self {
        ArticleId(uuid::Uuid::parse_str(&s).unwrap())
    }
}

impl sqlx::Type<sqlx::Sqlite> for ArticleId {
    fn type_info() -> SqliteTypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

impl<'de> Decode<'de, sqlx::Sqlite> for ArticleId {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'de>) -> Result<Self, BoxDynError> {
        let mut blob = <String as Decode<sqlx::Sqlite>>::decode(value)?;
        Ok(ArticleId(uuid::Uuid::parse_str(&blob).unwrap()))
    }
}
impl<'q> Encode<'q, sqlx::Sqlite> for ArticleId {
    fn encode_by_ref(&self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> sqlx::encode::IsNull {
        todo!()
    }
}
#[derive(
    Debug, Serialize, Deserialize
)]
struct Article {
    pub id: ArticleId,
    pub hash: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub modified_on_disk: NaiveDateTime,
    pub local_path: String,
    pub server_path: String,
    pub title: String,
    pub html: String,
}


#[async_std::main]
// or #[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    // let f = File::create("./.cache.db").unwrap();
    // f.set_len(0).unwrap();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://./.cache.db").await?; 
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    // ERROR: expected struct `ArticleId`, found struct `std::string::String`
    let row = sqlx::query_as!(Article, r#"select * from articles"#).fetch_one(&pool).await?;

    println!("row: {:?}", row.id);

    // // Make a simple query to return the given parameter
    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&pool).await?;

    // assert_eq!(row.0, 150);

    Ok(())
}