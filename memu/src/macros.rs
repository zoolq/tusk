/// Macro for creating a byte instance
#[cfg(feature = "macro")]
#[macro_export]
macro_rules! bytes {
    ($val:expr) => {
        Byte::from($val)
    };
}

/// Macro for creating a kilo byte instance
#[cfg(feature = "macro")]
#[macro_export]
macro_rules! kilo_bytes {
    ($val:expr) => {
        KiloByte::from($val)
    };
}

/// Macro for creating a mega byte instance
#[cfg(feature = "macro")]
#[macro_export]
macro_rules! mega_bytes {
    ($val:expr) => {
        MegaByte::from($val)
    };
}

/// Macro for creating a giga byte instance
#[cfg(feature = "macro")]
#[macro_export]
macro_rules! giga_byte {
    ($val:expr) => {
        GigaByte::from($val)
    };
}

/// Macro for creating a tera byte instance
#[cfg(feature = "macro")]
#[macro_export]
macro_rules! tera_byte {
    ($val:expr) => {
        TeraByte::from($val)
    };
}

/// Macro for creating a peta byte instance
#[cfg(feature = "macro")]
#[macro_export]
macro_rules! peta_byte {
    ($val:expr) => {
        PetaByte::from($val)
    };
}
