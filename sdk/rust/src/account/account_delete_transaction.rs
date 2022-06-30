use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountAddress, AccountId, Transaction};

/// Mark an account as deleted, moving all its current hbars to another account.
///
/// It will remain in the ledger, marked as deleted, until it expires.
/// Transfers into it a deleted account will fail.
///
pub type AccountDeleteTransaction = Transaction<AccountDeleteTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDeleteTransactionData {
    /// The account ID which will receive all remaining hbars.
    pub transfer_account_id: Option<AccountAddress>,

    /// The account ID which should be deleted.
    pub delete_account_id: Option<AccountAddress>,
}

impl AccountDeleteTransaction {
    /// Sets the account ID which should be deleted.
    pub fn delete_account_id(&mut self, id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.delete_account_id = Some(id.into());
        self
    }

    /// Sets the account ID which will receive all remaining hbars.
    pub fn transfer_account_id(&mut self, id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.transfer_account_id = Some(id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for AccountDeleteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).crypto_delete(request).await
    }
}

impl ToTransactionDataProtobuf for AccountDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let delete_account_id = self.delete_account_id.as_ref().map(AccountAddress::to_protobuf);
        let transfer_account_id =
            self.transfer_account_id.as_ref().map(AccountAddress::to_protobuf);

        services::transaction_body::Data::CryptoDelete(services::CryptoDeleteTransactionBody {
            delete_account_id,
            transfer_account_id,
        })
    }
}

impl From<AccountDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: AccountDeleteTransactionData) -> Self {
        Self::AccountDelete(transaction)
    }
}

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use crate::{AccountAddress, AccountDeleteTransaction, AccountId};
    use crate::transaction::{AnyTransaction, AnyTransactionData};

    // language=JSON
    const ACCOUNT_DELETE_TRANSACTION_JSON: &str = r#"{
  "$type": "accountDelete",
  "transferAccountId": "0.0.1001",
  "deleteAccountId": "0.0.1002"
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = AccountDeleteTransaction::new();

        transaction
            .transfer_account_id(AccountId::from(1001))
            .delete_account_id(AccountId::from(1002));

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, ACCOUNT_DELETE_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(ACCOUNT_DELETE_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::AccountDelete(transaction) => transaction);

        let transfer_account_id = assert_matches!(data.transfer_account_id.unwrap(), AccountAddress::AccountId(account_id) => account_id);
        assert_eq!(transfer_account_id, AccountId::from(1001));

        let delete_account_id = assert_matches!(data.delete_account_id.unwrap(), AccountAddress::AccountId(account_id) => account_id);
        assert_eq!(delete_account_id, AccountId::from(1002));

        Ok(())
    }
}
