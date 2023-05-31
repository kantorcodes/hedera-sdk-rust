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
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use time::Duration;
use tonic::transport::Channel;

use crate::ledger_id::RefLedgerId;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::staked_id::StakedId;
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    Hbar,
    Key,
    PublicKey,
    Transaction,
    ValidateChecksums,
};

/// Create a new Hedera™ account.
pub type AccountCreateTransaction = Transaction<AccountCreateTransactionData>;

// TODO: shard_id: Option<ShardId>
// TODO: realm_id: Option<RealmId>
// TODO: new_realm_admin_key: Option<Key>,

#[derive(Debug, Clone)]
pub struct AccountCreateTransactionData {
    /// The key that must sign each transfer out of the account.
    ///
    /// If `receiver_signature_required` is true, then it must also sign any transfer
    /// into the account.
    key: Option<Key>,

    /// The initial number of Hbar to put into the account.
    initial_balance: Hbar,

    /// If true, this account's key must sign any transaction depositing into this account.
    receiver_signature_required: bool,

    /// The account is charged to extend its expiration date every this many seconds.
    auto_renew_period: Option<Duration>,

    /// The account to be used at this account's expiration time to extend the
    /// life of the account.  If `None`, this account pays for its own auto renewal fee.
    auto_renew_account_id: Option<AccountId>,

    /// The memo associated with the account.
    account_memo: String,

    /// The maximum number of tokens that an Account can be implicitly associated with.
    ///
    /// Defaults to `0`. Allows up to a maximum value of `1000`.
    max_automatic_token_associations: u16,

    /// A key to be used as the account's alias.
    alias: Option<PublicKey>,

    /// A 20-byte EVM address to be used as the account's evm address.
    evm_address: Option<[u8; 20]>,

    /// ID of the account or node to which this account is staking, if any.
    staked_id: Option<StakedId>,

    /// If true, the account declines receiving a staking reward. The default value is false.
    decline_staking_reward: bool,
}

impl Default for AccountCreateTransactionData {
    fn default() -> Self {
        Self {
            key: None,
            initial_balance: Hbar::ZERO,
            receiver_signature_required: false,
            auto_renew_period: Some(Duration::days(90)),
            auto_renew_account_id: None,
            account_memo: String::new(),
            max_automatic_token_associations: 0,
            alias: None,
            evm_address: None,
            staked_id: None,
            decline_staking_reward: false,
        }
    }
}

impl AccountCreateTransaction {
    /// Get the key this account will be created with.
    ///
    /// Returns `Some(key)` if previously set, `None` otherwise.
    #[must_use]
    pub fn get_key(&self) -> Option<&Key> {
        self.data().key.as_ref()
    }

    /// Sets the key for this account.
    pub fn key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().key = Some(key.into());
        self
    }

    /// Get the balance that will be transferred to this account on creation.
    ///
    /// Returns `initial_balance` if previously set, `0` otherwise.
    #[must_use]
    pub fn get_initial_balance(&self) -> Hbar {
        self.data().initial_balance
    }

    /// Sets the balance that will be transferred to this account on creation.
    pub fn initial_balance(&mut self, balance: Hbar) -> &mut Self {
        self.data_mut().initial_balance = balance;
        self
    }

    /// Returns `true` if this account must sign any transfer of hbars _to_ itself.
    #[must_use]
    pub fn get_receiver_signature_required(&self) -> bool {
        self.data().receiver_signature_required
    }

    /// Sets to true to require this account to sign any transfer of hbars to this account.
    pub fn receiver_signature_required(&mut self, required: bool) -> &mut Self {
        self.data_mut().receiver_signature_required = required;
        self
    }

    /// Returns the auto renew period for this account.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Option<Duration> {
        self.data().auto_renew_period
    }

    /// Sets the auto renew period for this account.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.data_mut().auto_renew_period = Some(period);
        self
    }

    /// Gets the account to be used at this account's expiration time to extend the
    /// life of the account.  If `None`, this account pays for its own auto renewal fee.
    ///
    /// # Network Support
    /// Please note that this not supported on any hedera network at this time.
    #[must_use]
    pub fn get_auto_renew_account_id(&self) -> Option<AccountId> {
        self.data().auto_renew_account_id
    }

    /// Sets the account to be used at this account's expiration time to extend the
    /// life of the account.  If `None`, this account pays for its own auto renewal fee.
    ///
    /// # Network Support
    /// Please note that this not supported on any hedera network at this time.
    pub fn auto_renew_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().auto_renew_account_id = Some(id);
        self
    }

    /// Get the memo associated with the account
    #[must_use]
    pub fn get_account_memo(&self) -> &str {
        &self.data().account_memo
    }

    /// Sets the memo associated with the account.
    pub fn account_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().account_memo = memo.into();
        self
    }

    /// Get the maximum number of tokens that an Account can be implicitly associated with.
    ///
    /// Defaults to `0`. Allows up to a maximum value of `1000`.
    #[must_use]
    pub fn get_max_automatic_token_associations(&self) -> u16 {
        self.data().max_automatic_token_associations
    }

    /// Sets the maximum number of tokens that an Account can be implicitly associated with.
    pub fn max_automatic_token_associations(&mut self, amount: u16) -> &mut Self {
        self.data_mut().max_automatic_token_associations = amount;
        self
    }

    /// Returns the public key to be used as the account's alias.
    ///
    /// # Network Support
    /// Please note that this not currently supported on mainnet.
    #[must_use]
    pub fn get_alias(&self) -> Option<&PublicKey> {
        self.data().alias.as_ref()
    }

    /// The bytes to be used as the account's alias.
    ///
    /// A given alias can map to at most one account on the network at a time. This uniqueness will be enforced
    /// relative to aliases currently on the network at alias assignment.
    ///
    /// If a transaction creates an account using an alias, any further crypto transfers to that alias will
    /// simply be deposited in that account, without creating anything, and with no creation fee being charged.
    ///
    /// # Network Support
    /// Please note that this not currently supported on mainnet.
    pub fn alias(&mut self, key: PublicKey) -> &mut Self {
        self.data_mut().alias = Some(key);
        self
    }

    /// Returns the evm address the account will be created with.
    ///
    /// # Network Support
    /// Please note that this not currently supported on mainnet.
    #[must_use]
    pub fn get_evm_address(&self) -> Option<[u8; 20]> {
        self.data().evm_address
    }

    /// The last 20 bytes of the keccak-256 hash of a `ECDSA_SECP256K1` primitive key.
    ///
    /// # Network Support
    /// Please note that this not currently supported on mainnet.
    pub fn evm_address(&mut self, evm_address: [u8; 20]) -> &mut Self {
        self.data_mut().evm_address = Some(evm_address);
        self
    }

    /// Returns the ID of the account to which this account is staking.
    /// This is mutually exclusive with `staked_node_id`.
    #[must_use]
    pub fn get_staked_account_id(&self) -> Option<AccountId> {
        self.data().staked_id.and_then(StakedId::to_account_id)
    }

    /// Sets the ID of the account to which this account is staking.
    /// This is mutually exclusive with `staked_node_id`.
    pub fn staked_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().staked_id = Some(StakedId::AccountId(id));
        self
    }

    /// Returns the ID of the node to which this account is staking.
    /// This is mutually exclusive with `staked_account_id`.
    #[must_use]
    pub fn get_staked_node_id(&self) -> Option<u64> {
        self.data().staked_id.and_then(StakedId::to_node_id)
    }

    /// Sets the ID of the node to which this account is staking.
    /// This is mutually exclusive with `staked_account_id`.
    pub fn staked_node_id(&mut self, id: u64) -> &mut Self {
        self.data_mut().staked_id = Some(StakedId::NodeId(id));
        self
    }

    /// Returns `true` if the account should decline receiving staking rewards, `false` otherwise.
    #[must_use]
    pub fn get_decline_staking_reward(&self) -> bool {
        self.data().decline_staking_reward
    }

    /// If `true`, the account declines receiving a staking reward. The default value is false.
    pub fn decline_staking_reward(&mut self, decline: bool) -> &mut Self {
        self.data_mut().decline_staking_reward = decline;
        self
    }
}

