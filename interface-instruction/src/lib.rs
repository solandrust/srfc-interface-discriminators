extern crate self as spl_interface_instruction;

pub use spl_interface_instruction_derive::interface_instruction;

use solana_program::entrypoint::ProgramResult;

pub trait InterfaceInstruction {
    const NAMESPACE: &'static str;
    fn required_args() -> Vec<syn::Type>;
}

pub trait Interface<I>
where
    I: InterfaceInstruction,
{
    ///
    const NAMESPACE: &'static str;
    ///
    fn instruction_discriminator(instruction: I) -> [u8; 8] {
        let disc = [0u8; 8];
        disc.copy_from_slice(
            &solana_program::hash::hash(
                (Self::NAMESPACE.to_string() + "::" + I::NAMESPACE).as_bytes(),
            )
            .to_bytes()[..8],
        );
        disc
    }
    ///
    fn process(&self) -> ProgramResult;
}

pub struct IToken {
    mint_to: T,
    transfer: T,
    burn: T,
}
