//! Transforms `src/claims.yml` into a function.

use linked_hash_map::LinkedHashMap;
use serde::Deserialize;

use std::{
    collections::HashMap,
    env,
    error::Error,
    fmt,
    fs::{self, File},
    io::Write as _,
    path::Path,
};

#[derive(Debug, Deserialize)]
struct Field {
    name: String,
    description: String,
    link: Option<String>,
}

impl fmt::Display for Field {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "(Field {{ name: {name:?}, description: {descr:?}, link: {link:?} }})",
            name = self.name,
            descr = self.description.trim(),
            link = self.link
        )
    }
}

#[derive(Debug, Deserialize)]
struct Claim {
    #[serde(flatten)]
    field: Field,
    category: String,
}

impl fmt::Display for Claim {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{{ field: {field}, category: {cat:?} }}",
            field = self.field,
            cat = self.category
        )
    }
}

#[derive(Debug, Deserialize)]
struct ClaimCategory {
    title: String,
}

impl fmt::Display for ClaimCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, " {{ title: {title:?} }}", title = self.title)
    }
}

#[derive(Debug, Deserialize)]
struct StandardFields {
    standard_headers: HashMap<String, Field>,
    standard_claims: HashMap<String, Claim>,
    claims_categories: LinkedHashMap<String, ClaimCategory>,
}

fn generate_map_fn<T: fmt::Display>(
    dest_file: &mut File,
    fn_name: &str,
    ty: &str,
    fields: &HashMap<String, T>,
) -> Result<(), Box<dyn Error>> {
    writeln!(
        dest_file,
        "fn {fn_name}() -> HashMap<&'static str, {ty}> {{",
        fn_name = fn_name,
        ty = ty
    )?;
    writeln!(
        dest_file,
        "    let mut map = HashMap::with_capacity({});",
        fields.len()
    )?;
    for (field_name, field) in fields {
        writeln!(
            dest_file,
            "    map.insert({name:?}, {ty}{field});",
            name = field_name,
            ty = ty,
            field = field
        )?;
    }
    writeln!(dest_file, "    map")?;
    writeln!(dest_file, "}}")?;
    Ok(())
}

fn generate_slice_fn<T: fmt::Display>(
    dest_file: &mut File,
    fn_name: &str,
    ty: &str,
    fields: &LinkedHashMap<String, T>,
) -> Result<(), Box<dyn Error>> {
    writeln!(
        dest_file,
        "const fn {fn_name}() -> &'static [(&'static str, {ty})] {{",
        fn_name = fn_name,
        ty = ty
    )?;
    writeln!(dest_file, "    &[")?;
    for (field_name, field) in fields {
        writeln!(
            dest_file,
            "        ({name:?}, {ty}{field}),",
            name = field_name,
            ty = ty,
            field = field
        )?;
    }
    writeln!(dest_file, "    ]")?;
    writeln!(dest_file, "}}")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let src_file = fs::read("src/fields.toml")?;
    let fields: StandardFields = toml::from_slice(&src_file)?;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("std_maps.rs");
    let mut dest_file = File::create(dest_path)?;

    generate_map_fn(
        &mut dest_file,
        "create_headers_map",
        "StandardHeader",
        &fields.standard_headers,
    )?;
    generate_map_fn(
        &mut dest_file,
        "create_claims_map",
        "StandardClaim",
        &fields.standard_claims,
    )?;
    generate_slice_fn(
        &mut dest_file,
        "create_claim_categories",
        "ClaimCategory",
        &fields.claims_categories,
    )?;

    // Set up caching logic.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/fields.toml");

    Ok(())
}
