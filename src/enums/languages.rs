
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum Languages {
    CHINESE
}

#[derive(Deserialize, Serialize, Debug)]
pub enum MatchMode {

    // eg. 1986年到1980年 = 2 event.
    SINGLE,
    // eg. 1986年到1980年 = 1 event.
    CONNECTED
}
