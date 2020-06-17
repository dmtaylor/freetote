use crate::models::{User, NewUser};
use crate::db::FTDB;
use crate::models::*;
use diesel::prelude::*;

impl FTDB {
    pub fn new_user<'a> (
        &self,
        name: &'a str,
        desc: Option<&'a str>
    ) -> QueryResult<User>
    {
        use crate::schema::users;
        let new_user = NewUser {
            username: name,
            user_description: desc,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&self.conn)
    }

    pub fn find_by_username(&self, name: &str) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users.filter(username.eq(name)).first(&self.conn)
    }

}
