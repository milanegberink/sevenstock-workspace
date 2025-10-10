use std::collections::{HashMap, HashSet};

use strum_macros::{Display, EnumString};

pub type Permissions = HashMap<Resource, HashSet<Action>>;

#[derive(EnumString, Clone, Display)]
pub enum Resource {
    #[strum(serialize = "products")]
    Products,
    #[strum(serialize = "orders")]
    Orders,
}

#[allow(non_camel_case_types)]
#[derive(EnumString, Clone, Display)]
pub enum Action {
    #[strum(serialize = "read")]
    READ,
    #[strum(serialize = "write")]
    WRITE,
    #[strum(serialize = "delete")]
    DELETE,
    #[strum(serialize = "create")]
    CREATE,
    #[strum(serialize = "*")]
    ALL,
}
