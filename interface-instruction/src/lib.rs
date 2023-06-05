pub trait InterfaceInstruction {
    fn unpack(input: &[u8]) -> Result<Self, ProgramError>
    where
        Self: Sized;
    fn pack(&self, output: &mut [u8]) -> Result<(), ProgramError>;
}
