import toml

TOML_FILE_NAME = "build-src/hazedumper_csgo.toml"

parsed_toml = toml.load(TOML_FILE_NAME)

timestamp = parsed_toml["timestamp"]
signatures = parsed_toml["signatures"]
netvars = parsed_toml["netvars"]

netvars_members = ""
netvars_inits = ""

for (key, val) in netvars.items():
    netvars_members += f"    pub {key}: usize,\n"
    netvars_inits += f"            {key}: {val},\n"

signatures_members = ""
signatures_inits = ""

for (key, val) in signatures.items():
    signatures_members += f"    pub {key}: usize,\n"
    signatures_inits += f"            {key}: {val},\n"

generated = f"""
#[derive(Debug, Clone, Copy)]
pub struct HazeDumper {{
    pub timestamp: u64,
    pub signatures: Signatures,
    pub netvars: Netvars,
}}

impl HazeDumper {{
    #[allow(clippy::unreadable_literal)]
    pub const fn new() -> Self {{
        HazeDumper {{
            timestamp: {timestamp},
            signatures: Signatures::new(),
            netvars: Netvars::new(),
        }}
    }}
}}

#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct Signatures {{
{signatures_members}
}}

#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct Netvars {{
{netvars_members}
}}

impl Signatures {{
    #[allow(clippy::unreadable_literal)]
    pub const fn new() -> Self {{
        Signatures {{
{signatures_inits}
        }}
    }}
}}

impl Netvars {{
    #[allow(clippy::unreadable_literal)]
    pub const fn new() -> Self {{
        Netvars {{
{netvars_inits}
        }}
    }}
}}

impl Default for HazeDumper {{
    fn default() -> Self {{
        HazeDumper::new()
    }}
}}

impl Default for Signatures {{
    fn default() -> Self {{
        Signatures::new()
    }}
}}

impl Default for Netvars {{
    fn default() -> Self {{
        Netvars::new()
    }}
}}

/// A static instance containing the signatures and netvars as fetched by
/// HazeDumper and placed inside the `/build-src/hazedumper_csgo.toml` file by
/// `/fetch-hazedumper.sh`.
pub static HAZEDUMPER: self::HazeDumper = self::HazeDumper::new();
"""

print(generated)