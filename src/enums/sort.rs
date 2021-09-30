
use serde::{Deserialize, Serialize};
// default sort order by year

#[derive(Deserialize, Serialize, Debug)]
pub enum Sort {
    None, Asc, Desc
}
