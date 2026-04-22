use serde::{Deserialize, Serialize};

// TODO: Rethink naming.
#[derive(Serialize, Deserialize)]
pub enum Action {
    Add,
    Remove,
}
