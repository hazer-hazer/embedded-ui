#[cfg(all(not(feature = "std"), feature = "defmt"))]
pub mod logger {
    macro_rules! debug {
        ($($args: expr),* $(,)?) => {
            defmt::debug!($($args),*)
        };
    }

    macro_rules! warning {
        ($($args: expr),* $(,)?) => {
            defmt::warn!($($args),*)
        };
    }

    pub(crate) use debug;
    pub(crate) use warning;
}

#[cfg(all(feature = "std", not(feature = "defmt")))]
pub mod logger {
    macro_rules! debug {
        ($($args: expr),* $(,)?) => {
            std::println!($($args),*)
        };
    }

    macro_rules! warning {
        ($($args: expr),* $(,)?) => {
            std::eprintln!($($args),*)
        };
    }

    pub(crate) use debug;
    pub(crate) use warning;
}
