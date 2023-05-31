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
use hedera_proto::services::token_service_client::TokenServiceClient;
use time::{
    Duration,
    OffsetDateTime,
};
use tonic::transport::Channel;

use crate::ledger_id::RefLedgerId;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
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
    Key,
    TokenId,
    Transaction,
    ValidateChecksums,
};

/// At consensus, updates an already created token to the given values.
///
/// If no value is given for a field, that field is left unchanged. For an immutable token (that is,
/// a token without an admin key), only the expiry may be updated. Setting any other field in that
/// case will cause the transaction status to resolve to `TokenIsImmutable`.
///
/// ### --- Signing Requirements ---
/// 1. Whether or not a token has an admin key, its expiry can be extended with only the transaction
///    payer's signature.
/// 2. Updating any other field of a mutable token requires the admin key's signature.
/// 3. If a new admin key is set, this new key must sign **unless** it is exactly an empty
///    `KeyList`. This special sentinel key removes the existing admin key and causes the
///    token to become immutable. (Other [`Key`][Key] structures without a constituent
///    `Ed25519` key will be rejected with `InvalidAdminKey`.
/// 4. If a new treasury is set, the new treasury account's key must sign the transaction.
///
/// ### --- Nft Requirements ---
/// 1. If a non fungible token has a positive treasury balance, the operation will abort with
///    `CurrentTreasuryStillOwnsNfts`.
pub type TokenUpdateTransaction = Transaction<TokenUpdateTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TokenUpdateTransactionData {
    /// The token to be updated.
    token_id: Option<TokenId>,

    /// The publicly visible name of the token.
    token_name: String,

    /// The publicly visible token symbol.
    token_symbol: String,

    /// The account which will act as a treasury for the token.
    treasury_account_id: Option<AccountId>,

    /// The key which can perform update/delete operations on the token.
    admin_key: Option<Key>,

    /// The key which can grant or revoke KYC of an account for the token's transactions.
    kyc_key: Option<Key>,

    /// The key which can sign to freeze or unfreeze an account for token transactions.
    freeze_key: Option<Key>,

    /// The key which can wipe the token balance of an account.
    wipe_key: Option<Key>,

    /// The key which can change the supply of a token.
    supply_key: Option<Key>,

    /// An account which will be automatically charged to renew the token's expiration, at
    /// autoRenewPeriod interval
    auto_renew_account_id: Option<AccountId>,

    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    auto_renew_period: Option<Duration>,

    /// Sets the time at which the token should expire.
    expiration_time: Option<OffsetDateTime>,

    /// The memo associated with the token (UTF-8 encoding max 100 bytes)
    token_memo: String,

    /// The key which can change the token's custom fee schedule; must sign a TokenFeeScheduleUpdate
    /// transaction
    fee_schedule_key: Option<Key>,

    /// The Key which can pause and unpause the Token.
    /// If Empty the token pause status defaults to PauseNotApplicable, otherwise Unpaused.
    pause_key: Option<Key>,
}

