/*
 * Yagna Activity API
 *
 * It conforms with capability level 1 of the [Activity API specification](https://docs.google.com/document/d/1BXaN32ediXdBHljEApmznSfbuudTU8TmvOmHKl0gmQM).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderEvent {
    pub activity_id: String,
    pub agreement_id: String,
    pub event_type: ProviderEventType,
    pub event_date: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProviderEventType {
    #[serde(rename = "CreateActivity")]
    CreateActivity {
        #[serde(rename = "requestorPubKey", skip_serializing_if = "Option::is_none")]
        requestor_pub_key: Option<String>,
    },
    #[serde(rename = "DestroyActivity")]
    DestroyActivity {},
}
