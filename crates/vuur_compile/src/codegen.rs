//! Interface for code generators.

pub trait Codegen {
    type Input;
    type Output;

    // TODO: Error type
    fn compile(&mut self, input: &Self::Input) -> Result<Self::Output, String>;
}
