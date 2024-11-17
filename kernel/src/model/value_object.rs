pub trait ValueObject
where
    Self: Sized + PartialEq + TryFrom<Self::Value>,
{
    type Value;
    type Error;

    fn inner_ref(&self) -> &Self::Value;
    fn into_inner(self) -> Self::Value;
}

#[macro_export]
macro_rules! tuple_value_object_without_error {
    ($name:ident, $value:ty) => {
        #[derive(Debug, Eq, Hash, PartialEq, Clone)]
        pub struct $name($value);

        impl $crate::model::value_object::ValueObject for $name {
            type Value = $value;
            type Error = anyhow::Error;

            fn inner_ref(&self) -> &Self::Value {
                &self.0
            }

            fn into_inner(self) -> Self::Value {
                self.0
            }
        }

        impl TryFrom<$value> for $name {
            type Error = anyhow::Error;

            fn try_from(value: $value) -> Result<Self, Self::Error> {
                Ok(Self(value))
            }
        }
    };
}
