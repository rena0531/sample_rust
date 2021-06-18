use diesel::prelude::*;
use sample_rust::schema::users as users_schema;
use sample_rust::utils::establish_connection;

fn main() {
  let connection = establish_connection();
    diesel::delete(users_schema::dsl::users.find(2))
        .execute(&connection)
        .expect("Error deleting users");
}