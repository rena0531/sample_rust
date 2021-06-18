use diesel::prelude::*;
use sample_rust::models::NewUser;
use sample_rust::models::User;
use sample_rust::schema::users as users_schema;
use sample_rust::utils::establish_connection;

fn main() {
    // utils.rsのestablish_connection関数からDBとの接続インスタンスを取得
    let connection = establish_connection();
    // models.rsで定義したNewUser構造体のインスタンスを生成
    // 一件登録
    // let new_user = NewUser{
    //     name: String::from("Rena"),
    // };
    // 複数登録
    let new_user = vec![
        NewUser{
            name: String::from("Shokora"),
        },
        NewUser{
            name: String::from("Sora"),
        },
        NewUser{
            name: String::from("Moko"),
        },
    ];

    // インサート処理の実行　sqlのINSERT文のような感じ
    diesel::insert_into(users_schema::dsl::users)
    .values(&new_user)
    .execute(&connection)
    .expect("Error saving new user");
}