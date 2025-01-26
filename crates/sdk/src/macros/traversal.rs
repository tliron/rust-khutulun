// TODO!

/// Traverse a value by calling [Value::get](super::super::normal::Value::get) recursively.
///
/// The first argument is the starting [Value](super::super::normal::Value). The following arguments
/// are a sequence of keys, which will be tried one at a time. Any non-collection or
/// missing key will cause the macro to stop and return [None].
///
/// The keys are either [Value](super::super::normal::Value) or anything that implements
/// [Into]<[Value](super::super::normal::Value)>, which includes all the supported primitive types.
#[macro_export]
macro_rules! traverse(
    ( $value:expr ) => ( ::std::option::Option::<&$crate::functions::Value>::Some(&$value) );

    ( $value:expr, $key:expr ) => ( $value.into_get($key) );

    ( $value:expr, $key:expr, $( $next_key:expr ),+ $( , )? ) => (
        match $crate::traverse!( $value, $key ) {
            ::std::option::Option::Some(value) => $crate::traverse!( value $( , $next_key )+ ),
            ::std::option::Option::None => ::std::option::Option::None,
        }
    );
);

/// Traverse a value by calling [Value::get_mut](super::super::normal::Value::get_mut) recursively.
///
/// The first argument is the starting [Value](super::super::normal::Value). The following arguments
/// are a sequence of keys, which will be tried one at a time. Any non-collection or
/// missing key will cause the macro to stop and return [None].
///
/// The keys are either [Value](super::super::normal::Value) or anything that implements
/// [Into]<[Value](super::super::normal::Value)>, which includes all the supported primitive types.
#[macro_export]
macro_rules! traverse_mut(
    ( $value:expr ) => ( ::std::option::Option::<&mut $crate::functions::Value>::Some($value) );

    ( $value:expr, $key:expr ) => ( $value.into_get_mut($key) );

    ( $value:expr, $key:expr, $( $next_key:expr ),+ $( , )? ) => (
        match $crate::traverse_mut!( $value, $key ) {
            ::std::option::Option::Some(value) => $crate::traverse_mut!( value $( , $next_key )+ ),
            ::std::option::Option::None => ::std::option::Option::None,
        }
    );
);
