#[macro_export]
macro_rules! get_field {
    ($field: expr, $input: expr) => {
        doc!{
            "$getField": doc!{
                "field": $field,
                "input": $input
            }
        }
    };
}

pub use get_field;

#[macro_export]
macro_rules! switch {
    (default $default: expr, $(case $case: expr; then $then: expr),+) => {
        doc!{
            "$switch": doc!{
                "branches": vec![
                    $(
                        doc!{
                            "case": $case,
                            "then": $then
                        },
                    )+
                ],
                "default": $default
            }
        }
    };
    ($(case $case: expr; then $then: expr),+) => {
        doc!{
            "$switch": doc!{
                "branches": vec![
                    $(
                        doc!{
                            "case": $case,
                            "then": $then
                        },
                    )+
                ]
            }
        }
    };
}

pub use switch;

