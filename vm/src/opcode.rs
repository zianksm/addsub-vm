#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    PUSH(u128),
    ADD,
    SUB,
    POP,
}
