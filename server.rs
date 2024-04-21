use {
    super::*,
    base64::Engine,
    bitcoin::{consensus::Decodable, psbt::Psbt, Witness},
    std::io::Cursor,
  };
  
  pub(crate) struct Server {
    pub(crate) state: Arc<Mutex<State>>,
    pub(crate) network: Network,
  }
  
  impl Server {
    pub(crate) fn new(state: Arc<Mutex<State>>) -> Self {
      let network = state.lock().unwrap().network;
      Self { network, state }
    }
  
    fn state(&self) -> MutexGuard<State> {
      self.state.lock().unwrap()
    }
  
    fn not_found() -> jsonrpc_core::Error {
      jsonrpc_core::Error::new(jsonrpc_core::types::error::ErrorCode::ServerError(-8))
    }
  }
  
  impl Api for Server {
    fn get_balances(&self) -> Result<GetBalancesResult, jsonrpc_core::Error> {
      Ok(GetBalancesResult {
        mine: GetBalancesResultEntry {
          immature: Amount::from_sat(0),
          trusted: self
            .list_unspent(None, None, None, None, None)?
            .iter()
            .map(|entry| entry.amount)
            .sum(),
          untrusted_pending: Amount::from_sat(0),
        },
        watchonly: None,
      })
    }
  
    fn get_best_block_hash(&self) -> Result<BlockHash, jsonrpc_core::Error> {
      match self.state().hashes.last() {
        Some(block_hash) => Ok(*block_hash),
        None => Err(Self::not_found()),
      }
    }
  
    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResult, jsonrpc_core::Error> {
      Ok(GetBlockchainInfoResult {
        chain: String::from(match self.network {
          Network::Bitcoin => "main",
          Network::Testnet => "test",
          Network::Signet => "signet",
          Network::Regtest => "regtest",
          _ => panic!(),
        }),
        blocks: 0,
        headers: 0,
        best_block_hash: self.state().hashes[0],
        difficulty: 0.0,
        median_time: 0,
        verification_progress: 0.0,
        initial_block_download: false,
        chain_work: Vec::new(),
        size_on_disk: 0,
        pruned: false,
        prune_height: None,
        automatic_pruning: None,
        prune_target_size: None,
        softforks: HashMap::new(),
        warnings: String::new(),
      })
    }
  
  ...