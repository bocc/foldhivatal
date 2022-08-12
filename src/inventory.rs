use std::collections::HashMap;

use crate::{contract::Contract, stakeholder::Stakeholder};

// here we store our stateful contracts
// pub struct ContractInventory<T> {
//     pub contracts: HashMap<usize, Box<dyn Contract<Message<T> = ()>>>
// }

pub struct StakeholderInventory {
    pub stakeholders: Vec<Box<dyn Stakeholder>>,
}
