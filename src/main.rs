mod tools;

use time::OffsetDateTime;
use std::clone::Clone;
use sqlx::mysql::{ MySqlPoolOptions,MySqlPool,MySqlQueryResult};

#[derive(Debug, Clone, sqlx::FromRow)]
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

    let update_result = fetch_user_by_id(&pool,88).await?;

   println!("update result: {:?}", update_result);
      Ok(())
// let user = User {
//         username: "John10".to_string(),
//         password: tools::md5_crypto("123456".to_string()),
//         enable:1,
//         createTime: OffsetDateTime::now_utc(),
//         updateTime: OffsetDateTime::now_utc(),
//     };
//     let update_result = add_user_by_struct(&pool,user.clone()).await?;


//     let users = fetch_all_users(&pool).await;
//     println!("{:#?}", users);
//      Ok(())
//     println!("1111111");

//// ope MySqlTransaction ok
//     let mut tx = pool.begin().await?;
//
//     let result = sqlx::query("delete from user where id = ?")
//             .bind(6)
//             .execute(&mut tx)
//             .await;
//     println!("delete result {:?}", result);
//     // delete result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })
//
//     let result = sqlx::query("update user set username = ? where id = ?")
//             .bind("John34".to_string())
//             .bind(2)
//             .execute(&mut tx).await;
//     println!("update result {:?}", result);
//     //// update result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })
//     tx.commit().await?;
// //     tx.rollback().await?;
//
// ////  ope  update ok
//     let result = sqlx::query("update user set username = ? where id = ?")
//                 .bind("John34".to_string())
//                 .bind(2)
//                 .execute(&pool).await;
//         println!("update result {:?}", result);
        //// update result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })
//
// //// ope delete ok
//     let result = sqlx::query("delete from user where id = ?").bind(6).execute(&pool).await;
//     println!("delete result {:?}", result);
//     // delete result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })


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


}

async fn fetch_user_by_id(pool: &MySqlPool, id: i64) -> Result<User, sqlx::Error> {
    let result = sqlx::query_as::<_, User>("SELECT * FROM user where id = ?")
            .bind(id)
            .fetch_one(pool)
            .await?;
    println!("{:#?}", result);
    Ok(result)
}
// async fn fetch_user_by_id(pool: &MySqlPool, id: i64) -> Result<Option<User>, sqlx::Error> {
//       let result = sqlx::query_as::<_, User>("SELECT * FROM user where id = ?")
//                 .bind(id)
//                 .fetch_one(pool)
//                 .await?;
//       match result {
//           Some(row) => Ok(row.to_user()),
//           None => Ok(()),
//       }
// }

async fn fetch_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let rows = sqlx::query_as::<_, User>("SELECT * FROM user")
            .fetch_all(pool)
            .await?;
    Ok(rows.into_iter().map(|row| row.clone()).collect())
}


async fn update_username_by_id(pool: &MySqlPool, username: &str, id: i64) -> Result<MySqlQueryResult, sqlx::Error> {
    let  result = sqlx::query("update user set username = ? where id = ?")
            .bind(username)
            .bind(id)
            .execute(pool)
            .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}
async fn delete_user_by_id(pool: &MySqlPool,  id: i64) -> Result<(),sqlx::Error> {
     let result = sqlx::query("delete from user where id = ?")
            .bind(id)
            .execute(pool)
            .await;
     Ok(())
     // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}

async fn add_user_by_struct(pool: &MySqlPool,  data: User) -> Result< MySqlQueryResult,sqlx::Error> {
    let insert_sql = "INSERT INTO user (username, password, enable, createTime, updateTime) VALUES (?, ?, ?, ?, ?)";
    let result = sqlx::query(&insert_sql)
        .bind(&data.username)
        .bind(&data.password)
        .bind(&data.enable)
        .bind(&data.createTime)
        .bind(&data.updateTime)
        .execute(pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}