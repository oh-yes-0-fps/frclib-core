
#[macro_use]
mod components;
#[macro_use]
mod number;
#[macro_use]
mod helper;
#[doc(hidden)]
pub use paste;

/// A macro for defining a unit of measurement.
/// This macro is used to define a new unit of measurement.
/// 
/// # Example
/// ```no_run
/// use frclib_core::unit;
/// 
/// unit!(DegreeFloat: float);
/// unit!(RadianInt: int);
/// unit!(RotationUint: uint);
/// ```
#[macro_export]
macro_rules! unit {
    ($unit_name:ident : float) => {
        /// A unit of measurement.
        /// This is a newtype wrapper around a [`f64`].
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $unit_name(pub f64);

        impl std::hash::Hash for $unit_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }

        impl $unit_name {
            /// Creates a new instance of the unit with the given value.
            #[must_use]
            #[inline]
            pub const fn new(value: f64) -> Self {
                Self(value)
            }

            /// Returns the inner [`f64`] value.
            #[must_use]
            #[inline]
            pub const fn value(self) -> f64 {
                self.0
            }
        }

        $crate::unit_general!($unit_name : f64);
        $crate::unit_binops!($unit_name : f64);
        $crate::unit_neg!($unit_name : f64);
        $crate::unit_serde!($unit_name : f64);
        $crate::unit_num!($unit_name : f64);
        $crate::unit_float!($unit_name);
        $crate::unit_structure!($unit_name : f64);
    };
    ($unit_name:ident : int) => {
        /// A unit of measurement.
        /// This is a newtype wrapper around a [`i64`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $unit_name(pub i64);

        impl $unit_name {
            /// Creates a new instance of the unit with the given value.
            #[must_use]
            #[inline]
            pub const fn new(value: i64) -> Self {
                Self(value)
            }

            /// Returns the inner [`i64`] value.
            #[must_use]
            #[inline]
            pub const fn value(self) -> i64 {
                self.0
            }
        }

        $crate::unit_general!($unit_name : i64);
        $crate::unit_binops!($unit_name : i64);
        $crate::unit_neg!($unit_name : i64);
        $crate::unit_serde!($unit_name : i64);
        $crate::unit_num!($unit_name : i64);
        $crate::unit_integer!($unit_name);
        $crate::unit_structure!($unit_name : i64);
    };
    ($unit_name:ident : uint) => {
        /// A unit of measurement.
        /// This is a newtype wrapper around a [`u64`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $unit_name(pub u64);

        impl $unit_name {
            /// Creates a new instance of the unit with the given value.
            #[must_use]
            #[inline]
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            /// Returns the inner [`u64`] value.
            #[must_use]
            #[inline]
            pub const fn value(self) -> u64 {
                self.0
            }
        }

        $crate::unit_general!($unit_name : u64);
        $crate::unit_binops!($unit_name : u64);
        $crate::unit_serde!($unit_name : u64);
        $crate::unit_num!($unit_name : u64);
        $crate::unit_uinteger!($unit_name);
        $crate::unit_structure!($unit_name : u64);
    };
}