impl TransactionData for AccountCreateTransactionData {}

impl TransactionExecute for AccountCreateTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { CryptoServiceClient::new(channel).create_account(request).await })
    }
}

impl ValidateChecksums for AccountCreateTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.staked_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for AccountCreateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::CryptoCreateAccount(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for AccountCreateTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::CryptoCreateAccount(self.to_protobuf())
    }
}

impl From<AccountCreateTransactionData> for AnyTransactionData {
    fn from(transaction: AccountCreateTransactionData) -> Self {
        Self::AccountCreate(transaction)
    }
}

impl FromProtobuf<services::CryptoCreateTransactionBody> for AccountCreateTransactionData {
    fn from_protobuf(pb: services::CryptoCreateTransactionBody) -> crate::Result<Self> {
        let evm_address = pb.alias.as_slice().try_into().ok();

        let alias = (pb.alias.len() != 20)
            .then(|| PublicKey::from_alias_bytes(&pb.alias).transpose())
            .flatten()
            .transpose()?;

        Ok(Self {
            key: Option::from_protobuf(pb.key)?,
            initial_balance: Hbar::from_tinybars(pb.initial_balance as i64),
            receiver_signature_required: pb.receiver_sig_required,
            auto_renew_period: None,
            auto_renew_account_id: None,
            account_memo: pb.memo,
            max_automatic_token_associations: pb.max_automatic_token_associations as u16,
            alias,
            evm_address,
            staked_id: Option::from_protobuf(pb.staked_id)?,
            decline_staking_reward: pb.decline_reward,
        })
    }
}

impl ToProtobuf for AccountCreateTransactionData {
    type Protobuf = services::CryptoCreateTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        let key = self.key.to_protobuf();
        let auto_renew_period = self.auto_renew_period.to_protobuf();
        let staked_id = self.staked_id.map(|it| match it {
            StakedId::NodeId(id) => {
                services::crypto_create_transaction_body::StakedId::StakedNodeId(id as i64)
            }
            StakedId::AccountId(id) => {
                services::crypto_create_transaction_body::StakedId::StakedAccountId(
                    id.to_protobuf(),
                )
            }
        });

        #[allow(deprecated)]
        services::CryptoCreateTransactionBody {
            key,
            initial_balance: self.initial_balance.to_tinybars() as u64,
            proxy_account_id: None,
            send_record_threshold: i64::MAX as u64,
            receive_record_threshold: i64::MAX as u64,
            receiver_sig_required: self.receiver_signature_required,
            auto_renew_period,
            shard_id: None,
            realm_id: None,
            new_realm_admin_key: None,
            memo: self.account_memo.clone(),
            max_automatic_token_associations: i32::from(self.max_automatic_token_associations),
            alias: self.alias.map_or(vec![], |key| key.to_bytes_raw()),
            decline_reward: self.decline_staking_reward,
            staked_id,
        }
    }
}