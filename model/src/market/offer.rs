/*
 * Yagna Market API
 *
 * The version of the OpenAPI document: 1.6.1
 *
 * Generated by: https://openapi-generator.tech
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Offer {
    /// The object which includes all the Offer properties.
    /// This is a JSON object in \"flat convention\" - where keys are full
    /// property names and their values indicate properties.
    ///
    /// The value's Javascript type shall conform with the type of the
    /// property (as indicated in Golem Standards).
    ///
    /// ### Example property object:
    /// ```json
    /// {
    ///     "golem.com.pricing.model": "linear",
    ///     "golem.com.pricing.model.linear.coeffs": [0.001, 0.002, 0.0],
    ///     "golem.com.scheme": "payu",
    ///     "golem.com.scheme.payu.interval_sec": 6.0,
    ///     "golem.com.usage.vector": ["golem.usage.duration_sec", "golem.usage.cpu_sec"],
    ///     "golem.inf.cpu.architecture": "x86_64",
    ///     "golem.inf.cpu.cores": 4,
    ///     "golem.inf.cpu.threads": 7,
    ///     "golem.inf.mem.gib": 10.612468048930168,
    ///     "golem.inf.storage.gib": 81.7227783203125,
    ///     "golem.node.debug.subnet": "market-devnet",
    ///     "golem.node.id.name": "tworec@mf-market-devnet",
    ///     "golem.runtime.name": "vm",
    ///     "golem.runtime.version@v": "0.1.0"
    /// }
    /// ```
    #[serde(rename = "properties")]
    pub properties: serde_json::Value,
    #[serde(rename = "constraints")]
    pub constraints: String,
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "providerId")]
    pub provider_id: String, // TODO: use NodeId

    // TODO: ttl
    /// Object creation timestamp
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
}

impl Offer {
    pub fn new(
        properties: serde_json::Value,
        constraints: String,
        offer_id: String,
        provider_id: String,
        timestamp: DateTime<Utc>,
    ) -> Offer {
        Offer {
            properties,
            constraints,
            offer_id,
            provider_id,
            timestamp,
        }
    }
}
