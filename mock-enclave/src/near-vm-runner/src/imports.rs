use wasmi::{
  Error as InterpeterError, FuncInstance, FuncRef, ImportsBuilder, ModuleImportResolver,
  Signature, ValueType,
};

use crate::externals::HostFunctions;

pub fn create_builder(resolver: &dyn ModuleImportResolver) -> ImportsBuilder {
  ImportsBuilder::new.with_resolver("env", resolver)
}

#[derive(Debug, Clone)]
pub struct WasmiImportResolver {}

// https://paritytech.github.io/wasmi/wasmi/trait.Externals.html

impl ModuleImportResolver for WasmiImportResolver {
  fn resolve_func(
    &self, 
    func_name: &str,
    _signature: &Signature,
  ) -> Result<FuncRef, InterpeterError> {
    match func_name {
      "read_register" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64][..], None),
        HostFunctions::ReadRegister.into(),
      ),
      "register_len" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::RegisterLen.into(),
      ),
      "write_register" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64][..] , None),
        HostFunctions::WriteRegister.into(),
      ),
      "current_account_id" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..], None),
        HostFunctions::CurrentAccountId.into(),
      ),
      "signer_account_id" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..], None),
        HostFunctions::SignerAccountId.into(),
      ),
      "signer_account_pk" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..] , None),
        HostFunctions::SignerAccountPublicKey.into(),
      ),
      "predecessor_account_id" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..], None),
        HostFunctions::PredecessorAccountId.into(),
      ),
      "input" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..], None),
        HostFunctions::Input.into(),
      ),
      "block_number" => FuncInstance::alloc_host(
        Signature::new(&[][..], Some(ValueType::I64)),
        HostFunctions::BlockNumber.into(),
      ),
      "block_timestamp" => FuncInstance::alloc_host(
        Signature::new(&[][..], Some(ValueType::I64)),
        HostFunctions::BlockTimestamp.into(),
      ),
      "epoch_height" => FuncInstance::alloc_host(
        Signature::new(&[][..], Some(ValueType::I64)),
        HostFunctions::EpochHeight.into(),
      ),
      "storage_usage" => FuncInstance::alloc_host(
        Signature::new(&[][..], Some(ValueType::I64)),
        HostFunctions::StorageUsage.into(),
      ),
      "account_balance" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..], None),
        HostFunctions::AccountBalance.into(),
      ),
      "attached_deposit" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..], None),
        HostFunctions::AttachedDeposit.into(),
      ),
      "prepaid_gas" => FuncInstance::alloc_host(
        Signature::new(&[][..], Some(ValueType::I64)),
        HostFunctions::PrepaidGas.into(),
      ),
      "used_gas" => FuncInstance::alloc_host(
        Signature::new(&[][..], Some(ValueType::I64)),
        HostFunctions::UsedGas.into(),
      ),
      "random_seed" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..], None),
        HostFunctions::RandomSeed.into(),
      ),
      "sha256" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64][..], None),
        HostFunctions::Sha256.into(),
      ),
      "keccak256" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64][..], None),
        HostFunctions::Keccak256.into(),
      ),
      "keccak512" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64][..], None),
        HostFunctions::Keccak512.into(),
      ),
      "ripemd160" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64][..], None),
        HostFunctions::Ripemd160.into(),
      ),
      "ecrecover" => FuncInstance::alloc_host(
        Signature::new(&[ValueType:: I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::Ecrecover.into(),
      ),
      "value_return" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64][..], None),
        HostFunctions::ValueReturn.into(),
      ),
      "panic" => FuncInstance::alloc_host(
        Signature::new(&[][..], None),
        HostFunctions::Panic.into(),
      ),
      "panic_utf8" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64][..], None),
        HostFunctions::PanicUtf8.into(),
      ),
      "log_utf8" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64][..], None),
        HostFunctions::LogUtf8.into(),
      ),
      "log_utf16" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64][..], None),
        HostFunctions::LogUtf16.into(),
      ),
      "abort" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::Iu32, ValueType::Iu32, ValueType::I32, ValueType::I32][..], None),
        HostFunctions::Abort.into(),
      ),
      "promise_create" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::PromiseCreate.into(),
      ),
      "promise_then" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::PromiseThen.into(),
      ),
      "promise_and" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::PromiseAnd.into(),
      ),
      "promise_batch_create" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::PromiseBatchCreate.into(),
      ),
      "promise_batch_then" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::PromiseBatchThen.into(),
      ),
      "promise_batch_action_create_account" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..], None),
        HostFunctions::PromiseBatchActionCreateAccount.into(),
      ),
      "promise_batch_action_deploy_contract" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64][..], None),
        HostFunctions::PromiseBatchActionDeployContract.into(),
      ),
      "promise_batch_action_function_call" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64][..], None),
        HostFunctions::PromiseBatchActionFunctionCall.into(),
      ),
      "promise_batch_action_transfer" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64][..], None),
        HostFunctions::PromiseBatchActionTransfer.into(),
      ),
      "promise_batch_action_delete_account" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64][..], None),
        HostFunctions::PromiseBatchActionDeleteAccount.into(),
      ),
      "promise_results_count" => FuncInstance::alloc_host(
        Signature::new(&[][..], Some(ValueType::I64)),
        HostFunctions::PromiseResultsCount.into(),
      ),
      "promise_result" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::PromiseResult.into(),
      ),
      "promise_return" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64][..], None),
        HostFunctions::PromiseReturn.into(),
      ),
      "storage_write" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::StorageWrite.into(),
      ),
      "storage_read" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::StorageRead.into(),
      ),
      "storage_remove" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::StorageRemove.into(),
      ),
      "storage_has_key" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I64, ValueType::I64][..], Some(ValueType::I64)),
        HostFunctions::StorageHasKey.into(),
      ),
      "gas" => FuncInstance::alloc_host(
        Signature::new(&[ValueType::I32][..], None),
        HostFunctions::Gas.into(),
      ),
    }
  }
}
