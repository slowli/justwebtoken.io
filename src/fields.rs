//! Standard claims and token headers.

use once_cell::sync::Lazy;
use yew::{html, Html};

use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy)]
pub struct Field {
    pub name: &'static str,
    pub description: &'static str,
    pub link: Option<&'static str>,
}

#[derive(Clone)]
pub struct FieldWithValue {
    pub field: Field,
    pub value: Html,
}

impl Field {
    pub fn with_value(self, value: &dyn fmt::Display) -> FieldWithValue {
        FieldWithValue {
            field: self,
            value: html! { value },
        }
    }

    pub fn with_html_value(self, value: Html) -> FieldWithValue {
        FieldWithValue { field: self, value }
    }

    pub fn with_code_value(self, value: &dyn fmt::Display) -> FieldWithValue {
        self.with_html_value(html! { <code>{ value }</code> })
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

    pub fn with_value(self, value: &dyn fmt::Display) -> FieldWithValue {
        self.0.with_value(value)
    }

    pub fn with_code_value(self, value: &dyn fmt::Display) -> FieldWithValue {
        self.0.with_code_value(value)
    }
}
