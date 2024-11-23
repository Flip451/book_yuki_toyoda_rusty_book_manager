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
macro_rules! tuple_value_object_without_error {
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

        #[derive(Debug, thiserror::Error)]
        pub enum $error {
            #[error("invalid {0}")]
            InvalidValue($value),
        }

        impl $crate::model::value_object::ValueObjectError for $error {}
    };
}
