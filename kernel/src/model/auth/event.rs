use crate::model::user::UserId;

use super::AccessToken;

pub struct CreateToken {
    pub user_id: UserId,
    pub access_token: AccessToken,
}

impl CreateToken {
    pub fn new(user_id: UserId) -> Self {
        let access_token = AccessToken(uuid::Uuid::new_v4().simple().to_string());
        Self {
            user_id,
            access_token,
        }
    }
}
