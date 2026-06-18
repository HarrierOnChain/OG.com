//! OG.com venue metadata.
//!
//! The execution core, risk layer, and strategy implementations live in the
//! shared engine crate. This module just describes the venue this binary targets.

/// Display name of this venue.
pub const NAME: &str = "OG.com";

/// Venue category.
pub const VENUE_TYPE: &str = "Social / multi-outcome";

/// Strategies this venue runs on the shared engine.
pub const STRATEGIES: &[&str] = &[
    "Sports Execution",
    "Orderbook Imbalance",
    "Market Making",
];
