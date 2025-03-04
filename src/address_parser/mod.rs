mod ast;
mod lexer;
mod parser;
mod token;

pub type AddressParserResult<T> = eyre::Result<T>;
pub use parser::AddressParser;
pub trait AddressResolver {
    fn module_symbol_to_address(&self, module_name: &str) -> Option<isize>;
    fn dereference(&self, address: usize) -> Option<isize>;
}
