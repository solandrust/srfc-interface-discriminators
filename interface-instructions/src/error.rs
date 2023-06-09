#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
pub enum SplInterfaceError {
    #[error("Invalid interface namespace")]
    InvalidInterfaceNamespace,
    #[error("Instruction not found")]
    InstructionNotFound,
    #[error("Missing required instruction for interface")]
    InstructionMissing,
    #[error("Instruction has incorrect number of arguments")]
    InstructionIncorrectNumberOfArgs,
    #[error("Instruction has incorrect argument type")]
    InstructionIncorrectArgType,
}
