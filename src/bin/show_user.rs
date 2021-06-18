use diesel::prelude::*;
use sample_rust::models::User;
use sample_rust::schema::users as users_schema;
use sample_rust::utils::establish_connection;

fn main() {
    // utils.rsのestablish_connection関数からDBとの接続インスタンスを取得
    let connection = establish_connection();

    // ユーザーテーブルを指定し、その先頭レコードをUser型で返す
    let user = users_schema::dsl::users
        .first::<User>(&connection)
        .expect("Error loading users");

        println!("{:?}", user)
}
