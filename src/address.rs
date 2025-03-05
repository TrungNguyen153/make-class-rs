use std::collections::HashMap;

use crate::{address_parser::AddressParser, global_state::global_state};

pub struct AddressString {
    addr_str: String,
    value: Option<usize>,
}

impl From<usize> for AddressString {
    fn from(value: usize) -> Self {
        Self {
            addr_str: format!("{value:#X}"),
            value: Some(value),
        }
    }
}

impl Into<usize> for AddressString {
    fn into(self) -> usize {
        self.value.unwrap_or_default()
    }
}

impl std::fmt::Display for AddressString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.addr_str)
    }
}

impl AddressString {
    pub fn load_str(addr: &str) -> eyre::Result<Self> {
        let value = parse_address_str(addr)?;
        Ok(Self {
            addr_str: addr.to_string(),
            value: Some(value),
        })
    }

    pub fn address_value(&self) -> usize {
        self.value.unwrap_or_default()
    }
}

fn parse_address_str(addr: &str) -> eyre::Result<usize> {
    let eval = AddressParser::new(addr).parse()?;
    let mut env = HashMap::new();
    let v = eval.eval(&mut env, &*global_state().memory)?;
    Ok(v as _)
}
