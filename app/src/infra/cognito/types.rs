use derive_more::Deref;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct Jwk {
    pub e: String,
    // alg: String, RS256
    // kty: String, RSA
    pub kid: String,
    pub n: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyResponse {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Deserialize, Deref)]
#[serde(rename_all = "camelCase")]
pub struct Claims(serde_json::Map<String, Value>);
impl Claims {
    pub fn username(&self) -> Option<String> {
        self.get_str_val("cognito:username")
    }
    pub fn email(&self) -> Option<String> {
        self.get_str_val("email")
    }

    pub fn get_str_val(&self, key: &str) -> Option<String> {
        self.0.get(key).and_then(|v| match v {
            Value::String(v) => Some(v.to_string()),
            _ => None,
        })
    }
}
