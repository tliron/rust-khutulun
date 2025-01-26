/// Creates a [Value](super::super::normal::Value) from a bare primitive expression.
#[macro_export]
macro_rules! normal (
    ( $value:expr ) => ( $crate::normal::Value::from($value) );
);

/// Creates a [Value::ValueList](super::super::normal::Value::ValueList) from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_list (
    () => (
        $crate::normal::Value::ValueList(
            $crate::normal::ValueList::new(
                $crate::normal::List::new()
            )
        )
    );

    ( $( $value:expr ),+ $( , )? ) => (
        $crate::normal::Value::ValueList(
            $crate::normal::ValueList::new(
                $crate::normal::List::new_with(
                    [ $( $crate::normal!( $value ) ),+ ]
                )
            )
        )
    );
);

/// Creates a [Value::ValueMap](super::super::normal::Value::ValueMap) from a sequence of key-value tuples.
#[macro_export]
macro_rules! normal_map (
    () => (
        $crate::normal::Value::Map(
            $crate::normal::ValueMap::new(
                $crate::normal::Map::new()
            )
        )
    );

    ( $( ( $key:expr, $value:expr ) ),+ $( , )? ) => (
        $crate::normal::Value::ValueMap(
            $crate::normal::ValueMap::new(
                $crate::normal::Map::new_with(
                    [ $( ( $crate::normal!( $key ), $crate::normal!( $value ) ) ),+ ]
                )
            )
        )
    );
);

/// Creates a [Vec]<[Value](super::super::normal::Value)> from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_vec (
    ( $( $value:expr ),* $( , )? ) => (
        vec![ $( $crate::normal!( $value ) ),* ]
    );
);
