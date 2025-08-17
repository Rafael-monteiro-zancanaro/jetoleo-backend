use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PageParams {
    pub page: Option<u32>,
    pub size: Option<usize>,
}
