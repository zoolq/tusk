macro_rules! data_impl {
    (
        Self = $self:ty,
        Unit = $unit:literal,
        Factor = $factor:ident $(,)?
    ) => {
        impl $self {
            /// Maps any function to the internally held [`u64`].
            pub fn map<F, T>(&mut self, f: F) -> T
            where
                F: FnOnce(u64) -> T,
            {
                f(self.0)
            }

            /// Returns the unit suffix of this unit of data.
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("assert_eq!(", stringify!($self), "::UNIT, ", stringify!($unit), ");")]
            /// ```
            #[cfg(feature = "units")]
            pub const fn get_unit() -> &'static str {
                Self::UNIT
            }

            /// Creates this unit from a [`u8`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("# use memu::constants::", stringify!($factor), ";")]
            #[doc = concat!("let unit = ", stringify!($self), "::from_u8(1);")]
            ///
            #[doc = concat!("assert_eq!(unit, ",stringify!($self),"::new(", stringify!($factor), "));")]
            /// ```
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or the equivalent [`From`] trait")]
            pub const fn from_u8(value: u8) -> Self {
                <$self>::new(value as u64 * Self::FACTOR)
            }

            /// Creates this unit from a [`u16`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("# use memu::constants::", stringify!($factor), ";")]
            #[doc = concat!("let unit = ", stringify!($self), "::from_u16(1);")]
            ///
            #[doc = concat!("assert_eq!(unit, ",stringify!($self),"::new(", stringify!($factor), "));")]
            /// ```
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or the equivalent [`From`] trait")]
            pub const fn from_u16(value: u16) -> Self {
                <$self>::new(value as u64 * Self::FACTOR)
            }

            /// Creates this unit from a [`u32`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("# use memu::constants::", stringify!($factor), ";")]
            #[doc = concat!("let unit = ", stringify!($self), "::from_u32(1);")]
            ///
            #[doc = concat!("assert_eq!(unit, ",stringify!($self),"::new(", stringify!($factor), "));")]
            /// ```
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or the equivalent [`From`] trait")]
            pub const fn from_u32(value: u32) -> Self {
                <$self>::new(value as u64 * Self::FACTOR)
            }

            /// Creates this unit from a [`u64`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("# use memu::constants::", stringify!($factor), ";")]
            #[doc = concat!("let unit = ", stringify!($self), "::from_u64(1);")]
            ///
            #[doc = concat!("assert_eq!(unit, ",stringify!($self),"::new(", stringify!($factor), "));")]
            /// ```
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or the equivalent [`From`] trait")]
            pub const fn from_u64(value: u64) -> Self {
                <$self>::new(value * Self::FACTOR)
            }

            /// Creates this unit from a [`u128`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("# use memu::constants::", stringify!($factor), ";")]
            #[doc = concat!("let unit = ", stringify!($self), "::from_u128(1);")]
            ///
            #[doc = concat!("assert_eq!(unit, ",stringify!($self),"::new(", stringify!($factor), "));")]
            /// ```
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or the equivalent [`From`] trait")]
            pub const fn from_u128(value: u128) -> Self {
                <$self>::new(value as u64 * Self::FACTOR)
            }

            /// Creates this unit from the absoloute value of a [`i8`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("# use memu::constants::", stringify!($factor), ";")]
            #[doc = concat!("let unit = ", stringify!($self), "::from_i8(-1);")]
            ///
            #[doc = concat!("assert_eq!(unit, ",stringify!($self),"::new(", stringify!($factor), "));")]
            /// ```
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or the equivalent [`From`] trait")]
            pub const fn from_i8(value: i8) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }

            /// Creates this unit from the absoloute value of a [`i16`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("# use memu::constants::", stringify!($factor), ";")]
            #[doc = concat!("let unit = ", stringify!($self), "::from_i16(-1);")]
            ///
            #[doc = concat!("assert_eq!(unit, ",stringify!($self),"::new(", stringify!($factor), "));")]
            /// ```
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or the equivalent [`From`] trait")]
            pub const fn from_i16(value: i16) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }

            /// Creates this unit from the absoloute value of a [`i32`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("# use memu::constants::", stringify!($factor), ";")]
            #[doc = concat!("let unit = ", stringify!($self), "::from_i32(-1);")]
            ///
            #[doc = concat!("assert_eq!(unit, ",stringify!($self),"::new(", stringify!($factor), "));")]
            /// ```
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or the equivalent [`From`] trait")]
            pub const fn from_i32(value: i32) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }

            /// Creates this unit from the absoloute value of a [`i64`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("# use memu::constants::", stringify!($factor), ";")]
            #[doc = concat!("let unit = ", stringify!($self), "::from_i64(-1);")]
            ///
            #[doc = concat!("assert_eq!(unit, ",stringify!($self),"::new(", stringify!($factor), "));")]
            /// ```
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or the equivalent [`From`] trait")]
            pub const fn from_i64(value: i64) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }

            /// Creates this unit from the absoloute value of a [`i128`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use memu::units::", stringify!($self), ";")]
            #[doc = concat!("# use memu::constants::", stringify!($factor), ";")]
            #[doc = concat!("let unit = ", stringify!($self), "::from_i128(-1);")]
            ///
            #[doc = concat!("assert_eq!(unit, ",stringify!($self),"::new(", stringify!($factor), "));")]
            /// ```
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or the equivalent [`From`] trait")]
            pub const fn from_i128(value: i128) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }

            /// Creates this unit from the absoloute value of a [`f32`].
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or an equivalent From trait.")]
            pub fn from_f32(value: f32) -> Self {
                let value = if value.is_sign_negative() {
                    -value
                } else {
                    value
                };
                <$self>::new((value * Self::FACTOR as f32) as u64)
            }

            /// Creates this unit from the absoloute value of a [f64].
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or an equivalent From trait.")]
            pub fn from_f64(value: f64) -> Self {
                let value = if value.is_sign_negative() {
                    -value
                } else {
                    value
                };
                <$self>::new((value * Self::FACTOR as f64) as u64)
            }

            /// Returns this unit as a `f32`.
            pub fn as_f32(&self) -> f32 {
                self.0 as f32 / Self::FACTOR as f32
            }

            /// Returns this unit as a `f64`.
            pub fn as_f64(&self) -> f64 {
                self.0 as f64 / Self::FACTOR as f64
            }

            /// Returns this unit as a `u8`.
            pub const fn as_u8(&self) -> u8 {
                (self.0 / Self::FACTOR) as u8
            }

            /// Returns this unit as a `u16`.
            pub const fn as_u16(&self) -> u16 {
                (self.0 / Self::FACTOR) as u16
            }

            /// Returns this unit as a `u32`.
            pub const fn as_u32(&self) -> u32 {
                (self.0 / Self::FACTOR) as u32
            }

            /// Returns this unit as a `u64`.
            pub const fn as_u64(&self) -> u64 {
                (self.0 / Self::FACTOR) as u64
            }

            /// Returns a string containing`self.as_u64()` and `Self::UNIT`.
            #[cfg(feature = "units")]
            pub fn as_string_with_unit(&self) -> String {
                format!("{:.}{}", self.as_f64(), Self::UNIT)
            }

            /// Returns a string contaning `self.as_f64()` with the given precision and `Self::UNIT`.
            #[cfg(feature = "units")]
            pub fn as_string_with_unit_and_precision(&self, precision: usize) -> String {
                format!("{:.precision$}{}", self.as_f64(), Self::UNIT, precision = precision)
            }

        }

        impl std::ops::AddAssign for $self {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl std::ops::Add for $self {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                <$self>::from(self.0 + rhs.0)
            }
        }

        impl std::ops::BitAnd for $self {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                <$self>::from(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAndAssign for $self {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0
            }
        }

        impl std::ops::BitOr for $self {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                <$self>::from(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOrAssign for $self {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0
            }
        }

        impl std::ops::BitXor for $self {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self::Output {
                <$self>::from(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXorAssign for $self {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0
            }
        }

        impl std::ops::Mul for $self {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                <$self>::from(self.0 * rhs.0)
            }
        }

        impl std::ops::MulAssign for $self {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0
            }
        }

        impl std::ops::RemAssign for $self {
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0
            }
        }

        impl std::ops::Sub for $self {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                <$self>::from(self.0 - rhs.0)
            }
        }

        impl std::ops::SubAssign for $self {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl From<u8> for $self {
            fn from(value: u8) -> Self {
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<u16> for $self {
            fn from(value: u16) -> Self {
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<u32> for $self {
            fn from(value: u32) -> Self {
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<u64> for $self {
            fn from(value: u64) -> Self {
                <$self>::new(value * Self::FACTOR)
            }
        }

        impl From<u128> for $self {
            fn from(value: u128) -> Self {
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<i8> for $self {
            fn from(value: i8) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<i16> for $self {
            fn from(value: i16) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<i32> for $self {
            fn from(value: i32) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<i64> for $self {
            fn from(value: i64) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<i128> for $self {
            fn from(value: i128) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64)
            }
        }

        impl From<f32> for $self {
            /// Creates this unit from the absoloute value of a [f64].
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or an equivalent From trait.")]
            fn from(value: f32) -> Self {
                let value = if value.is_sign_negative() {
                    -value
                } else {
                    value
                };
                <$self>::new((value * Self::FACTOR as f32) as u64)
            }
        }

        impl From<f64> for $self {
            /// Creates this unit from the absoloute value of a [f64].
            ///
            #[doc = concat!("Note: this function takes in a direct amount of the unit, if you want to create ", stringify!($self), " from bytes use [`", stringify!($self), "::new`] or an equivalent From trait.")]
            fn from(value: f64) -> Self {
                let value = if value.is_sign_negative() {
                    -value
                } else {
                    value
                };
                <$self>::new((value * Self::FACTOR as f64) as u64)
            }
        }

        impl std::fmt::Debug for $self {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($self), self.0)
            }
        }

        impl std::fmt::Display for $self {
            /// This displays the amout of bytes. If you want to display the unit directly use
            #[doc= concat!("[`", stringify!($self), "::as_f64()`] or a similar casting method.")]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::ops::Deref for $self {
            type Target = u64;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $self {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }


        impl $crate::MemoryUnit for $self {
            fn as_bits(&self) -> u128 {
                self.0 as u128 * $crate::constants::BYTE as u128
            }

            fn as_byte(&self) -> Byte {
                $crate::units::Byte::new(self.0)
            }

            fn as_kilo_byte(&self) -> $crate::units::KiloByte {
                $crate::units::KiloByte::new(self.0)
            }

            fn as_mega_byte(&self) -> $crate::units::MegaByte {
                $crate::units::MegaByte::new(self.0)
            }

            fn as_giga_byte(&self) -> $crate::units::GigaByte {
                $crate::units::GigaByte::new(self.0)
            }

            fn as_tera_byte(&self) -> $crate::units::TeraByte {
                $crate::units::TeraByte::new(self.0)
            }

            fn as_peta_byte(&self) -> $crate::units::PetaByte {
                $crate::units::PetaByte::new(self.0)
            }
        }
    };
}

pub(crate) use data_impl;
