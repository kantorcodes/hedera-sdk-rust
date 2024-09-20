/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use hedera_proto::services;

use crate::pending_airdrop_id::PendingAirdropId;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};

/// A record of a new pending airdrop.
#[derive(Clone, Debug)]
pub struct PendingAirdropRecord {
    /// A unique, composite, identifier for a pending airdrop.
    /// This field is REQUIRED.
    pub pending_airdrop_id: PendingAirdropId,

    /// A single pending airdrop amount.
    pub pending_airdrop_value: Option<u64>,
}

impl PendingAirdropRecord {
    /// Create a new `NodeAddressBook` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }
}

impl FromProtobuf<services::PendingAirdropRecord> for PendingAirdropRecord {
    fn from_protobuf(pb: services::PendingAirdropRecord) -> crate::Result<Self> {
        let airdrop_id = PendingAirdropId::from_protobuf(pb_getf!(pb, pending_airdrop_id)?)?;
        Ok(Self {
            pending_airdrop_id: airdrop_id,
            pending_airdrop_value: pb.pending_airdrop_value.map(|v| v.amount),
        })
    }
}

impl ToProtobuf for PendingAirdropRecord {
    type Protobuf = services::PendingAirdropRecord;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::PendingAirdropRecord {
            pending_airdrop_id: Some(self.pending_airdrop_id.to_protobuf()),
            pending_airdrop_value: self
                .pending_airdrop_value
                .map(|v| hedera_proto::services::PendingAirdropValue { amount: v }),
        }
    }
}