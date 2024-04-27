use once_cell::sync::Lazy;

use crate::auth::Claims;

pub static CLAIMS: Lazy<Claims> = Lazy::new(|| Claims {
    sub: "off-chain-agent".to_string(),
    company: "gobazzinga".to_string(),
});