/// A macro for defining a unit conversion.
/// This macro is used to define a conversion between two units of the same dimension.
/// 
/// # Example
/// ```no_run
/// use frclib_core::{unit_conversion, unit};
/// 
/// unit!(Degree: float);
/// unit!(Radian: float);
/// 
/// unit_conversion!(Degree(float) <-> Radian(float) ~ degree_to_radian);
/// ````
#[macro_export]
macro_rules! unit_conversion {
    ($unit_a:ident ( float ) <-> $unit_b:ident ( float ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a f64 | $unit_b f64 : $conv_fn);
    };
    ($unit_a:ident ( int ) <-> $unit_b:ident ( int ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a i64 | $unit_b i64 : $conv_fn);
    };
    ($unit_a:ident ( uint ) <-> $unit_b:ident ( uint ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a u64 | $unit_b u64 : $conv_fn);
    };
    ($unit_a:ident ( float ) <-> $unit_b:ident ( int ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a f64 | $unit_b i64 : $conv_fn);
    };
    ($unit_a:ident ( float ) <-> $unit_b:ident ( uint ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a f64 | $unit_b u64 : $conv_fn);
    };
    ($unit_a:ident ( int ) <-> $unit_b:ident ( float ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a i64 | $unit_b f64 : $conv_fn);
    };
    ($unit_a:ident ( uint ) <-> $unit_b:ident ( float ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a u64 | $unit_b f64 : $conv_fn);
    };
    ($unit_a:ident ( int ) <-> $unit_b:ident ( uint ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a i64 | $unit_b u64 : $conv_fn);
    };
    ($unit_a:ident ( uint ) <-> $unit_b:ident ( int ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!($unit_a u64 | $unit_b i64 : $conv_fn);
    };
}

/// A macro for defining a unit family.
/// Unit families allow all units to fall under a single trait.
/// This allows for easy conversion between units of the same family
/// and allows for functions to be generic over all units of a family.
/// 
/// # Example
/// ```no_run
/// use frclib_core::{unit_family, unit};
/// 
/// unit!(Degree: float);
/// unit!(Radian: float);
/// unit!(Rotation: float);
/// 
/// unit_conversion!(Degree(float) <-> Radian(float) ~ degree_to_radian);
/// unit_conversion!(Degree(float) <-> Rotation(float) ~ degree_to_rotation);
/// unit_conversion!(Radian(float) <-> Rotation(float) ~ radian_to_rotation);
/// 
/// unit_family!(Angle(Radian): Degree, Rotation);
/// ````
#[macro_export]
macro_rules! unit_family {
    ($family_name:ident ( $standard:ident ): $($unit_name:ident),*) => {
        $crate::units::macros::paste::paste! {
            #[doc = "A family of units. "]
            #[doc = "The standard unit is `" $standard "`."]
            #[doc = "The other units are `" $($unit_name)"`, `"* "`."]
            pub trait $family_name: Into<$standard> + From<$standard> + Copy {
                #[doc = "Converts this unit to the standard unit of the family."]
                #[inline]
                fn standard(self) -> $standard {
                    self.into()
                }

                $(
                    #[doc = "Converts this unit to `" $unit_name "`."]
                    #[inline]
                    fn [<to_ $unit_name:lower>](self) -> $unit_name {
                        $unit_name::from(self.standard())
                    }
                )*

                #[doc = "Converts this unit to `" $standard "`."]
                #[doc = "This is the same as [`standard`](#method.standard)." ]
                #[inline]
                fn [<to_ $standard:lower>](self) -> $standard {
                    self.standard()
                }
            }
        }

        impl<T> $family_name for T
        where
            T: Into<$standard> + From<$standard> + Copy,
        {
        }
    };
}

/// A macro for defining a unit dimension analysis.
/// 
/// # Example
/// ```no_run
/// use frclib_core::{unit_dim_analysis, unit};
/// 
/// unit!(Degree: float);
/// unit!(Second: float);
/// unit!(DegreePerSecond: float);
/// 
/// unit_dim_analysis!(DegreePerSecond * Second = Degree);
/// // also supports division but mult implicitly adds support for division the other way
/// ```
#[macro_export]
macro_rules! unit_dim_analysis {
    ($unit_a:ident * $unit_b:ident = $ret:ident) => {
        impl std::ops::Mul<$unit_b> for $unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<$unit_a> for $unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_b> for $unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<$unit_a> for &$unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<$unit_b> for &$unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_a> for $unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_b> for &$unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_a> for &$unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }

        //other order
        impl std::ops::Div<$unit_a> for $ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: $unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_b> for $ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: $unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_a> for $ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: &$unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_a> for &$ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: $unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_a> for &$ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: &$unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for $ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_b> for &$ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: $unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for &$ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
    };
    ($unit_a:ident / $unit_b:ident = $ret:ident) => {
        impl std::ops::Div<$unit_b> for $unit_a {
            type Output = $ret;
            fn div(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for $unit_a {
            type Output = $ret;
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_b> for &$unit_a {
            type Output = $ret;
            fn div(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for &$unit_a {
            type Output = $ret;
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
    };
}


#[cfg(test)]
mod test {
    unit!(Degree: float);
    unit!(Millisecond: int);
    unit!(Microsecond: uint);

    #[test]
    fn ops() {
        let deg = Degree(1.0);
        let new_deg = 1.0f64 + deg;
        assert_eq!(new_deg, Degree(2.0));

        let milli = Millisecond(-1);
        let new_milli = 1i64 + milli;
        assert_eq!(new_milli, Millisecond(0));

        let micro = Microsecond(1);
        let new_micro = 1u64 + micro;
        assert_eq!(new_micro, Microsecond(2));
    }
}
