use chrono::{DateTime, Utc};
use fake::{faker::{internet::en::Username, name::en::{FirstName, LastName}}, Fake};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = sqlx::postgres::PgPool::connect("postgres://tester:tester@localhost:5432/tester").await.unwrap();
    // select query
    let result = sqlx::query_as::<_, Profile>("select * from profile where id = $1")
        .bind(2)
        .fetch_one(&conn)
        .await;
    println!("{:?}", result.unwrap());
    // insert query
    let result2 = sqlx::query_as::<_, EntityId>("insert into message (user_id, body, likes) values ($1, $2, $3) returning id")
        .bind(2)
        .bind("hello world")
        .bind(10)
        .fetch_one(&conn)
        .await;
    println!("{:?}", result2.unwrap());
    // query with transaction
    Ok(())
}

#[derive(FromRow, Deserialize, Serialize, Debug)]
struct EntityId {
    id: i64,
}

#[derive(FromRow, Deserialize, Serialize, Debug)]
struct Profile {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
}