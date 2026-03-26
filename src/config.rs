//! Configuration for Bakong API client

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    Sandbox,
    Production,
}

impl Environment {
    pub fn base_url(&self) -> &str {
        match self {
            Environment::Sandbox => "https://sit-api-bakong.nbc.gov.kh",
            Environment::Production => "https://api-bakong.nbc.gov.kh",
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Sandbox
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BakongConfig {
    pub token: String,
    pub environment: Environment,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    #[serde(default)]
    pub base_url: Option<String>,
}

fn default_timeout() -> u64 {
    30
}

impl BakongConfig {
    pub fn sandbox(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            environment: Environment::Sandbox,
            timeout_secs: default_timeout(),
            base_url: None,
        }
    }

    pub fn production(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            environment: Environment::Production,
            timeout_secs: default_timeout(),
            base_url: None,
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn is_sandbox(&self) -> bool {
        self.environment == Environment::Sandbox
    }
}
