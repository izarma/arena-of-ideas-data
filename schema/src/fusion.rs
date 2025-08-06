use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default, Hash)]
pub struct UnitActionRange {
    pub trigger: u8,
    pub start: u8,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UnitTriggerRef {
    pub unit: u64,
    pub trigger: u8,
}
