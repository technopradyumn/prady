// Phase 4 — Enums, Pattern Matching, Generics, Option/Result, Module System
// Module exports

pub mod enums;
pub mod generics;
pub mod modules;
pub mod option_result;
pub mod patterns;

pub use enums::{AdtPayload, EnumInstance, EnumTypeDef, EnumVariantDef};
pub use generics::{ConcreteType, GenericSignature, GenericSolver};
pub use modules::{ModuleNode, ModuleSymbol, ModuleSystem};
pub use option_result::{PradyOption, PradyResult};
pub use patterns::{MatchArm, Pattern, PatternEngine};
