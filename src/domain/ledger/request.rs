use serde;
use serde_json;
use time;

use crate::utils::did::{DidValue, ShortDidValue};
use std::collections::HashMap;

pub const DEFAULT_LIBIDY_DID: &str = "LibindyDid111111111111";
pub const DEFAULT_PROTOCOL_VERSION: ProtocolVersion = ProtocolVersion::Node1_4;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ProtocolVersion {
    Node1_3 = 1,
    Node1_4 = 2,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TxnAuthrAgrmtAcceptanceData {
    pub mechanism: String,
    pub taa_digest: String,
    pub time: u64,
}

fn get_req_id() -> u64 {
    time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request<T: serde::Serialize> {
    pub req_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<ShortDidValue>,
    pub operation: T,
    pub protocol_version: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taa_acceptance: Option<TxnAuthrAgrmtAcceptanceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endorser: Option<ShortDidValue>,
}

impl<T: serde::Serialize> Request<T> {
    pub fn new(
        req_id: u64,
        identifier: ShortDidValue,
        operation: T,
        protocol_version: ProtocolVersion,
    ) -> Request<T> {
        Request {
            req_id,
            identifier: Some(identifier),
            operation,
            protocol_version: Some(protocol_version as usize),
            signature: None,
            signatures: None,
            taa_acceptance: None,
            endorser: None,
        }
    }

    pub fn build_request(
        protocol_version: ProtocolVersion,
        identifier: Option<&DidValue>,
        operation: T,
    ) -> Result<String, String> {
        let req_id = get_req_id();

        let identifier = match identifier {
            Some(identifier_) => identifier_.clone().to_short(),
            None => ShortDidValue(DEFAULT_LIBIDY_DID.to_string()),
        };

        serde_json::to_string(&Request::new(
            req_id,
            identifier,
            operation,
            protocol_version,
        ))
        .map_err(|err| format!("Cannot serialize Request: {:?}", err))
    }
}
