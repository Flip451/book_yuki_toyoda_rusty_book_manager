pub trait Entity: Eq
where
    Self::Identity: Eq,
{
    type Identity;

    fn identity(&self) -> &Self::Identity;
    fn eq(&self, other: &Self) -> bool {
        self.identity() == other.identity()
    }
}

// TODO: identity_field で複数のフィールドを指定できるようにする
#[macro_export]
macro_rules! impl_entity {
    ($target:ty, $identity_field:ident, $identity_type:ty) => {
        impl $crate::model::entity::Entity for $target {
            type Identity = $identity_type;

            fn identity(&self) -> &Self::Identity {
                &self.$identity_field
            }
        }

        impl PartialEq for $target {
            fn eq(&self, other: &Self) -> bool {
                <$target as $crate::model::entity::Entity>::eq(self, other)
            }
        }

        impl Eq for $target {}
    };
}
