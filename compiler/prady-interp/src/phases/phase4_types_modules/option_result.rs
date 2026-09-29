// Phase 4 — Option & Result Built-in Types & Try Operator (?)
// Core monadic types for error handling and nullable representation in Prady.

#[derive(Debug, Clone, PartialEq)]
pub enum PradyOption<T> {
    Some(T),
    None,
}

impl<T> PradyOption<T> {
    pub fn is_some(&self) -> bool {
        matches!(self, PradyOption::Some(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, PradyOption::None)
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            PradyOption::Some(v) => v,
            PradyOption::None => default,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> PradyOption<U> {
        match self {
            PradyOption::Some(v) => PradyOption::Some(f(v)),
            PradyOption::None => PradyOption::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PradyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> PradyResult<T, E> {
    pub fn is_ok(&self) -> bool {
        matches!(self, PradyResult::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, PradyResult::Err(_))
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            PradyResult::Ok(v) => v,
            PradyResult::Err(_) => default,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> PradyResult<U, E> {
        match self {
            PradyResult::Ok(v) => PradyResult::Ok(f(v)),
            PradyResult::Err(e) => PradyResult::Err(e),
        }
    }

    /// Implements the semantics of the `?` try operator in Prady.
    /// Propagates Err early, or returns Ok value.
    pub fn try_unwrap(self) -> Result<T, E> {
        match self {
            PradyResult::Ok(val) => Ok(val),
            PradyResult::Err(err) => Err(err),
        }
    }
}
