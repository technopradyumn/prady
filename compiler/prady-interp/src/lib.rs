pub mod env;
pub mod eval;
pub mod value;
pub mod dsa;

pub use env::Environment;
pub use eval::{Interpreter, RuntimeError};
pub use value::Value;
