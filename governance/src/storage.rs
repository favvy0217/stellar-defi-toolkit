// fix(#3): misleading error message on insufficient funds
pub enum DataKey {
    Proposal(u64),
    TimelockDelay,
}
