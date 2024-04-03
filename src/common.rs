#[macro_export]
macro_rules! to_bson {
    ($val: expr) => {
        bson::to_bson($val).unwrap()
    };
}

pub use to_bson;

#[macro_export]
macro_rules! def_vec_obj_as_objs {
    ($objs: ident, $obj: ty) => {
        #[derive(Serialize, Deserialize)]
        pub struct $objs(Vec<$obj>);

        impl From<Vec<$obj>> for $objs {
            fn from(v: Vec<$obj>) -> Self {
                Self(v)
            }
        }
    };
}

pub use def_vec_obj_as_objs;
