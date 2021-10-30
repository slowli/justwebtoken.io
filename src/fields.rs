//! Standard claims and token headers.

use once_cell::sync::Lazy;

use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy)]
pub struct Field {
    pub name: &'static str,
    pub description: &'static str,
    pub link: &'static str,
}

#[derive(Clone, Copy)]
pub struct FieldWithValue<'a> {
    pub field: Field,
    pub value: &'a dyn fmt::Display,
}

impl Field {
    pub fn with_value(self, value: &dyn fmt::Display) -> FieldWithValue<'_> {
        FieldWithValue { field: self, value }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StandardClaim(pub Field);

// Defines:
//
// fn create_claims_map() -> HashMap<&'static str, StandardField> { /* ... */ }
// fn create_headers_map() -> HashMap<&'static str, StandardField> { /* ... */ }
include!(concat!(env!("OUT_DIR"), "/std_maps.rs"));

static CLAIMS_MAP: Lazy<HashMap<&'static str, StandardClaim>> = Lazy::new(create_claims_map);
static HEADERS_MAP: Lazy<HashMap<&'static str, StandardHeader>> = Lazy::new(create_headers_map);

impl StandardClaim {
    pub fn by_name(name: &str) -> Self {
        CLAIMS_MAP[name]
    }

    pub fn get(name: &str) -> Option<Self> {
        CLAIMS_MAP.get(name).copied()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StandardHeader(pub Field);

impl StandardHeader {
    pub fn by_name(name: &str) -> Self {
        HEADERS_MAP[name]
    }

    pub fn with_value(self, value: &dyn fmt::Display) -> FieldWithValue<'_> {
        self.0.with_value(value)
    }
}
