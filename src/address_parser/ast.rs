use std::collections::HashMap;

use eyre::ContextCompat;

use super::{AddressParserResult, AddressResolver};

#[derive(Debug)]
pub enum Node {
    Number(isize),
    ModuleSymbol(String),
    Dereference(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Sin(Box<Node>),
    Cos(Box<Node>),
    Sqrt(Box<Node>),
    Var(String),
    Assignment(String, Box<Node>),
}

impl Node {
    pub fn eval(
        &self,
        env: &mut HashMap<String, isize>,
        resolver: &dyn AddressResolver,
    ) -> AddressParserResult<isize> {
        match self {
            Node::Number(a) => Ok(*a),
            Node::ModuleSymbol(a) => resolver
                .module_symbol_to_address(a.as_str())
                .context(obfstring!("Failed load module symbol: ") + a),
            Node::Dereference(node) => {
                let addr = node.eval(env, resolver)?;
                if addr <= 0 {
                    eyre::bail!("{}{addr:#X}", obfstr!("Cant dereference negative ptr: "))
                }
                resolver.dereference(addr as usize).context(
                    obfstring!("Failed dereference address: ") + format!("{addr:#X}").as_str(),
                )
            }
            Node::Add(node, node1) => Ok(node
                .eval(env, resolver)?
                .saturating_add(node1.eval(env, resolver)?)),
            Node::Sub(node, node1) => Ok(node
                .eval(env, resolver)?
                .saturating_sub(node1.eval(env, resolver)?)),
            Node::Mul(node, node1) => Ok(node
                .eval(env, resolver)?
                .saturating_mul(node1.eval(env, resolver)?)),
            Node::Div(node, node1) => Ok(node
                .eval(env, resolver)?
                .saturating_div(node1.eval(env, resolver)?)),
            Node::Pow(node, node1) => Ok(node
                .eval(env, resolver)?
                .saturating_pow(node1.eval(env, resolver)?.try_into()?)),
            Node::Sin(node) => Ok(f64::sin(node.eval(env, resolver)? as f64).round() as isize),
            Node::Cos(node) => Ok(f64::cos(node.eval(env, resolver)? as f64).round() as isize),
            Node::Sqrt(node) => Ok(f64::sqrt(node.eval(env, resolver)? as f64).round() as isize),
            Node::Var(a) => Ok(env
                .get(a)
                .copied()
                .context(obfstring!("Failed get variable: ") + a)?),
            Node::Assignment(a, node) => {
                let value = node.eval(env, resolver)?;
                env.insert(a.to_owned(), value);
                Ok(value)
            }
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
