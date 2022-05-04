macro_rules! unsupported_types {
    ($($ty:ident)*) => {
        $($crate::de::util::unsupported_types_helper!{$ty})*
    }
}

macro_rules! unsupported_types_helper {
    (tuple) => {
        $crate::de::util::unsupported_types_helper!{
            tuple(len: usize)
        }
    };
    (struct) => {
        $crate::de::util::unsupported_types_helper!{
            struct(name: &'static str, fields: &'static [&'static str])
        }
    };
    (enum) => {
        $crate::de::util::unsupported_types_helper!{
            enum(name: &'static str, variants: &'static [&'static str])
        }
    };
    (tuple_struct) => {
        $crate::de::util::unsupported_types_helper!{
            tuple_struct(name: &'static str, len: usize)
        }
    };
    (unit_struct) => {
        $crate::de::util::unsupported_types_helper!{
            unit_struct(name: &'static str)
        }
    };
    ($ty:ident) => {
        $crate::de::util::unsupported_types_helper!{$ty()}
    };
    ($ty:ident($($arg:ident : $arg_ty:ty),*)) => {
        paste::paste! {
            fn [<deserialize_ $ty>]<V>(self, $([<_ $arg>]: $arg_ty,)* _visitor: V) -> Result<V::Value>
            where
                V: serde::de::Visitor<'de>
            {
                Err(Error::unsupported_type(stringify!($ty)))
            }
        }
    }
}

pub(crate) use unsupported_types;
pub(crate) use unsupported_types_helper;
