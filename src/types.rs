use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ServerState {
    x: clients_connected,
}

impl Entity {
    pub fn to_json(&self) -> String {
        format!("{{\"position\":{{\"x\":{},\"y\":{}}}, \"id\":{}}}", self.pos.0, self.pos.1, self.id)
    }
}

pub type Entities = HashMap<u32,Entity>;