//! Common macros.

macro_rules! define_enum {
    ($($vis:vis enum $enum_name:ident($enum_type:ty, $enum_doc:literal) {
        $($variant_name:ident => ($variant_value:literal, $variant_doc:literal)),+ $(,)?
    })+) => {
        $(
        #[doc = concat!($enum_doc, ".")]
        $vis enum $enum_name {
            $(
            #[doc = concat!($variant_doc, ".")]
            $variant_name,
            )+
            #[doc = "Unknown."]
            Unknown($enum_type),
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                    $enum_name::$variant_name => write!(f, $variant_doc),
                    )+
                    $enum_name::Unknown(v) => write!(f, "Unknown ({v:#x})"),
                }
            }
        }

        impl std::convert::From<$enum_type> for $enum_name {
            fn from(v: $enum_type) -> $enum_name {
                match v {
                    $(
                    $variant_value => $enum_name::$variant_name,
                    )+
                    _ => $enum_name::Unknown(v),
                }
            }
        }

        impl std::convert::From<$enum_name> for $enum_type {
            fn from(v: $enum_name) -> $enum_type {
                match v {
                    $(
                    $enum_name::$variant_name => $variant_value,
                    )+
                    $enum_name::Unknown(v) => v,
                }
            }
        }
        )+
    };
}

macro_rules! define_opaque {
    ($vis:vis opaque $name:ident(const)) => {
        /// Constant opaque type.
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        $vis struct $name(*const std::ffi::c_void);

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $name {
            /// Returns an unsafe pointer to the opaque object.
            pub fn as_ptr(&self) -> *const std::ffi::c_void {
                self.0
            }
        }
    };
    ($vis:vis opaque $name:ident(mut)) => {
        /// Mutable opaque type.
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        $vis struct $name(*mut std::ffi::c_void);

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $name {
            /// Returns an unsafe pointer to the opaque object.
            pub fn as_ptr(&self) -> *const std::ffi::c_void {
                self.0
            }

            /// Returns an unsafe mutable pointer to the opaque object.
            pub fn as_mut_ptr(&self) -> *mut std::ffi::c_void {
                self.0
            }
        }
    };
    ($($vis:vis opaque $name:ident($mut:ident);)+) => {
        $(
        $crate::macros::define_opaque! {
            $vis opaque $name($mut)
        }
        )+
    };
}

pub(crate) use define_enum;
pub(crate) use define_opaque;
