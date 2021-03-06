use diesel::prelude::*;
use crate::schema::users;
use crate::utils::establish_connection;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    name: &'a str,
}
impl User {
  pub fn all() -> Vec<User> {
      let connection = establish_connection();
      users::dsl::users
          .limit(30)
          .load::<User>(&connection)
          .expect("Error loading users")
  }

  pub fn create(name: &str) -> User {
      use self::users::id;
      let new_user = NewUser { name: name };
      let connection = establish_connection();
      diesel::insert_into(users::table)
          .values(&new_user)
          .execute(&connection)
          .expect("Error saving new user");
      users::dsl::users
          .order(id.desc())
          .first::<User>(&connection)
          .expect("Error finding users")
  }

}