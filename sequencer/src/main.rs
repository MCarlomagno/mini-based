use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use alloy::primitives::{Address, Bytes};
use alloy::providers::ProviderBuilder;
use alloy::sol;
use alloy::transports::http::reqwest::Url;
use dotenv::dotenv;
use alloy::{
  network::EthereumWallet,
  signers::local::PrivateKeySigner,
};

sol! {
    #[allow(missing_docs)]
    // solc v0.8.28; solc src/sol/Inbox.sol --bin
    #[sol(rpc, bytecode="608060405234601c57600e6020565b610a2461002b8239610a2490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61058d565b61001d5f3561005c565b80634972134a14610057578063b32c4d8d14610052578063f6b8e4811461004d5763fd2379720361000e57610559565b610440565b610385565b6100df565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261007a57565b61006c565b1c90565b90565b61009690600861009b930261007f565b610083565b90565b906100a99154610086565b90565b6100b75f5f9061009e565b90565b90565b6100c6906100ba565b9052565b91906100dd905f602085019401906100bd565b565b3461010f576100ef366004610070565b61010b6100fa6100ac565b610102610062565b918291826100ca565b0390f35b610068565b5f80fd5b610121816100ba565b0361012857565b5f80fd5b9050359061013982610118565b565b9060208282031261015457610151915f0161012c565b90565b61006c565b90565b61017061016b610175926100ba565b610159565b6100ba565b90565b906101829061015c565b5f5260205260405f2090565b634e487b7160e01b5f525f60045260245ffd5b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156101d5575b60208310146101d057565b6101a1565b91607f16916101c5565b60209181520190565b5f5260205f2090565b905f929180549061020b610204836101b5565b80946101df565b916001811690815f146102625750600114610226575b505050565b61023391929394506101e8565b915f925b81841061024a57505001905f8080610221565b60018160209295939554848601520191019290610237565b92949550505060ff19168252151560200201905f8080610221565b90610287916101f1565b90565b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906102b29061028a565b810190811067ffffffffffffffff8211176102cc57604052565b610294565b906102f16102ea926102e1610062565b9384809261027d565b03836102a8565b565b905f1061030657610303906102d1565b90565b61018e565b6103219061031c6001915f92610178565b6102f3565b90565b5190565b60209181520190565b90825f9392825e0152565b61035b6103646020936103699361035281610324565b93848093610328565b95869101610331565b61028a565b0190565b6103829160208201915f81840391015261033c565b90565b346103b5576103b16103a061039b36600461013b565b61030b565b6103a8610062565b9182918261036d565b0390f35b610068565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156104005781359167ffffffffffffffff83116103fb5760200192600183028401116103f657565b6103c2565b6103be565b6103ba565b90602082820312610436575f82013567ffffffffffffffff81116104315761042d92016103c6565b9091565b610114565b61006c565b5f0190565b3461046f57610459610453366004610405565b9061085a565b610461610062565b8061046b8161043b565b0390f35b610068565b5f80fd5b9061048b610484610062565b92836102a8565b565b67ffffffffffffffff81116104ab576104a760209161028a565b0190565b610294565b90825f939282370152565b909291926104d06104cb8261048d565b610478565b938185526020850190828401116104ec576104ea926104b0565b565b610474565b9080601f8301121561050f5781602061050c933591016104bb565b90565b6103ba565b9190916040818403126105545761052d835f830161012c565b92602082013567ffffffffffffffff811161054f5761054c92016104f1565b90565b610114565b61006c565b346105885761057261056c366004610514565b90610972565b61057a610062565b806105848161043b565b0390f35b610068565b5f80fd5b5f1c90565b6105a26105a791610591565b610083565b90565b6105b49054610596565b90565b5090565b601f602091010490565b1b90565b919060086105e49102916105de5f19846105c5565b926105c5565b9181191691161790565b90565b919061060761060261060f9361015c565b6105ee565b9083546105c9565b9055565b5f90565b61062991610623610613565b916105f1565b565b5b818110610637575050565b806106445f600193610617565b0161062c565b9190601f811161065a575b505050565b61066661068b936101e8565b906020610672846105bb565b83019310610693575b610684906105bb565b019061062b565b5f8080610655565b91506106848192905061067b565b906106b1905f199060080261007f565b191690565b816106c0916106a1565b906002021790565b916106d390826105b7565b9067ffffffffffffffff8211610792576106f7826106f185546101b5565b8561064a565b5f90601f831160011461072a57918091610719935f9261071e575b50506106b6565b90555b565b90915001355f80610712565b601f19831691610739856101e8565b925f5b81811061077a57509160029391856001969410610760575b5050500201905561071c565b610770910135601f8416906106a1565b90555f8080610754565b9193602060018192878701358155019501920161073c565b610294565b906107a292916106c8565b565b91906107be816107b7816107c395610328565b80956104b0565b61028a565b0190565b916107ec9391926107df60408201945f8301906100bd565b60208185039101526107a4565b90565b634e487b7160e01b5f52601160045260245ffd5b61080c906100ba565b5f19811461081a5760010190565b6107ef565b5f1b90565b906108305f199161081f565b9181191691161790565b9061084f61084a6108569261015c565b6105ee565b8254610824565b9055565b9061087a828290610875600161086f5f6105aa565b90610178565b610797565b6108835f6105aa565b9190916108bc7f72a5f12a7972245e884f3a2c93467e52ee2ae2579fb04c769ff0c0e57e3494e6936108b3610062565b938493846107c7565b0390a16108d96108d36108ce5f6105aa565b610803565b5f61083a565b565b6108e4906102d1565b90565b60209181520190565b5f7f496e76616c69642070726f6f6600000000000000000000000000000000000000910152565b610924600d6020926108e7565b61092d816108f0565b0190565b6109469060208101905f818303910152610917565b90565b1561095057565b610958610062565b62461bcd60e51b81528061096e60048201610931565b0390fd5b6109969161098c610987610991936001610178565b6108db565b6109de565b610949565b61099f5f6105aa565b6109d57f1314e2f660aa082153647cd3e7f838d7c61acf4efd72f449715fae60d6868b69916109cc610062565b918291826100ca565b0390a1565b5f90565b50506109e86109da565b5060019056fea264697066735822122040a409aced86a6ed8f46a28a12a0fd8582e0f6f6836169236a96ddaee7a7887a64736f6c634300081c0033")]
    contract Inbox {
      uint256 public batchId;

      mapping(uint256 => bytes) public batches;
  
      event BatchProposed(uint256 batchId, bytes batchData);
      event BatchProved(uint256 batchId);
  
      // anyone can propose a batch
      function proposeBatch(bytes calldata batchData) public {
          batches[batchId] = batchData;
          emit BatchProposed(batchId, batchData);
          batchId++;
      }
  
      // anyone can prove a batch
      function proveBatch(uint256 id, bytes memory proof) public {
          require(_verifyBatch(batches[id], proof), "Invalid proof");
          emit BatchProved(batchId);
      }
  
      function _verifyBatch(bytes memory _batch, bytes memory _proof) private pure returns (bool) {
          // TODO: implement proof verification.
          return true;
      }
    }
}

