pub mod source;

use chrono::{DateTime, Utc};
use rusr_decimal::Decimal;

#[drive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol(pub String);

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// One price observation, regardelss of transport.
#[derive(Debug, Clone)]
pub struct Tick {
    pub symbol: Symbol,
    pub price: Decimal,
    pub at: DateTime<Utc>,
}
