use crate::domain::types::string::impl_len_restricted_string_model;
use crate::domain::user;

impl_len_restricted_string_model!(AssetKey, "S3キー", 1, 255);
impl AssetKey {
    pub fn asset_key(user_id: user::Id, file_name: String) -> Self {
        Self(format!("asset/{}/{}", user_id.as_str(), file_name))
    }
    pub fn temp_key(user_id: user::Id, file_name: String) -> Self {
        Self(format!("tmp/{}/{}", user_id.as_str(), file_name))
    }
}
