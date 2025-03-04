use std::collections::HashMap;

use make_class_rs::{
    self,
    address_parser::{AddressParser, AddressResolver},
};
use tracing::{Level, info};
use tracing_subscriber::fmt::format::FmtSpan;
fn main() {
    let sub = tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_ansi(true)
        .with_level(true)
        .with_max_level(Level::DEBUG)
        .with_span_events(FmtSpan::ENTER)
        .with_file(false)
        .with_target(false)
        .without_time()
        .finish();
    tracing::subscriber::set_global_default(sub).unwrap();

    info!(" Go ");
    let src = "((10 + 11) + 1) / 2";
    let mut parser = AddressParser::new(src);
    let eval = parser.parse().unwrap();

    let mut env = HashMap::new();
    struct Resolver;
    impl AddressResolver for Resolver {
        fn module_symbol_to_address(&self, module_name: &str) -> Option<isize> {
            match module_name {
                "target.dll" => Some(0x10),
                "unTarget.dll" => Some(0x20),
                _ => None,
            }
        }

        fn dereference(&self, address: usize) -> Option<isize> {
            if address == 0x10 {
                return Some(0x20);
            }
            None
        }
    }

    let resolver = Resolver;
    let ret = eval.eval(&mut env, &resolver).unwrap();
    println!("Ret={ret}");
}
