use std::sync::Arc;

use jsonrpsee::RpcModule;

use sp_blockchain::{Error as BlockchainError, HeaderBackend, HeaderMetadata};
use sc_transaction_pool_api::TransactionPool;
use sp_block_builder::BlockBuilder;
use sp_api::ProvideRuntimeApi;

pub use sc_rpc_api::DenyUnsafe;

use serai_runtime::{opaque::Block, AccountId, Balance, Index};

pub struct FullDeps<C, P> {
  pub client: Arc<C>,
  pub pool: Arc<P>,
  pub deny_unsafe: DenyUnsafe
}

pub fn create_full<
  C: ProvideRuntimeApi<Block> +
     HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockchainError> +
     Send + Sync + 'static,
  P: TransactionPool + 'static
>(
  deps: FullDeps<C, P>
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
  where C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index> +
                pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance> +
                BlockBuilder<Block> {

  use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
  use substrate_frame_rpc_system::{System, SystemApiServer};

  let mut module = RpcModule::new(());
  let FullDeps { client, pool, deny_unsafe } = deps;

  module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
  module.merge(TransactionPayment::new(client).into_rpc())?;

  Ok(module)
}