impl TokenUpdateTransaction {
    /// Returns the token to be updated.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data().token_id
    }

    /// Sets the token to be updated.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.data_mut().token_id = Some(token_id.into());
        self
    }

    /// Returns the new publicly visible name of the token.
    #[must_use]
    pub fn get_token_name(&self) -> &str {
        &self.data().token_name
    }

    /// Sets the new publicly visible name of the token.
    ///
    /// Maximum 100 characters.
    pub fn token_name(&mut self, token_name: impl Into<String>) -> &mut Self {
        self.data_mut().token_name = token_name.into();
        self
    }

    ///Returns the new publicly visible token symbol.
    #[must_use]
    pub fn get_token_symbol(&self) -> &str {
        &self.data().token_symbol
    }

    /// Sets the new publicly visible token symbol.
    ///
    /// Maximum 100 characters.
    pub fn token_symbol(&mut self, token_symbol: impl Into<String>) -> &mut Self {
        self.data_mut().token_symbol = token_symbol.into();
        self
    }

    /// Returns the new account which will act as a treasury for the token.
    #[must_use]
    pub fn get_treasury_account_id(&self) -> Option<AccountId> {
        self.data().treasury_account_id
    }

    /// Sets the new account which will act as a treasury for the token.
    ///
    /// If the provided `treasury_account_id` does not exist or has been deleted, the response
    /// will be `InvalidTreasuryAccountForToken`.
    ///
    /// If successful, the token balance held in the previous treasury account is transferred to the
    /// new one.
    pub fn treasury_account_id(&mut self, treasury_account_id: AccountId) -> &mut Self {
        self.data_mut().treasury_account_id = Some(treasury_account_id);
        self
    }

    /// Returns the new key which can perform update/delete operations on the token.
    #[must_use]
    pub fn get_admin_key(&self) -> Option<&Key> {
        self.data().admin_key.as_ref()
    }

    /// Sets the new key which can perform update/delete operations on the token.
    ///
    /// If the token is immutable, transaction will resolve to `TokenIsImmutable`.
    pub fn admin_key(&mut self, admin_key: impl Into<Key>) -> &mut Self {
        self.data_mut().admin_key = Some(admin_key.into());
        self
    }

    /// Returns the new key which can grant or revoke KYC of an account for the token's transactions.
    #[must_use]
    pub fn get_kyc_key(&self) -> Option<&Key> {
        self.data().kyc_key.as_ref()
    }

    /// Sets the new key which can grant or revoke KYC of an account for the token's transactions.
    ///
    /// If the token does not currently have a KYC key, transaction will resolve to `TokenHasNoKycKey`.
    pub fn kyc_key(&mut self, kyc_key: impl Into<Key>) -> &mut Self {
        self.data_mut().kyc_key = Some(kyc_key.into());
        self
    }

    /// Returns the new key which can sign to freeze or unfreeze an account for token transactions.
    #[must_use]
    pub fn get_freeze_key(&self) -> Option<&Key> {
        self.data().freeze_key.as_ref()
    }

    /// Sets the new key which can sign to freeze or unfreeze an account for token transactions.
    ///
    /// If the token does not currently have a Freeze key, transaction will resolve to `TokenHasNoFreezeKey`.
    pub fn freeze_key(&mut self, freeze_key: impl Into<Key>) -> &mut Self {
        self.data_mut().freeze_key = Some(freeze_key.into());
        self
    }

    /// Returns the new key which can wipe the token balance of an account.
    #[must_use]
    pub fn get_wipe_key(&self) -> Option<&Key> {
        self.data().wipe_key.as_ref()
    }

    /// Sets the new key which can wipe the token balance of an account.
    ///
    /// If the token does not currently have a Wipe key, transaction will resolve to `TokenHasNoWipeKey`.
    pub fn wipe_key(&mut self, wipe_key: impl Into<Key>) -> &mut Self {
        self.data_mut().wipe_key = Some(wipe_key.into());
        self
    }

    /// Returns the new key which can change the supply of a token.
    #[must_use]
    pub fn get_supply_key(&self) -> Option<&Key> {
        self.data().supply_key.as_ref()
    }

    /// Sets the new key which can change the supply of a token.
    ///
    /// If the token does not currently have a Supply key, transaction will resolve to `TokenHasNoSupplyKey`.
    pub fn supply_key(&mut self, supply_key: impl Into<Key>) -> &mut Self {
        self.data_mut().supply_key = Some(supply_key.into());
        self
    }

    /// Returns the new account which will be automatically charged to renew the token's expiration.
    #[must_use]
    pub fn get_auto_renew_account_id(&self) -> Option<AccountId> {
        self.data().auto_renew_account_id
    }

    /// Sets the new account which will be automatically charged to renew the token's expiration.
    pub fn auto_renew_account_id(&mut self, auto_renew_account_id: AccountId) -> &mut Self {
        self.data_mut().auto_renew_account_id = Some(auto_renew_account_id);
        self
    }

    /// Returns the new interval at which the auto renew account will be charged to extend the token's expiry.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Option<Duration> {
        self.data().auto_renew_period
    }

    /// Sets the new interval at which the auto renew account will be charged to extend
    /// the token's expiry.
    pub fn auto_renew_period(&mut self, auto_renew_period: Duration) -> &mut Self {
        self.data_mut().auto_renew_period = Some(auto_renew_period);
        self
    }

    /// Returns the new time at which the token should expire.
    #[must_use]
    pub fn get_expiration_time(&self) -> Option<OffsetDateTime> {
        self.data().expiration_time
    }

    /// Sets the new time at which the token should expire.
    ///
    /// If the new expiration time is earlier than the current expiration time, transaction
    /// will resolve to `InvalidExpirationTime`.
    pub fn expiration_time(&mut self, expiration_time: OffsetDateTime) -> &mut Self {
        let data = self.data_mut();
        data.expiration_time = Some(expiration_time);
        data.auto_renew_period = None;

        self
    }

    /// Returns the new memo associated with the token.
    #[must_use]
    pub fn get_token_memo(&self) -> &str {
        &self.data().token_memo
    }

    /// Sets the new memo associated with the token.
    ///
    /// Maximum of 100 bytes.
    pub fn token_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().token_memo = memo.into();
        self
    }

    /// Returns the new key which can change the token's custom fee schedule.
    #[must_use]
    pub fn get_fee_schedule_key(&self) -> Option<&Key> {
        self.data().fee_schedule_key.as_ref()
    }

    /// Sets the new key which can change the token's custom fee schedule.
    ///
    /// If the token does not currently have a fee schedule key, transaction will resolve to
    /// `TokenHasNoFeeScheduleKey`.
    pub fn fee_schedule_key(&mut self, fee_schedule_key: impl Into<Key>) -> &mut Self {
        self.data_mut().fee_schedule_key = Some(fee_schedule_key.into());
        self
    }

    /// Returns the new key which can pause and unpause the token.
    #[must_use]
    pub fn get_pause_key(&self) -> Option<&Key> {
        self.data().pause_key.as_ref()
    }

    /// Sets the new key which can pause and unpause the Token.
    ///
    /// If the token does not currently have a pause key, transaction will resolve to `TokenHasNoPauseKey`.
    pub fn pause_key(&mut self, pause_key: impl Into<Key>) -> &mut Self {
        self.data_mut().pause_key = Some(pause_key.into());
        self
    }
}

