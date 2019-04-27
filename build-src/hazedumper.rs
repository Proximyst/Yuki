macro_rules! hazedump {
    (
    timestamp = $timestamp:literal

    [signatures]
    $($signame:ident = $sigval:literal)*

    [netvars]
    $($netvarname:ident = $netvarval:literal)*
    ) => {
    #[derive(Debug, Clone, Copy)]
    pub struct HazeDumper {
        pub timestamp: u64,
        pub signatures: Signatures,
        pub netvars: Netvars,
    }

    #[derive(Debug, Clone, Copy)]
    #[allow(non_snake_case)]
    pub struct Signatures {
        $(pub $signame: usize),*
    }

    #[derive(Debug, Clone, Copy)]
    #[allow(non_snake_case)]
    pub struct Netvars {
        $(pub $netvarname: usize),*
    }

    impl HazeDumper {
        pub const fn new() -> Self {
            HazeDumper {
                timestamp: $timestamp,
                signatures: Signatures::new(),
                netvars: Netvars::new(),
            }
        }
    }

    impl Signatures {
        #[allow(non_snake_case)]
        pub const fn new() -> Self {
            Signatures {
                $($signame: $sigval),*
            }
        }
    }

    impl Netvars {
        #[allow(non_snake_case)]
        pub const fn new() -> Self {
            Netvars {
                $($netvarname: $netvarval),*
            }
        }
    }

    impl Default for HazeDumper {
        fn default() -> Self {
            HazeDumper::new()
        }
    }

    impl Default for Signatures {
        fn default() -> Self {
            Signatures::new()
        }
    }

    impl Default for Netvars {
        fn default() -> Self {
            Netvars::new()
        }
    }
}
}

hazedump! {
    {toml}
}

/// A static instance containing the signatures and netvars as fetched by
/// HazeDumper and placed inside the `/build-src/hazedumper_csgo.toml` file by
/// `/fetch-hazedumper.sh`.
pub static HAZEDUMPER: self::HazeDumper = self::HazeDumper::new();
