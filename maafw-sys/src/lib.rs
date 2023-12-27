pub mod maafw;
pub mod maatk;

pub use maafw::def::*;
pub use maafw::instance::controller::*;
pub use maafw::instance::resource::*;
pub use maatk::config::*;
pub use maatk::device::*;


/// `cenum` is a macro that defines a new type and a set of associated constants.
///
/// This macro can be used in two ways:
///
/// 1. With a `type_name` parameter, which specifies the name of the new type.
/// 2. Without a `type_name` parameter, in which case the name of the new type is the same as the name of the enum.
///
/// # Parameters
///
/// * `$name` - The name of the enum.
/// * `$t` - The type of the enum and the associated constants.
/// * `type_name=$type_name` - An optional parameter that specifies the name of the new type.
/// * `$variant` - The name of a constant.
/// * `$value` - The value of a constant.
///
/// # Example
///
/// ```rust
/// use maafw_sys::cenum;
/// cenum! {
///     enum MyEnum : i32, type_name=MyEnumType {
///         Foo = 0,
///         Bar = 1,
///     }
/// }
/// ```
///
/// This will generate the following code:
///
/// ```rust
/// pub type MyEnumType = i32;
///
/// #[allow(non_upper_case_globals)]
/// pub const Foo: i32 = 0;
/// #[allow(non_upper_case_globals)]
/// pub const Bar: i32 = 1;
/// ```
#[macro_export]
macro_rules! cenum {
    (enum $name:ident : $t:ty, type_name=$type_name:ident { $($variant:ident = $value:expr),* $(,)? }) => {
        pub type $type_name = $t;

        $(
            #[allow(non_upper_case_globals)]
            pub const $variant: $t = $value;
        )*
    };
    (enum $name:ident : $t:ty { $($variant:ident = $value:expr),* $(,)? }) => {
        pub type $name = $t;

        $(
            #[allow(non_upper_case_globals)]
            pub const $variant: $t = $value;
        )*
    };
}

/// `opaque` is a Rust macro that defines a new type as a mutable pointer to `c_void`.
///
/// This macro can be used in two ways:
///
/// 1. With an `underlying` parameter, document about underlying C type will be generated.
/// 2. Without an `underlying` parameter, no document will be generated.
///
/// # Parameters
///
/// * `$name` - The name of the new type.
/// * `underlying=$underlying_ty` - An optional parameter that specifies the original C type that the new type represents.
///
/// # Example
///
/// ```rust
/// use maafw_sys::opaque;
/// opaque!(MyOpaqueType, underlying=MyOriginalCType);
/// ```
///
/// This will generate the following code:
///
/// ```rust
/// /// ***Original C Header Definition***: `typedef struct MyOriginalCType* MyOpaqueType`. For safety reasons, the definition ***has been modified in Rust***
/// pub type MyOpaqueType = *mut libc::c_void;
/// ```
#[macro_export]
macro_rules! opaque {
    ($name:ident, underlying=$underlying_ty:ident) => {
        #[doc = concat!(
            "***Original C Header Definition***: `typedef struct ",
            stringify!($underlying_ty),
            "* ",
            stringify!($name),
            "`. For safety reasons, the definition ***has been modified in Rust***"
        )]
        pub type $name = *mut libc::c_void;
    };

    ($name:ident) => {
        pub type $name = *mut libc::c_void;
    };
}



