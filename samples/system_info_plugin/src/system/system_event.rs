use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SystemEvent {
    MemoryUsage,
    Variant2,
}
