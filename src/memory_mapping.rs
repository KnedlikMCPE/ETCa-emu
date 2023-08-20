use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MappingMode {
    Buffer,
    Indirect
}

#[derive(Serialize, Deserialize)]
pub struct MappingTableElement {
    mapping_mode: MappingMode,
    data: Vec<u8>,
    address_start: usize,
    address_end: usize
}

#[derive(Serialize, Deserialize)]
pub struct MappingTable {
    elements: Vec<MappingTableElement>
}

pub trait MemoryMapping {
    fn initialize(start: usize) -> usize;
    fn table() -> MappingTable;
}