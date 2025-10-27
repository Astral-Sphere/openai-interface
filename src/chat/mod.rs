//! Response to a given `chat` conversation.

use serde::{Deserialize, Serialize};

pub mod create;

/// The service tier used for processing the request.
///
/// This enum represents the different service tiers that can be specified when
/// making a request to the API. Each tier corresponds to different performance
/// characteristics and pricing models.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    /// Automatically select the service tier based on project settings.
    Auto,
    /// Use the default service tier with standard pricing and performance.
    Default,
    /// Use the flex service tier for flexible processing requirements.
    Flex,
    /// Use the scale service tier for scalable processing needs.
    Scale,
    /// Use the priority service tier for high-priority requests.
    Priority,
}
