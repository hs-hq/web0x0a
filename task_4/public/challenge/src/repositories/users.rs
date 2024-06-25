use crate::config::database_connection;
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

use super::common::DBError;

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserInsertable<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
    role: &'a str,
}

pub struct UsersRepository;

impl UsersRepository {
    pub fn get_all_users(&self) -> Result<Vec<User>, DBError> {
        let connection = &mut database_connection();
        users.load::<User>(connection)
    }
    pub fn get_user_by_id(&self, user_id: i32) -> Result<User, DBError> {
        let connection = &mut database_connection();
        users.filter(id.eq(user_id)).first::<User>(connection)
    }

    pub fn sinsert_user(&self, new_user: &UserInsertable) -> Result<User, DBError> {
        let connection = &mut database_connection();
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(connection)
    }

    pub fn pinsert_user<'a>(
        &self,
        new_username: &'a str,
        new_password: &'a str,
        new_email: &'a str,
        new_role: &'a str,
    ) -> Result<User, DBError> {
        let new_user = UserInsertable {
            username: new_username,
            password: new_password,
            email: new_email,
            role: new_role,
        };
        let connection = &mut database_connection();
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(connection)
    }

    pub fn delete_user_by_id(&self, user_id: i32) -> bool {
        let connection = &mut database_connection();
        diesel::delete(users.filter(id.eq(user_id)))
            .execute(connection)
            .is_ok()
    }

    pub fn update_user<'a>(
        &self,
        user_id: i32,
        updated_username: &'a str,
        updated_password: &'a str,
        updated_email: &'a str,
        updated_role: &'a str,
    ) -> Result<User, DBError> {
        let updated_user = UserInsertable {
            username: updated_username,
            password: updated_password,
            email: updated_email,
            role: updated_role,
        };
        let connection = &mut database_connection();
        diesel::update(users)
            .filter(id.eq(user_id))
            .set(updated_user)
            .get_result::<User>(connection)
    }

    pub fn delete_all_users(&self) -> bool {
        let connection = &mut database_connection();
        diesel::delete(users).execute(connection).is_ok()
    }

    pub fn get_user_by_username(&self, user_username: &str) -> Result<User, DBError> {
        let connection = &mut database_connection();
        users
            .filter(username.eq(user_username))
            .first::<User>(connection)
    }

    pub fn get_user_by_email(&self, user_email: &str) -> Result<User, DBError> {
        let connection = &mut database_connection();
        users.filter(email.eq(user_email)).first::<User>(connection)
    }

    pub fn delete_all_users_except_admin(&self) -> bool {
        let connection = &mut database_connection();
        diesel::delete(users)
            .filter(role.ne("admin"))
            .execute(connection)
            .map(|rows_affected| if rows_affected > 0 { true } else { false })
            .unwrap()
    }
}
