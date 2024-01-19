use diesel::{Queryable,Insertable};
use crate::db::QueryResult;
// 定义与数据库表对应的结构体（模型）
#[derive(Queryable, Insertable)]
#[table_name = "user"]
struct User {
    id: Option<i64>,
    name: String,
    passwd: String,
    phone: String,
}

// 插入数据
// fn insert_user() -> QueryResult<usize> {
//
//     let new_user = User {
//         id: None,
//         name: "name".to_string(),
//         passwd: "name".to_string(),
//         phone: "phone".to_string(),
//     };
//
//     diesel::insert_into(user)
//         .values(&new_user)
//         .execute(&*CONNECTION)
// }

// // 查询数据
// fn get_users() -> QueryResult<Vec<User>> {
//
//     users.load::<User>(&*CONNECTION)
// }
//
// // 更新数据
// fn update_user(id: i32) -> QueryResult<usize> {
//
//     diesel::update(users.filter(id.eq(id)))
//         .set(name.eq("new_name".to_string))
//         .execute(&*CONNECTION)
// }
//
// // 删除数据
// fn delete_user(id: i32) -> QueryResult<usize> {
//
//     diesel::delete(users.filter(id.eq(id)))
//         .execute(&*CONNECTION)
// }