async fn send_batch(inbox_raw_address: &str, rpc_url: &str, batch: String) {
    dotenv().ok(); 
    let pk = &std::env::var("PRIVATE_KEY").unwrap();
    let signer: PrivateKeySigner = PrivateKeySigner::from_str(pk).unwrap();
    let wallet = EthereumWallet::from(signer);
    let inbox_address = Address::from_str(inbox_raw_address).unwrap();
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_http(Url::from_str(rpc_url).unwrap());

    let inbox = Inbox::new(inbox_address, provider);

    let batch_bytes = Bytes::from(hex::decode(batch.trim_start_matches("0x")).unwrap());
    let result = inbox
        .proposeBatch(batch_bytes)
        .send().await
        .unwrap();

    println!("propose batch call result {:?}", result)
}

#[tokio::main]
async fn main() {
    let rpc_url = "http://localhost:8545";
    let l1_inbox_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    let mut file = File::open("../node/mempool.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
 
    let transactions: Vec<String> = serde_json::from_str(&buff).unwrap();
    println!("Loaded {} transactions", transactions.len());

    let batch = transactions.iter()
        .take(10)
        .enumerate()
        .map(|(i, tx)| {
            if i == 0 {
                tx.clone()
            } else {
                tx.trim_start_matches("0x").to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("");

    send_batch(l1_inbox_address, rpc_url, batch).await;
}