impl TransactionData for TokenUpdateTransactionData {}

impl TransactionExecute for TokenUpdateTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { TokenServiceClient::new(channel).update_token(request).await })
    }
}

impl ValidateChecksums for TokenUpdateTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.token_id.validate_checksums(ledger_id)?;
        self.auto_renew_account_id.validate_checksums(ledger_id)?;
        self.treasury_account_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TokenUpdateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::TokenUpdate(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TokenUpdateTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::TokenUpdate(self.to_protobuf())
    }
}

impl From<TokenUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: TokenUpdateTransactionData) -> Self {
        Self::TokenUpdate(transaction)
    }
}

impl FromProtobuf<services::TokenUpdateTransactionBody> for TokenUpdateTransactionData {
    fn from_protobuf(pb: services::TokenUpdateTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            token_id: Option::from_protobuf(pb.token)?,
            token_name: pb.name,
            token_symbol: pb.symbol,
            treasury_account_id: Option::from_protobuf(pb.treasury)?,
            admin_key: Option::from_protobuf(pb.admin_key)?,
            kyc_key: Option::from_protobuf(pb.kyc_key)?,
            freeze_key: Option::from_protobuf(pb.freeze_key)?,
            wipe_key: Option::from_protobuf(pb.wipe_key)?,
            supply_key: Option::from_protobuf(pb.supply_key)?,
            auto_renew_account_id: Option::from_protobuf(pb.auto_renew_account)?,
            auto_renew_period: pb.auto_renew_period.map(Into::into),
            expiration_time: pb.expiry.map(Into::into),
            token_memo: pb.memo.unwrap_or_default(),
            fee_schedule_key: Option::from_protobuf(pb.fee_schedule_key)?,
            pause_key: Option::from_protobuf(pb.pause_key)?,
        })
    }
}

impl ToProtobuf for TokenUpdateTransactionData {
    type Protobuf = services::TokenUpdateTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TokenUpdateTransactionBody {
            token: self.token_id.to_protobuf(),
            name: self.token_name.clone(),
            symbol: self.token_symbol.clone(),
            treasury: self.treasury_account_id.to_protobuf(),
            admin_key: self.admin_key.to_protobuf(),
            kyc_key: self.kyc_key.to_protobuf(),
            freeze_key: self.freeze_key.to_protobuf(),
            wipe_key: self.wipe_key.to_protobuf(),
            supply_key: self.supply_key.to_protobuf(),
            expiry: self.expiration_time.map(Into::into),
            auto_renew_account: self.auto_renew_account_id.to_protobuf(),
            auto_renew_period: self.auto_renew_period.map(Into::into),
            memo: Some(self.token_memo.clone()),
            fee_schedule_key: self.fee_schedule_key.to_protobuf(),
            pause_key: self.pause_key.to_protobuf(),
        }
    }
}