pub trait ValueObject: PartialEq
where
    Self: Sized,
{
    type Value;
    type Error;

    fn new(value: Self::Value) -> Result<Self, Self::Error>;
    fn inner_ref(&self) -> &Self::Value;
    fn into_inner(self) -> Self::Value;
}

#[macro_export]
macro_rules! tuple_value_object_without_error {
    ($name:ident, $value:ty) => {
        #[derive(Debug, Eq, Hash, PartialEq, Clone, derive_new::new)]
        pub struct $name($value);

        impl $crate::model::value_object::ValueObject for $name {
            type Value = $value;
            type Error = anyhow::Error;

            fn new(value: Self::Value) -> Result<Self, Self::Error> {
                Ok($name(value))
            }

            fn inner_ref(&self) -> &Self::Value {
                &self.0
            }

            fn into_inner(self) -> Self::Value {
                self.0
            }
        }
    };
}
