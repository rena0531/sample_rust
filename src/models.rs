// ユーザーテーブルのスキーマを取得
use crate::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

#[derive(Debug, Queryable)]
// Debugトレイとは構造体のフィールド確認用に継承
// Queryableトレイとは、データの読み込みに利用するために継承
pub struct User {
  pub id: u64, //u64型は、Unsigned<Bigint>型に対応する
  pub name: String,
}