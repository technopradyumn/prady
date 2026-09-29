// Phase 6 — OOP, Interfaces, Traits, and Architecture/Quality Policy Engine
// Module root

pub mod classes;
pub mod traits;
pub mod policy_engine;

pub use classes::{ClassDef, ClassHierarchy, MethodDef};
pub use policy_engine::{ArchViolation, ArchitecturePolicyEngine, LayerPolicy};
pub use traits::{TraitDef, TraitMethod, VTable};
