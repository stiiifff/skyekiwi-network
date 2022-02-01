use skw_vm_primitives::contract_runtime::{
    AccountId, BlockNumber, MerkleHash, CryptoHash, ContractCode,
};
use skw_vm_primitives::account::{Account, AccessKey};
use skw_vm_primitives::views::ViewStateResult;
use skw_vm_primitives::crypto::PublicKey;

/// Adapter for querying runtime.
pub trait ViewRuntimeAdapter {
    fn view_account(
        &self,
        state_root: MerkleHash,
        account_id: &AccountId,
    ) -> Result<Account, crate::state_viewer::errors::ViewAccountError>;

    fn view_contract_code(
        &self,
        state_root: MerkleHash,
        account_id: &AccountId,
    ) -> Result<ContractCode, crate::state_viewer::errors::ViewContractCodeError>;

    fn call_function(
        &self,
        state_root: MerkleHash,
        block_number: BlockNumber,
        block_timestamp: u64,
        last_block_hash: &CryptoHash,
        block_hash: &CryptoHash,
        contract_id: &AccountId,
        method_name: &str,
        args: &[u8],
        logs: &mut Vec<String>,
        // current_protocol_version: ProtocolVersion,
    ) -> Result<Vec<u8>, crate::state_viewer::errors::CallFunctionError>;

    fn view_access_key(
        &self,
        state_root: MerkleHash,
        account_id: &AccountId,
        public_key: &PublicKey,
    ) -> Result<AccessKey, crate::state_viewer::errors::ViewAccessKeyError>;

    fn view_access_keys(
        &self,
        state_root: MerkleHash,
        account_id: &AccountId,
    ) -> Result<Vec<(PublicKey, AccessKey)>, crate::state_viewer::errors::ViewAccessKeyError>;

    fn view_state(
        &self,
        state_root: MerkleHash,
        account_id: &AccountId,
        prefix: &[u8],
    ) -> Result<ViewStateResult, crate::state_viewer::errors::ViewStateError>;
}
