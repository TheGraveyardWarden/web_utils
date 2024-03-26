#[macro_export]
macro_rules! to_bson {
    ($val: expr) => {
        bson::to_bson($val).unwrap()
    };
}

pub use to_bson;
