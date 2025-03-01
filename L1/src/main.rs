use alloy::{
    network::EthereumWallet, node_bindings::Anvil, providers::ProviderBuilder,
    signers::local::PrivateKeySigner, sol,
};
use dotenv::dotenv;
use eyre::Result;
use std::str::FromStr;

sol! {
    #[allow(missing_docs)]
    // solc v0.8.28; solc src/sol/Inbox.sol --bin
    #[sol(rpc, bytecode="6080604052348015600e575f5ffd5b50610eb78061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061004a575f3560e01c80634972134a1461004e57806356ffeca01461006c5780635d47964b14610088578063fd237972146100b8575b5f5ffd5b6100566100d4565b604051610063919061046b565b60405180910390f35b610086600480360381019061008191906106b3565b6100d9565b005b6100a2600480360381019061009d9190610724565b610153565b6040516100af91906107c2565b60405180910390f35b6100d260048036038101906100cd91906107e2565b610204565b005b5f5481565b8060015f5f5481526020019081526020015f2090805190602001906100ff929190610381565b507f0a5cd30ae5adf36ea809b1efc13fc876f3a47861f01bda8b4b44c6e35ce0b1dd5f548260405161013292919061093f565b60405180910390a15f5f81548092919061014b9061099a565b919050555050565b6001602052815f5260405f20818154811061016c575f80fd5b905f5260205f20015f9150915050805461018590610a0e565b80601f01602080910402602001604051908101604052809291908181526020018280546101b190610a0e565b80156101fc5780601f106101d3576101008083540402835291602001916101fc565b820191905f5260205f20905b8154815290600101906020018083116101df57829003601f168201915b505050505081565b6102e960015f8481526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b828210156102df578382905f5260205f2001805461025490610a0e565b80601f016020809104026020016040519081016040528092919081815260200182805461028090610a0e565b80156102cb5780601f106102a2576101008083540402835291602001916102cb565b820191905f5260205f20905b8154815290600101906020018083116102ae57829003601f168201915b505050505081526020019060010190610237565b5050505082610376565b610328576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161031f90610a98565b60405180910390fd5b7f3617c629ef908dd3c9bea65de314e12661cc7506881d05347ec3b7c55931ef545f5460015f8581526020019081526020015f2060405161036a929190610bf6565b60405180910390a15050565b5f6001905092915050565b828054828255905f5260205f209081019282156103c7579160200282015b828111156103c65782518290816103b69190610db2565b509160200191906001019061039f565b5b5090506103d491906103d8565b5090565b5b808211156103f7575f81816103ee91906103fb565b506001016103d9565b5090565b50805461040790610a0e565b5f825580601f106104185750610435565b601f0160209004905f5260205f20908101906104349190610438565b5b50565b5b8082111561044f575f815f905550600101610439565b5090565b5f819050919050565b61046581610453565b82525050565b5f60208201905061047e5f83018461045c565b92915050565b5f604051905090565b5f5ffd5b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6104df82610499565b810181811067ffffffffffffffff821117156104fe576104fd6104a9565b5b80604052505050565b5f610510610484565b905061051c82826104d6565b919050565b5f67ffffffffffffffff82111561053b5761053a6104a9565b5b602082029050602081019050919050565b5f5ffd5b5f5ffd5b5f67ffffffffffffffff82111561056e5761056d6104a9565b5b61057782610499565b9050602081019050919050565b828183375f83830152505050565b5f6105a461059f84610554565b610507565b9050828152602081018484840111156105c0576105bf610550565b5b6105cb848285610584565b509392505050565b5f82601f8301126105e7576105e6610495565b5b81356105f7848260208601610592565b91505092915050565b5f61061261060d84610521565b610507565b905080838252602082019050602084028301858111156106355761063461054c565b5b835b8181101561067c57803567ffffffffffffffff81111561065a57610659610495565b5b80860161066789826105d3565b85526020850194505050602081019050610637565b5050509392505050565b5f82601f83011261069a57610699610495565b5b81356106aa848260208601610600565b91505092915050565b5f602082840312156106c8576106c761048d565b5b5f82013567ffffffffffffffff8111156106e5576106e4610491565b5b6106f184828501610686565b91505092915050565b61070381610453565b811461070d575f5ffd5b50565b5f8135905061071e816106fa565b92915050565b5f5f6040838503121561073a5761073961048d565b5b5f61074785828601610710565b925050602061075885828601610710565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f61079482610762565b61079e818561076c565b93506107ae81856020860161077c565b6107b781610499565b840191505092915050565b5f6020820190508181035f8301526107da818461078a565b905092915050565b5f5f604083850312156107f8576107f761048d565b5b5f61080585828601610710565b925050602083013567ffffffffffffffff81111561082657610825610491565b5b610832858286016105d3565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f61087f82610762565b6108898185610865565b935061089981856020860161077c565b6108a281610499565b840191505092915050565b5f6108b88383610875565b905092915050565b5f602082019050919050565b5f6108d68261083c565b6108e08185610846565b9350836020820285016108f285610856565b805f5b8581101561092d578484038952815161090e85826108ad565b9450610919836108c0565b925060208a019950506001810190506108f5565b50829750879550505050505092915050565b5f6040820190506109525f83018561045c565b818103602083015261096481846108cc565b90509392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6109a482610453565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036109d6576109d561096d565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680610a2557607f821691505b602082108103610a3857610a376109e1565b5b50919050565b5f82825260208201905092915050565b7f496e76616c69642070726f6f66000000000000000000000000000000000000005f82015250565b5f610a82600d83610a3e565b9150610a8d82610a4e565b602082019050919050565b5f6020820190508181035f830152610aaf81610a76565b9050919050565b5f81549050919050565b5f819050815f5260205f209050919050565b5f819050815f5260205f209050919050565b5f8154610af081610a0e565b610afa8186610865565b9450600182165f8114610b145760018114610b2a57610b5c565b60ff198316865281151560200286019350610b5c565b610b3385610ad2565b5f5b83811015610b5457815481890152600182019150602081019050610b35565b808801955050505b50505092915050565b5f610b708383610ae4565b905092915050565b5f600182019050919050565b5f610b8e82610ab6565b610b988185610846565b935083602082028501610baa85610ac0565b805f5b85811015610be457848403895281610bc58582610b65565b9450610bd083610b78565b925060208a01995050600181019050610bad565b50829750879550505050505092915050565b5f604082019050610c095f83018561045c565b8181036020830152610c1b8184610b84565b90509392505050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302610c6e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82610c33565b610c788683610c33565b95508019841693508086168417925050509392505050565b5f819050919050565b5f610cb3610cae610ca984610453565b610c90565b610453565b9050919050565b5f819050919050565b610ccc83610c99565b610ce0610cd882610cba565b848454610c3f565b825550505050565b5f5f905090565b610cf7610ce8565b610d02818484610cc3565b505050565b5b81811015610d2557610d1a5f82610cef565b600181019050610d08565b5050565b601f821115610d6a57610d3b81610ad2565b610d4484610c24565b81016020851015610d53578190505b610d67610d5f85610c24565b830182610d07565b50505b505050565b5f82821c905092915050565b5f610d8a5f1984600802610d6f565b1980831691505092915050565b5f610da28383610d7b565b9150826002028217905092915050565b610dbb82610762565b67ffffffffffffffff811115610dd457610dd36104a9565b5b610dde8254610a0e565b610de9828285610d29565b5f60209050601f831160018114610e1a575f8415610e08578287015190505b610e128582610d97565b865550610e79565b601f198416610e2886610ad2565b5f5b82811015610e4f57848901518255600182019150602085019450602081019050610e2a565b86831015610e6c5784890151610e68601f891682610d7b565b8355505b6001600288020188555050505b50505050505056fea2646970667358221220c37aa8319026130ace6f90a72f8faf4953ac0c7d7959c1bc80785d1dde82daa464736f6c634300081c0033")]
    contract Inbox {
      uint256 public batchId;

      mapping(uint256 => bytes[]) public batches;

      event BatchProposed(uint256 batchId, bytes[] batchData);
      event BatchProved(uint256 batchId, bytes[] batchData);

      // anyone can propose a batch
      function proposeBatch(bytes[] calldata batchData) public {
          batches[batchId] = batchData;
          emit BatchProposed(batchId, batchData);
          batchId++;
      }

      // anyone can prove a batch
      function proveBatch(uint256 id, bytes memory proof) public {
          require(_verifyBatch(batches[id], proof), "Invalid proof");
          emit BatchProved(batchId, batches[id]);
      }

      function _verifyBatch(bytes[] memory _batch, bytes memory _proof) private pure returns (bool) {
          // TODO: implement proof verification.
          return true;
      }
  }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // run node
    let l1_port_str = std::env::var("L1_PORT").unwrap();
    let l1_port = u16::from_str(&l1_port_str).unwrap();
    let anvil = Anvil::new()
        .keep_stdout()
        .port(l1_port)
        .block_time(1)
        .try_spawn()
        .unwrap();

    println!("running L1 node in port {:?} 🛜", anvil.port());

    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    let rpc_url = anvil.endpoint_url();
    let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
    let contract = Inbox::deploy(&provider).await?;

    println!("Deployed L1 Inbox contract at address: {} 🚀", contract.address());
    // Keep anvil alive
    println!("Press Ctrl+C to exit");
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");
    Ok(())
}
