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


//// ope MySqlTransaction ok
    let mut tx = pool.begin().await?;

    let result = sqlx::query("delete from user where id = ?")
            .bind(6)
            .execute(&mut tx)
            .await;
    println!("delete result {:?}", result);
    // delete result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })

    let result = sqlx::query("update user set username = ? where id = ?")
            .bind("John34".to_string())
            .bind(2)
            .execute(&mut tx).await;
    println!("update result {:?}", result);
    //// update result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })
    tx.commit().await?;
//     tx.rollback().await?;

////  ope  update ok
    let result = sqlx::query("update user set username = ? where id = ?")
                .bind("John34".to_string())
                .bind(2)
                .execute(&pool).await;
        println!("update result {:?}", result);
        //// update result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })

//// ope delete ok
    let result = sqlx::query("delete from user where id = ?").bind(6).execute(&pool).await;
    println!("delete result {:?}", result);
    // delete result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })


////   ope  insert ok
//     let user = User {
//         username: "John3".to_string(),
//         password: tools::md5_crypto("123456".to_string()),
//         enable:1,
//         createTime: OffsetDateTime::now_utc(),
//         updateTime: OffsetDateTime::now_utc(),
//     };
//     let insert_sql = "INSERT INTO user (username, password, enable, createTime, updateTime) VALUES (?, ?, ?, ?, ?)";
//
//     let result = sqlx::query(&insert_sql)
//         .bind(&user.username)
//         .bind(&user.password)
//         .bind(&user.enable)
//         .bind(&user.createTime)
//         .bind(&user.updateTime)
//         .execute(&pool)
//         .await?;
//
//     println!("{:?}", result);
        //// MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }


////   ope  query ok
//     let rows = sqlx::query_as::<_, User>("SELECT * FROM user").fetch_all(&pool).await?;
//     println!("{:#?}", rows);

    Ok(())
}


