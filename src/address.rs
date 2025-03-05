use std::collections::HashMap;

use crate::{address_parser::AddressParser, global_state::global_state};

pub fn parse_address_str(addr: &str) -> eyre::Result<usize> {
    let eval = AddressParser::new(addr).parse()?;
    let mut env = HashMap::new();
    let v = eval.eval(&mut env, &*global_state().memory)?;
    Ok(v as _)
}
