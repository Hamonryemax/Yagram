use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JwtUserPayload {
    pub sub: String,
}
