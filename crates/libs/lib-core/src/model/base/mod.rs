mod crud_fns;
mod macro_utils;
mod utils;

pub use crud_fns::*;
use modql::SIden;
pub use utils::*;

const LIST_LIMIT_DEFAULT: i64 = 100;
const LIST_LIMIT_MAX: i64 = 250;

use sea_query::{Iden, IntoIden, TableRef};

#[derive(Iden)]
pub enum CommonIden {
    Id,
    OwnerId,
}

#[derive(Iden)]
pub enum TimestampIden {
    Cid,
    Ctime,
    Mid,
    Mtime,
}

pub trait DbBmc {
    const TABLE: &'static str;

    fn table_ref() -> TableRef {
        TableRef::Table(SIden(Self::TABLE).into_iden())
    }

    fn has_timestamps() -> bool {
        true
    }

    fn has_owner_id() -> bool {
        false
    }
}
