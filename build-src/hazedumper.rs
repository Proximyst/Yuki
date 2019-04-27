macro hazedump(
    timestamp = $timestamp:literal

    [signatures]
    $($signame:ident = $sigval:literal)*

    [netvars]
    $($netvarname:ident = $netvarval:literal)*
) {
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

    impl Default for HazeDumper {
        fn default() -> Self {
            HazeDumper {
                timestamp: $timestamp,
                signatures: Default::default(),
                netvars: Default::default(),
            }
        }
    }

    #[allow(non_snake_case)]
    impl Default for Signatures {
        fn default() -> Self {
            Signatures {
                $($signame: $sigval),*
            }
        }
    }

    #[allow(non_snake_case)]
    impl Default for Netvars {
        fn default() -> Self {
            Netvars {
                $($netvarname: $netvarval),*
            }
        }
    }
}

hazedump! {
    {toml}
}