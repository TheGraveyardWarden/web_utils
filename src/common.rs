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
        #[derive(Serialize, Deserialize, Debug)]
        pub struct $objs(Vec<$obj>);

        impl From<Vec<$obj>> for $objs {
            fn from(v: Vec<$obj>) -> Self {
                Self(v)
            }
        }

        impl std::ops::Deref for $objs {
            type Target = Vec<$obj>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $objs {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

pub use def_vec_obj_as_objs;
