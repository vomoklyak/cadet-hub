use crate::repository::entity::UserEntity;
use common::model::{User, UserBuilder, UserRole};
use std::str::FromStr;

impl From<UserEntity> for User {
    fn from(value: UserEntity) -> Self {
        UserBuilder::default()
            .id(value.id().clone())
            .login(value.login().clone())
            .password(value.password().clone())
            .role(UserRole::from_str(value.role()).unwrap_or_default())
            .build()
            .expect("failed build User")
    }
}
