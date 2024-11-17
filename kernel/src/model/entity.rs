pub use anyhow::Result;

pub trait Entity
where
    Self::Identity: PartialEq,
{
    type Identity;

    fn identity(&self) -> &Self::Identity;
    fn eq(&self, other: &Self) -> bool {
        self.identity() == other.identity()
    }
}

#[macro_export]
macro_rules! impl_entity {
    ($target:ty, $identity_field:ident, $identity_type:ty) => {
        impl $crate::model::entity::Entity for $target {
            type Identity = $identity_type;

            fn identity(&self) -> &Self::Identity {
                &self.$identity_field
            }
        }
    };
}
