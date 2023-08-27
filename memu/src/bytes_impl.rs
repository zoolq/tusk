macro_rules! data_impl {
    ($self:ty) => {
        impl $self {
            /// Maps any function to the internally held `u64`.
            pub fn map<F, T>(&mut self, f: F) -> T
            where
                F: FnOnce(u64) -> T,
            {
                f(self.0)
            }

            /// Returns the units suffix.
            #[cfg(feature = "units")]
            pub fn get_unit() -> &'static str {
                Self::UNIT
            }

            /// Returns the unit as a `f32`.
            pub fn as_f32(&self) -> f32 {
                self.0 as f32 / Self::FACTOR as f32
            }

            /// Returns the unit as a `f64`.
            pub fn as_f64(&self) -> f64 {
                self.0 as f64 / Self::FACTOR as f64
            }

            /// Returns the unit as a `u8`.
            pub fn as_u8(&self) -> u8 {
                (self.0 / Self::FACTOR) as u8
            }

            /// Returns the unit as a `u16`.
            pub fn as_u16(&self) -> u16 {
                (self.0 / Self::FACTOR) as u16
            }

            /// Returns the unit as a `u32`.
            pub fn as_u32(&self) -> u32 {
                (self.0 / Self::FACTOR) as u32
            }

            /// Returns the unit as a `u64`.
            pub fn as_u64(&self) -> u64 {
                (self.0 / Self::FACTOR) as u64
            }

            /// Returns a string containing `self.as_f64` at the availible precision.
            pub fn as_string(&self) -> String {
                format!("{:.}", self.as_f64())
            }

            /// Returns a string containing`self.as_u64()` and `Self::UNIT`.
            #[cfg(feature = "units")]
            pub fn as_string_with_unit(&self) -> String {
                format!("{:.}{}", self.as_f64(), Self::UNIT)
            }

            /// Returns a string containing `self.as_f64()` with the given precision.
            pub fn as_string_with_precision(&self, precision: usize) -> String {
                format!("{:.precision$}", self.as_f64(), precision = precision)
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
            #[doc = concat!("Takes in a direct amount of the unit, if you want to create ", stringify!($self), " based on bytes use [`", stringify!($self), "::new`].")]
            ///
            #[doc = concat!("Converts the absolute value into ", stringify!($self), ".")]
            fn from(value: i8) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<i16> for $self {
            #[doc = concat!("Takes in a direct amount of the unit, if you want to create ", stringify!($self), " based on bytes use [`", stringify!($self), "::new`].")]
            ///
            #[doc = concat!("Converts the absolute value into ", stringify!($self), ".")]
            fn from(value: i16) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<i32> for $self {
            #[doc = concat!("Takes in a direct amount of the unit, if you want to create ", stringify!($self), " based on bytes use [`", stringify!($self), "::new`].")]
            ///
            #[doc = concat!("Converts the absolute value into ", stringify!($self), ".")]
            fn from(value: i32) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<i64> for $self {
            #[doc = concat!("Takes in a direct amount of the unit, if you want to create ", stringify!($self), " based on bytes use [`", stringify!($self), "::new`].")]
            ///
            #[doc = concat!("Converts the absolute value into ", stringify!($self), ".")]
            fn from(value: i64) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<i128> for $self {
            #[doc = concat!("Takes in a direct amount of the unit, if you want to create ", stringify!($self), " based on bytes use [`", stringify!($self), "::new`].")]
            ///
            #[doc = concat!("Converts the absolute value into ", stringify!($self), ".")]
            fn from(value: i128) -> Self {
                let value = if value.is_negative() { -value } else { value };
                <$self>::new(value as u64 * Self::FACTOR)
            }
        }

        impl From<f32> for $self {
            #[doc = concat!("Converts the absolute value into ", stringify!($self), ".")]
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
            #[doc = concat!("Converts the absolute value into ", stringify!($self), ".")]
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
            #[doc= concat!("[`", stringify!($self), "::as_f64()`], or a similar casting method.")]
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
