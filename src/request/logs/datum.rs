use std::rc::Rc;

use serde::{Deserialize, Serialize};

use super::role::Role;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Datum {
    roles: Role,
}

impl Datum {
    pub fn get_roles(&self) -> &Role {
        &self.roles
    }
}
