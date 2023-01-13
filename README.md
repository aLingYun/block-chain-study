# block-chain-study
Use Rust implement a simple block chain.

### Block
我们构建一个 Block 如下：
```rust
struct Block {
    //data: String,
    data: Vec<Transaction>,
    pre_hash: String,
    time_stamp: i64,
    nonce: String,
    hash: String,
}
```
区块链是由一个一个区块构成的有序链表，每一个区块都记录了一系列交易: `data`。并且每个区块都指向前一个区块，从而形成一个链条。区块通过记录上一个区块的哈希来指向上一个区块: `pre_hash`。每个区块都有一个唯一的哈希标识，被称为区块哈希: `hash`。每个 Block 都需要一个时间戳: `time_stamp`。以及一个随机量: `nonce`，用于挖矿时，不改变交易数据的情况下改变区块的 HASH 值。
### Transaction
构建一个 Transaction 如下：
```rust
struct Transaction {
    from: String,
    to: String,
    amount: u32,
    signature: String,
}
```
Transaction 用于记录交易信息，那必然有交易的三要素：

* 付钱方：`from`
* 收钱方：`to`
* 金额：`amount`

`signature` 用于存储私钥对 Transaction 加密产生的签名。这个签名可以用公钥解密出原始数据。

### Chain
构造一个 Chain 如下：
```rust
struct Chain {
    chain: Vec<Block>,
    transaction_pool: Vec<Transaction>,
    miner_reward: u8,
    diffculty: u8,
}
```
其中包含：
* 区块链主体：`chain`
* 交易池：`transaction_pool`
* 矿工奖励：`miner_reward`
* 挖矿难度：`difficulty`

