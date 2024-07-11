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

    #[allow(unused)]
    pub(crate) use debug;

    #[allow(unused)]
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

    #[allow(unused)]
    pub(crate) use debug;

    #[allow(unused)]
    pub(crate) use warning;
}

#[cfg(all(not(feature = "std"), not(feature = "defmt")))]
pub mod logger {
    macro_rules! debug {
        ($($args:expr),* $(,)?) => {};
    }

    macro_rules! warning {
        ($($args:expr),* $(,)?) => {};
    }

    #[allow(unused)]
    pub(crate) use debug;

    #[allow(unused)]
    pub(crate) use warning;
}
