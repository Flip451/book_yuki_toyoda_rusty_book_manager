pub trait ValueObject
where
    Self: Sized + PartialEq + TryFrom<Self::Value>,
    <Self as ValueObject>::Error: ValueObjectError,
{
    type Value;
    type Error;

    fn inner_ref(&self) -> &Self::Value;
    fn into_inner(self) -> Self::Value;
}

// マーカートレイト
pub trait ValueObjectError: std::error::Error {}

#[macro_export]
macro_rules! tuple_value_object_with_simple_error {
    ($name:ident, $value:ty, $error:ident) => {
        #[derive(Debug, Eq, Hash, PartialEq, Clone, derive_new::new)]
        pub struct $name($value);

        impl $crate::model::value_object::ValueObject for $name {
            type Value = $value;
            type Error = $error;

            fn inner_ref(&self) -> &Self::Value {
                &self.0
            }

            fn into_inner(self) -> Self::Value {
                self.0
            }
        }

        impl TryFrom<$value> for $name {
            type Error = $error;

            fn try_from(value: $value) -> Result<Self, Self::Error> {
                Ok(Self(value))
            }
        }

        #[derive(Debug, thiserror::Error)]
        pub enum $error {
            #[error("invalid {0}")]
            InvalidValue($value),

            #[error("parse error: {0}")]
            ParseError(#[from] Box<dyn std::error::Error + Send + Sync>),
        }

        impl $crate::model::value_object::ValueObjectError for $error {}
    };
}

#[macro_export]
macro_rules! tuple_value_object_requiring_error_definition {
    ($name:ident, $value:ty, $error:ident) => {
        #[derive(Debug, Eq, Hash, PartialEq, Clone)]
        pub struct $name($value);

        impl $crate::model::value_object::ValueObject for $name {
            type Value = $value;
            type Error = $error;

            fn inner_ref(&self) -> &Self::Value {
                &self.0
            }

            fn into_inner(self) -> Self::Value {
                self.0
            }
        }

        impl TryFrom<$value> for $name {
            type Error = $error;

            fn try_from(value: $value) -> Result<Self, Self::Error> {
                Ok(Self(value))
            }
        }

        impl $crate::model::value_object::ValueObjectError for $error {}
    };
}

#[macro_export]
macro_rules! enum_value_object_with_simple_error {
    (
        $(#[$derive:meta])?
        $name:ident
        {
            $(
                $(#[$meta_for_each_variant:meta])*
                $variant:ident,
            )+
        },
        $error:ident
    ) => {
        $(#[$derive])?
        #[derive(Debug, Eq, Hash, PartialEq, Clone)]
        pub enum $name {
            $(
                $(#[$meta_for_each_variant])*
                $variant,
            )+
        }

        impl $crate::model::value_object::ValueObject for $name {
            type Value = Self;
            type Error = $error;

            fn inner_ref(&self) -> &Self::Value {
                &self
            }

            fn into_inner(self) -> Self::Value {
                self
            }
        }

        #[derive(Debug, thiserror::Error)]
        pub enum $error {}

        impl $crate::model::value_object::ValueObjectError for $error {}
    };
}

#[macro_export]
macro_rules! define_id_with_uuid {
    ($name:ident, $error:ident) => {
        tuple_value_object_with_simple_error!($name, uuid::Uuid, $error);

        impl std::str::FromStr for $name {
            type Err = $error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(
                    uuid::Uuid::parse_str(s).map_err(|e| $error::ParseError(Box::new(e)))?,
                ))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    self.0
                        .as_simple()
                        .encode_lower(&mut uuid::Uuid::encode_buffer())
                )
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> Self {
                id.to_string()
            }
        }
    };
}
