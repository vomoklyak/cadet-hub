use crate::cadet_hub_common_prelude::*;
use crate::error::CadetHubError;
use crate::util::string_util;
use crate::validator::validator::not_blank_str;
use crate::CadetHubResult;
use std::collections::HashSet;
use std::str::FromStr;

pub const USER_STRUCT_NAME: &str = "User";

#[derive(
    Default, Debug, Getters, Setters, Builder, Serialize, Deserialize, Validate, Clone, PartialEq,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct User {
    id: Option<i64>,
    #[validate(custom(function = "not_blank_str"))]
    login: String,
    #[validate(custom(function = "not_blank_str"))]
    password: String,
    role: UserRole,
}

impl User {
    pub fn normalize(&mut self) {
        self.set_login(string_util::lowercase(self.login()));
    }

    pub fn require_id(&self) -> i64 {
        self.id.expect("id missing")
    }

    pub fn has_id(&self, id: i64) -> bool {
        self.require_id() == id
    }

    pub fn has_admin_role(&self) -> bool {
        self.role.eq(&UserRole::Admin)
    }

    pub fn has_administrate_permission(&self) -> bool {
        self.role
            .permissions()
            .contains(&UserRolePermission::Administrate)
    }

    pub fn has_read_permission(&self) -> bool {
        self.role.permissions().contains(&UserRolePermission::Read)
    }

    pub fn has_write_permission(&self) -> bool {
        self.role.permissions().contains(&UserRolePermission::Write)
    }

    pub fn root_admin(&self) -> bool {
        self.login.eq("admin")
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserRole {
    #[default]
    Reader,
    Writer,
    Admin,
}

impl UserRole {
    pub fn to_str(&self) -> String {
        format!("{:?}", self)
    }

    pub fn permissions(&self) -> HashSet<UserRolePermission> {
        match self {
            UserRole::Reader => HashSet::from([UserRolePermission::Read]),
            UserRole::Writer => {
                HashSet::from([UserRolePermission::Read, UserRolePermission::Write])
            }
            UserRole::Admin => HashSet::from([
                UserRolePermission::Read,
                UserRolePermission::Write,
                UserRolePermission::Administrate,
            ]),
        }
    }

    pub fn names() -> Vec<String> {
        vec![
            Self::Reader.to_str(),
            Self::Writer.to_str(),
            Self::Admin.to_str(),
        ]
    }
}

impl FromStr for UserRole {
    type Err = CadetHubError;

    fn from_str(name: &str) -> CadetHubResult<Self> {
        match name {
            "Admin" => Ok(Self::Admin),
            "Reader" => Ok(Self::Reader),
            "Writer" => Ok(Self::Writer),
            _ => Err(CadetHubError::general_error_with_context(format!(
                "Invalid user role name: '{name}'"
            ))),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum UserRolePermission {
    #[default]
    Read,
    Write,
    Administrate,
}
