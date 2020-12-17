/*
 * Yagna Market API
 *
 * The version of the OpenAPI document: 1.6.1
 *
 * Generated by: https://openapi-generator.tech
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::market::{Demand, Offer};
use crate::NodeId;

/// Agreement expresses the terms of the deal between Provider and Requestor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Agreement {
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    #[serde(rename = "demand")]
    pub demand: Demand,
    #[serde(rename = "offer")]
    pub offer: Offer,
    /// End of validity period.
    ///
    /// Agreement needs to be approved, rejected or cancelled before this date;
    /// otherwise will expire.
    #[serde(rename = "validTo")]
    pub valid_to: DateTime<Utc>,
    /// Agreement approval timestamp
    #[serde(rename = "approvedDate", skip_serializing_if = "Option::is_none")]
    pub approved_date: Option<DateTime<Utc>>,
    /// See [State](enum.State.html).
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "appSessionId", skip_serializing_if = "Option::is_none")]
    pub app_session_id: Option<String>,
    #[serde(rename = "proposedSignature", skip_serializing_if = "Option::is_none")]
    pub proposed_signature: Option<String>,
    #[serde(rename = "approvedSignature", skip_serializing_if = "Option::is_none")]
    pub approved_signature: Option<String>,
    #[serde(rename = "committedSignature", skip_serializing_if = "Option::is_none")]
    pub committed_signature: Option<String>,
}

impl Agreement {
    pub fn new(
        agreement_id: String,
        demand: Demand,
        offer: Offer,
        valid_to: DateTime<Utc>,
        state: State,
        timestamp: DateTime<Utc>,
    ) -> Agreement {
        Agreement {
            agreement_id,
            demand,
            offer,
            valid_to,
            approved_date: None,
            state,
            timestamp,
            app_session_id: None,
            proposed_signature: None,
            approved_signature: None,
            committed_signature: None,
        }
    }

    pub fn provider_id(&self) -> &NodeId {
        &self.offer.provider_id
    }

    pub fn requestor_id(&self) -> &NodeId {
        &self.demand.requestor_id
    }
}

/// * `Proposal` - newly created by a Requestor (based on Proposal)
/// * `Pending` - confirmed by a Requestor and send to Provider for approval
/// * `Cancelled` by a Requestor
/// * `Rejected` by a Provider
/// * `Approved` by both sides
/// * `Expired` - not accepted, rejected nor cancelled within validity period
/// * `Terminated` - finished after approval.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    /// Newly created by a Requestor (draft based on Proposal)
    #[serde(rename = "Proposal")]
    Proposal,
    /// Confirmed by a Requestor and send to Provider for approval
    #[serde(rename = "Pending")]
    Pending,
    /// Cancelled by a Requestor
    #[serde(rename = "Cancelled")]
    Cancelled,
    /// Rejected by a Provider
    #[serde(rename = "Rejected")]
    Rejected,
    /// Approved by both sides
    #[serde(rename = "Approved")]
    Approved,
    /// Not approved, rejected nor cancelled within validity period
    #[serde(rename = "Expired")]
    Expired,
    /// Finished after approval
    #[serde(rename = "Terminated")]
    Terminated,
}
