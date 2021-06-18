use diesel::prelude::*;
use sample_rust::schema::users as users_schema;
use sample_rust::utils::establish_connection;

fn main() {
  let connection = establish_connection();
  // 主キーが2のレコードについてカラムの名前を更新
  diesel::update(users_schema::dsl::users.find(2))
  .set(users_schema::name.eq("update_Shokora"))
  .execute(&connection)
  .expect("Error updating users");
}