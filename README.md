<img src="https://tva1.sinaimg.cn/large/008i3skNgy1gqz4uri7ckj33dl0otn1c.jpg" width="600px"/>

**NOTE:** This is a very early version of the SkyeKiwi Network. It is in heavy development. We will not advise anyone to use in production environment yet.

<br>

<div>
    <img src="https://s6.jpg.cm/2021/10/26/IzgUTy.jpg" width="40%" align="left"/>
    <img src="https://s3.bmp.ovh/imgs/2022/02/f6c5695831043dbf.png" width="40%" align="left"/>
</div>
<br><br><br><br><br><br><br><br>

> **A fun background story behind our logo** <br/><br/>
> Little do people know that among all Greek letters, Sigma is a special one. Not only because it’s the Greek for S and S for  SkyeKiwi(duh..), but also because it’s the only Greek letter that can be written in three ways: uppercase “Σ”, lowercase “σ” and lowercase in word-final position “ς” and English likely adopt “ς” as “S” (they do look alike, right?). We make our logo to honor the Greeks’ letter Sigma but intentionally leave out the “ς” ( at a word-final position :) ), to keep this a secret (Shhhh... ). To read more on this fun Greek fact. [Link](https://en.wikipedia.org/wiki/Sigma)

## SkyeKiwi Network

The SkyeKiwi Network is a confidential smart contract execution blockchain based on the SkyeKiwi Protocol. 

## Components & Status

|Component Name|Description|Status|
|---|---|---|
|`crates/skw-blockchain-*`|The main Substrate based blockchain client|See Below|
|`crates/skw-vm-*`|The offchain NEAR compatible VM|Early Internal Alpha|
|`crates/skw-sgx-*`|SkyeKiwi Protocol in Intel SGX Enclave|Internal Alpha|
|`crates/skw-contract-*`|Secret Smart Contract SDK, a wild cousin of NEAR SDK|Early Internal Alpha|
|`crates/near-*`|NEAR Compatible Tests|N/A|
|`enclave/*`|Intel SGX Enclave for offchain runtime|Impl SkyeKiwi Protocol, Early Internal Alpha|
|`mock-enclave/*`|Mock offchain runtime connector|Not implemented yet|
|`skw-tools-chaos/*`|Chaos Party! Spawn blockchain with random calls|Not ready|
|`skw-tools-scripts/*`|CI Scripts/Build Scripts etc. |Up to date|
|`teaclave-sgx-sdk/*`|Submodule for Rust Intel SGX SDK|Up to date|



## Descriptions & Build & Testing Guide

Language Dependncies: `nodejs`(used to run scripts), `rust` (developing langauge) and `docker`. Any modern version of the tools should do. 

**Substrate Based Blockchain**

The SkyeKiwi Network blockchain is based on Substrate and currently contains 3 major pallets: 

​	`pallet-registry` for SecretKeeper Registration; Status: Internal Alpha

​	`pallet-secrets` for secret registration: Late Alpha

​	`pallet-s-contract` for an exposed interface to call secret contracts: Alpha

​	`pallet-parentchain` for syncing offchain blocks on chain: Alpha

There is not a `chain-spec` file yet to run a test-net. Est. Avaliable by the end of Feberuray 2022. 



**SkyeKiwi Offchain VM & Contract SDK**

The SkyeKiwi Network offchain VM, (mostly) compatible with the NEAR VM. It's generally a simplified version of the NEAR Protocol VM BUT 

- without staking & validator information (Status: Alpha Stage)
- use `wasmi` to be enclave friendly instead of `wasmer` or `wasmtime` (Status: Alpha Stage)
- Use Static Lrc Cache for compiled contracts (Status: Alpha Stage)
- use SGX Protected FIles for state instead of RocksDB (Status: Alpha Stage)
- Less strict account balance check (Status: Early Alpha)
- Allow arbitrary state patching bypassing all runtime checks. Note: the VM will run in SGX Enclave .. so this will still ba safe (Status: Alpha Stage) 

**For Contract SDK**

- Roughly exactly the same as the NEAR Contracts. Developer in the NEAR ecosystem can easily re-deploy the contract as a secret contract to the SkyeKiwi Network with little changes to the contract source code. (Status: Alpha Stage)
- SkyeKiwi VM Runtime Simulator (Status: Not Ready)



**SkyeKiwi Protocol in Intel SGX Enclave**

The SGX version of the SkyeKiwi Protocol follows the exact same formats as the client version of the [SkyeKiwi Protocol](https://github.com/skyekiwi/skyekiwi-protocol). However, implementations differs greatly, to reduce memory copies and referencing and better fit the SGX Enclave environment. 

As a result: 

- The default enclave heap size allocated is `0xf00000` bytes. As a result, it will be hard to process secrets larger than 1MB. The process might panic. Generally, the SkyeKiwi Protocol inside the SGX Enclave is designed to process secret smart contract states ... and they rarely reaches these much of storage usage either way. Plus, we always have the option to allocate more memory to the enclave. 
- The IPFS module does not comes with the automatic fallback function as the client side yet. Not a top priority for us yet. For testers, if your enclave tests failed because of `HttpErrors`. Try again.
- The typical `upstream` and `downstream` processing is now divided into three seperated steps: `pre-processing`, `encrypt-cid-list` and `post-processing`. We might blog about it later, or we might find a better arch for it later. Details below. 

**Building & Testing**

0. Make sure all Rust env are correctly installed. 

Here's the cheatsheet. 

To Install Rust:
```
curl https://sh.rustup.rs -sSf | sh -s -- -y
source ~/.cargo/env
```

Install target `wasm32`
```
rustup target add wasm32-unknown-unknown --toolchain nightly
```

1. `yarn main:build`

Will generate three binaries: `skyekiwi-node`, `skw-vm-engine-cli` and `skw-vm-interface`. 

`skyekiwi-node` is the Substrate based blockchain binary. 

`skw-vm-engine-cli` is a testing tool for manualy run a low-level SkyeKiwi Offchain VM secret contract. 

`skw-vm-interface` is used to executing transactions in complete runtime environment. Supported types of transactions are `deploy` to deploy contracts; `create_account` to create accounts; `call` to invoke smart contract calls and `view_method_call` to view the secret state. 

2. `yarn main:test` Might take a while to finish!

Will run tests on:

- All pallets included in the Substrate blockchain 
- The Contract SDK
- The offchain VM
- etc. 

3. Run Enclave Tests:

Currently, the enclave only contains code to run unit tests and integration tests. It is recommanded to run inside the Docker enviornment provided by `teaclave-sgx-sdk`. Make sure you have Docker installed and launched; then run `yarn enclave:sim ` to enter into a interactive environment to work witht eh enclave code. Instead, run `yarn enclave:ci` to simple run all tests inside the enclave. 

**Note:**  the enclave runs on **Intel SGX Platform**, therefore, AMD based computers, cloud VMs, or ARM based computers (like Apple M1 MacBooks) cannot run the docker simulation of the real enclave. For more information, refer to [Apache/Teaclave-SGX-SDK](http://github.com/apache/incubator-teaclave-sgx-sdk). 

## Integrating the SkyeKiwi Protocol to your project
For most users, READMEs on the TypeScript version of the SkyeKiwi Protocol will be more applicable. https://github.com/skyekiwi/skyekiwi-protocol. It's a known issue that most browser wallet extensions have limited to none supports to decrypt/encrypt with ECDH based encryption. We have been working on this problem. 

For curious users who want to integrate the TEE version of the SkyeKiwi Protocol to their code, it won't be as straightforwards as the TypeScript version. Guess a general descriptions of how secrets are processed inside the enclave will be helpful. 

There are the **Trust** and **Untrusted** parts of the protocol. A generaly principle is to NEVER have the unencrypted secrets exposed to the untrusted parts, while minimize the exposure of the metadata to the untrusted parts as well. The calls into the enclaves are called `ecall`, and the processing on the untrusted parts are called `host` or `app`. In our implementation of the SkyeKiwi Protocol inside TEE, the `upstream` and `downstream` processes are broken down to three calls: 

`ecall_protocol_upstream_pre`: to pre-packaging the file from the SGX protected filesystem, encrypt them and emit to the normal file system for the `host` to upload to IPFS. 

`ecall_protocol_upstream_cid_list`: to encrypt the CID list we got from uploading from the previous step. 

`ecall_protocol_upstream_seal`: to process the CID list and seal the metadata to the desired recipients. 

For the downstream procss, it's symmetrical to the upstream process but reversed. 

`ecall_protocol_downstream_pre`: reverse the processed metadata from `ecall_protocol_upstream_seal`;

`ecall_protocol_downstream_cid_list`: reverse the processed CID list from `ecall_protocol_upstream_cid_list`;

`ecall_protocol_downstream_unseal`: reverse the processed metadata from `ecall_protocol_upstream_pre`;

The metadata packaged are identitical to the metadata from the TypeScript implementation. 


## License

The entire code within this repository is licensed under the [GPLv3](LICENSE).

Please [contact us](https://skye.kiwi) if you have questions about
the licensing of our products.

