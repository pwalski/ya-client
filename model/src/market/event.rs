/*
 * Yagna Market API
 *
 * The version of the OpenAPI document: 1.6.1
 *
 * Generated by: https://openapi-generator.tech
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::market::{Agreement, PropertyQuery, Proposal, Reason};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "eventType")]
pub enum ProviderEvent {
    #[serde(rename = "ProposalEvent")]
    ProposalEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "proposal")]
        proposal: Proposal,
    },
    #[serde(rename = "ProposalRejectedEvent")]
    ProposalRejectedEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "proposalId")]
        proposal_id: String,
        #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
        reason: Option<Reason>,
    },
    #[serde(rename = "AgreementEvent")]
    AgreementEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "agreement")]
        agreement: Agreement,
    },
    #[serde(rename = "PropertyQueryEvent")]
    PropertyQueryEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "propertyQuery")]
        property_query: PropertyQuery,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "eventType")]
pub enum RequestorEvent {
    #[serde(rename = "ProposalEvent")]
    ProposalEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "proposal")]
        proposal: Proposal,
    },
    #[serde(rename = "ProposalRejectedEvent")]
    ProposalRejectedEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "proposalId")]
        proposal_id: String,
        #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
        reason: Option<Reason>,
    },
    #[serde(rename = "PropertyQueryEvent")]
    PropertyQueryEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "propertyQuery")]
        property_query: PropertyQuery,
    },
}
