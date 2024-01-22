use sqlx::mysql::{ MySqlPoolOptions};
use time::OffsetDateTime;
#[derive(Debug,  sqlx::FromRow)]
struct User {
    id: i32,
    username : String,
    createTime:OffsetDateTime,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new().connect("mysql://naive_admin:naive_admin_pass@localhost:33069/naive_admin").await?;


  let rows = sqlx::query_as::<_, User>("SELECT * FROM user")
        .fetch_all(&pool)
        .await?;

    println!("{:#?}", rows);




//
//      test ok
//     let pool = MySqlPoolOptions::new().connect("mysql://naive_admin:naive_admin_pass@localhost:33069/naive_admin").await?;
//
//
//      let rows = sqlx::query!("select * from user") .fetch_all(&pool).await?;
//      println!("{:#?}", rows);

    Ok(())
}




