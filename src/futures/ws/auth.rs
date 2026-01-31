#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct FuturesWebsocketAuth {
    pub api_key: String,
    pub secret_key: String,
}

impl FuturesWebsocketAuth {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
        }
    }

    pub fn generate_signature(&self, req_time: &str) -> String {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        let mut mac = Hmac::<Sha256>::new_from_slice(
            self.secret_key
                .as_bytes(),
        )
        .expect("HMAC can take key of any size");
        mac.update(
            format!(
                "{}{}",
                self.api_key, req_time
            )
            .as_bytes(),
        );
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
}
