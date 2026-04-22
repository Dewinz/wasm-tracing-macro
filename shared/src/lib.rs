use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Action {
    Add,
    Remove,
}
