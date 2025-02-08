use std::str::FromStr;

use alloy::{providers::ProviderBuilder, sol, transports::http::reqwest::Url, network::EthereumWallet};
use eyre::Result;

sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc Inbox.sol --via-ir --optimize --bin
    #[sol(rpc, bytecode="60808060405234601557610381908161001a8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c9081634972134a146102a957508063ec8539ae1461017b578063f9acb4b61461007e5763fd1190ea14610048575f80fd5b3461007a57602036600319011261007a576004355f526001602052602060018060a01b0360405f205416604051908152f35b5f80fd5b3461007a57604036600319011261007a5760043567ffffffffffffffff811161007a576100af9036906004016102e8565b6024356001600160a01b0381169081900361007a575f545f52600160205260405f20816bffffffffffffffffffffffff60a01b8254161790555f549160405191606083018484526060602085015282518091526020608085019301905f5b81811061016557867f832df7297f330d630ec907f8c644fd951cbfc3d4cbb2f9946651bbbc064b4aed8780888860408301520390a15f198114610151576001015f55005b634e487b7160e01b5f52601160045260245ffd5b825185526020948501949092019160010161010d565b3461007a57604036600319011261007a5760043567ffffffffffffffff811161007a576101ac9036906004016102e8565b5060243567ffffffffffffffff811161007a573660238201121561007a5780600401359067ffffffffffffffff8211610295576101f2601f8301601f19166020016102c2565b91808352366024828401011161007a575f9281602460209401848301370101525f805480825260016020526040909120546001600160a01b0316330361025f5760207f1314e2f660aa082153647cd3e7f838d7c61acf4efd72f449715fae60d6868b6991604051908152a1005b60405162461bcd60e51b815260206004820152600e60248201526d24b73b30b634b210383937bb32b960911b6044820152606490fd5b634e487b7160e01b5f52604160045260245ffd5b3461007a575f36600319011261007a576020905f548152f35b6040519190601f01601f1916820167ffffffffffffffff81118382101761029557604052565b9080601f8301121561007a5781359167ffffffffffffffff8311610295578260051b906020806103198185016102c2565b80968152019282010192831161007a57602001905b82821061033b5750505090565b813581526020918201910161032e56fea2646970667358221220163ac6bfa3738322486f91688e08b1a1f84def75ce7212bc0928e65f6766b4bc64736f6c634300081c0033")]
    contract Inbox {
        uint256 public batchId;
    
        mapping(uint256 => address) public provers;
    
        event BatchProposed(uint256 batchId, bytes32[] batch, address prover);
        event BatchProved(uint256 batchId);
    
        function proposeBatch(bytes32[] memory batch, address prover) public {
            provers[batchId] = prover;
            emit BatchProposed(batchId, batch, prover);
            batchId++;
        }
    
        function proveBatch(bytes32[] memory batch, bytes memory proof) public {
            require(provers[batchId] == msg.sender, "Invalid prover");
            require(_verifyBatch(batch, proof), "Invalid proof");
            emit BatchProved(batchId);
        }
    
        function _verifyBatch(bytes32[] memory _batch, bytes memory _proof) private pure returns (bool) {
            return true;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
  let wallet = EthereumWallet::from(&std::env::var("ETH_PRIVATE_KEY").unwrap());
  let url = Url::from_str("https://rpc.sepolia.org").unwrap();
  let provider = ProviderBuilder::new()
    .wallet(wallet)
    .on_http(url)

  let contract = Inbox::deploy(&provider).await?;

  println!("Deployed contract at address: {}", contract.address());
  Ok(())
}