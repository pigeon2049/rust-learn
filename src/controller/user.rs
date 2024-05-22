use poem::{ handler, web::Path};
use poem::Response;
use mysql::*;
use mysql::prelude::*;
use poem::http::StatusCode;
use crate::db::connection::POOL;
use crate::entity::example_user::ExampleUser;

#[handler]
pub(crate) async fn show_user(Path(id): Path<i64>) -> Response  {

    let mut conn = match POOL.get_conn() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to get connection from pool: {:?}", err);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to get connection from pool")
        }
    };

    // 条件查询，查询单个数据
    let res = conn.query_first(format!("{}{}", "select * from example_user where user_id = ", id) )
        .map(
            // Unpack Result
            |row| {
                row.map(|(user_id, user_name, nick_name)| ExampleUser {
                    user_id,
                    user_name,
                    nick_name
                })
            },
        );


    // 使用 match 处理查询结果
    match res {
        Ok(Some(user)) => {
            // 查询成功，构造 JSON 响应返回数据
            // 查询成功，将整个用户对象作为 JSON 数据返回
            let json_data = serde_json::to_string(&user).unwrap();

            // 构造并返回 Response
            Response::builder()
                .header("Content-Type", "application/json")
                .body(json_data)

        }
        _ => {
            // 查询失败或未找到用户，返回 404 响应
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("User not found")


        }
    }

}

