// Phase 4 — Enums, Pattern Matching, Generics, Option/Result, Module System
// Module exports

pub mod enums;
pub mod patterns;
pub mod generics;
pub mod option_result;
pub mod modules;

pub use enums::{AdtPayload, EnumInstance, EnumTypeDef, EnumVariantDef};
pub use generics::{ConcreteType, GenericSignature, GenericSolver};
pub use modules::{ModuleNode, ModuleSymbol, ModuleSystem};
pub use option_result::{PradyOption, PradyResult};
pub use patterns::{MatchArm, Pattern, PatternEngine};
