use sqlx::mysql::{ MySqlPoolOptions,MySqlPool};
use time::OffsetDateTime;
mod tools;
#[derive(Debug,  sqlx::FromRow)]
struct User {
    username : String,
    password : String,
    enable : i8,
    createTime : OffsetDateTime,
    updateTime : OffsetDateTime,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new().connect("mysql://naive_admin:naive_admin_pass@localhost:33069/naive_admin").await?;

//     update

//     insert ok
    let user = User {
        username: "John2".to_string(),
        password: tools::md5_crypto("123456".to_string()),
        enable:1,
        createTime: OffsetDateTime::now_utc(),
        updateTime: OffsetDateTime::now_utc(),
    };
    let query = "INSERT INTO user (username, password, enable, createTime, updateTime) VALUES (?, ?, ?, ?, ?)";

    let result = sqlx::query(&query)
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.enable)
        .bind(&user.createTime)
        .bind(&user.updateTime)
        .execute(&pool)
        .await?;

    println!("{:?}", result);


//     query ok
//     let rows = sqlx::query_as::<_, User>("SELECT * FROM user").fetch_all(&pool).await?;
//     println!("{:#?}", rows);

    Ok(())
}


