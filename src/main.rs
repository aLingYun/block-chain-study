use rand::prelude::*;
use sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use crypto::ed25519::{keypair, signature, verify};

// 获取 difficult 个 0 的 String
fn get_answer(difficult: u8) -> String {
    let mut answer = "".to_string();
    for _i in 0..difficult {
        answer.push('0');
    }
    answer
}

// 获取时间戳
fn get_time_stamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ns = since_the_epoch.as_secs() as i64 * 1_000_000_000_i64
        + (since_the_epoch.subsec_nanos() as f64) as i64;
    ns
}

// 将 &[u8] 转换为 String
fn u8_to_string(array: &[u8]) -> String {
    let mut s = "".to_string();
    for i in array {
        let tmp = format!("{:02X}", i);
        s.push_str(&tmp[..]);
    }
    s
}

// 将 String 转换为 [u8]
fn string_to_u8(ss: &String) -> [u8; 64] {
    let mut array_u8 = [0; 64];
    let mut i = 0_usize;
    for iter in ss.bytes() {
        if iter < 65 {
            if i % 2 == 0 {
                array_u8[i/2] = (iter - 48) * 16;
            } else {
                array_u8[i/2] += iter - 48;
            }
        } else {
            if i % 2 == 0 {
                array_u8[i/2] = (iter - 55) * 16;
            } else {
                array_u8[i/2] += iter - 55;
            }
        }
        i += 1;
    }
    array_u8
}

// transaction: 用于记录每一笔转账
#[derive(Debug, Clone)]
struct Transaction {
    from: String,
    to: String,
    amount: u32,
    signature: String,
}

impl Transaction {
    fn new(from_arg: String, to_arg: String, amount_arg: u32) -> Transaction {
        Transaction {
            from: from_arg,
            to: to_arg,
            amount: amount_arg,
            signature: "".to_string(),
        }
    }

    // 用私钥对 transaction 的 sha256 进行签名
    fn sign(&mut self, private_key: &[u8]) {
        self.signature = u8_to_string(&signature(sha256::digest(self.to_string()).as_bytes(), private_key));
    }

    // 用公钥对签名进行解密，并与 transaction 的 sha256 比较，验证 transaction 是否有效
    fn is_valid_transaction(&self) -> bool {
        if self.from == "".to_string() && self.to != "".to_string() {
            return true;
        }
        if self.signature == "".to_string() {
            return false;
        }
        //println!("is valid: {:?}", &string_to_u8(&self.from));
        return verify(sha256::digest(self.to_string()).as_bytes(), 
                      &string_to_u8(&self.from)[0..32], 
                      &string_to_u8(&self.signature));
    }
}

// 实现 to_string() 以便传入 sha256::digest() 计算整个 transaction 的 SHA256
impl ToString for Transaction {
    fn to_string(&self) -> String {
        format!("{}{}{}", self.from, self.to, self.amount)
    }
}

// Block 定义
#[derive(Debug)]
struct Block {
    //data: String,
    data: Vec<Transaction>,
    pre_hash: String,
    time_stamp: i64,
    nonce: String,
    hash: String,
}

impl Block {
    fn new(data_arg: Vec<Transaction>) -> Block {
        Block {
            data: data_arg,
            pre_hash: "".to_string(),
            time_stamp: get_time_stamp(),
            nonce: "".to_string(),
            hash: "".to_string(),
        }
    }
    // 以便传入 sha256::digest() 计算整个 block 的 SHA256
    fn to_string_for_hash(&self) -> String {
        let mut data_string = "".to_string();
        for iter in &self.data {
            data_string.push_str(&iter.to_string()[..]);
        }
        format!("{}{}{}{}", data_string, self.pre_hash, self.time_stamp, self.nonce)
    }

    // 挖矿 (计时功能不需要)
    fn mine(&mut self, difficult: u8) {
        let mut rng = thread_rng();
        let sys_time = SystemTime::now();
        loop {
            if self.hash[0..(difficult as usize)] != get_answer(difficult) {
                self.nonce = rng
                    .gen_range(0 as usize..18446744073709551615 as usize)
                    .to_string();
                self.hash = sha256::digest(self.to_string_for_hash());
            } else {
                break;
            }
        }
        println!(
            "挖矿结束，用时 {:#?} 微秒",
            sys_time.elapsed().unwrap().as_micros()
        );
    }

    // 遍历整个 transaction pool，验证有效性
    fn all_transaction_is_valid(&self) -> bool {
        for iter in &self.data {
            if !iter.is_valid_transaction() {
                println!("This is invalid transaction");
                return false;
            }
        }
        true
    }
}

// Chain 定义
#[derive(Debug)]
struct Chain {
    chain: Vec<Block>,
    transaction_pool: Vec<Transaction>,
    miner_reward: u8,
    diffculty: u8,
}

impl Chain {
    fn new() -> Chain {
        let tran = Transaction::new(
            "".to_string(), 
            "".to_string(), 
            0_u32);
        let mut blk = Block::new(vec![tran]);
        blk.hash = sha256::digest(blk.to_string_for_hash());
        Chain {
            chain: vec![blk],
            transaction_pool: vec![],
            miner_reward: 50_u8,// 矿工的奖励
            diffculty: 4_u8,    // 挖矿的难度设置，即前面有几个连续的 0
        }
    }

    // 将某个 transaction 添加到 transaction pool 中
    fn add_transaction(&mut self, tran: Transaction) {
        // 如果支付和接受方有一方没有信息，都是无效的
        if tran.from == "".to_string() || tran.to == "".to_string() {
            println!("Invalid from or to!");
            return;
        }
        // transaction 有效才会加入 pool 中
        if tran.is_valid_transaction() {
            self.transaction_pool.push(tran);
        } else {
            println!("Invalid Transaction!");
        }
    }

    // 挖整个 transaction pool
    fn mine_transaction_pool(&mut self, miner: String) {
        // 矿工奖励 transaction
        let tran = Transaction::new(
            "".to_string(),
            miner,
            self.miner_reward as u32,
        );
        // 有效 transaction，加入 pool
        if tran.is_valid_transaction() {
            self.transaction_pool.push(tran);
        }

        // 从 transaction 创建 Block，并添加到 Chain
        let blk = Block::new(self.transaction_pool.clone());
        self.add_block(blk);
        // 将 Chain 上已有的 transaction 都保存到 Block 中，
        // 并加入到 Chain 后，将 pool 清空
        self.transaction_pool = vec![];      
    }

    // 添加 block 到 Chain 上
    fn add_block(&mut self, mut blk: Block) {
        blk.pre_hash = self.chain[self.chain.len() - 1].hash.clone();
        blk.hash = sha256::digest(blk.to_string_for_hash());
        // 挖矿
        blk.mine(self.diffculty);
        // 挖完将 Block 加入到 Chain
        self.chain.push(blk);
    }

    // 验证整个 Chain 是否有效
    fn is_valid_chain(&self) -> bool {
        // 只有一个创世区块
        if self.chain.len() == 1 {
            if self.chain[0].hash != sha256::digest(self.chain[0].to_string_for_hash())
            {
                return false;
            }
            return true;
        }
        // 遍历 Chain 上所有的 Block 
        for iter in 1..self.chain.len() {
            let blk_tmp = &self.chain[iter];
            // Block 中所有 transaction 的有效性
            if !blk_tmp.all_transaction_is_valid() {
                println!("This is a invalid block!");
                return false;
            }
            // Block 整体数据的有效性
            if blk_tmp.hash != sha256::digest(blk_tmp.to_string_for_hash())
            {
                println!("数据被篡改");
                return false;
            }
            // Chain 上所有 Block 的链接关系验证
            if blk_tmp.pre_hash != self.chain[iter - 1].hash {
                println!("区块断裂");
                return false;
            }
        }
        return true;
    }
}

fn main() {
    // 生成发送发和接收方的密钥对
    let seed_string = b"qwertyuiopasdfghjklzxcvbnm012345";  
    let (private_key_s, public_key_s) = keypair(seed_string);
    // println!("public key = {:?}", u8_to_string(&public_key_s));
    // println!("private key = {:?}", u8_to_string(&private_key_s));
    let seed_string = b"012345qwertyuiopasdfghjklzxcvbnm";  
    let (_private_key_r, public_key_r) = keypair(seed_string);
    // println!("public key = {:?}", u8_to_string(&public_key_r));
    // println!("private key = {:?}", u8_to_string(&private_key_r));

    // 新建一个 Chain
    let mut chain = Chain::new();

    // 新建三笔转账
    let mut tran1 = Transaction::new(
        u8_to_string(&public_key_s),
        u8_to_string(&public_key_r), 
        10_u32
    );
    let mut tran2 = Transaction::new(
        u8_to_string(&public_key_s),
        u8_to_string(&public_key_r), 
        20_u32
    );
    let mut tran3 = Transaction::new(
        u8_to_string(&public_key_s),
        u8_to_string(&public_key_r),  
        30_u32
    );
    // 对这三笔转账进行签名
    tran1.sign(&private_key_s);
    tran2.sign(&private_key_s);
    tran3.sign(&private_key_s);
    // println!("{}", tran1.is_valid_transaction());
    // println!("{}", tran2.is_valid_transaction());
    // println!("{}", tran3.is_valid_transaction());

    // 添加到 Chain 的 transaction pool 中，等待矿工挖矿
    chain.add_transaction(tran1);
    chain.add_transaction(tran2);
    chain.add_transaction(tran3);
    // 挖整个 transaction pool
    chain.mine_transaction_pool("miner1".to_string());

    // 篡改区块中的数据
    //chain.chain[1].data[1].amount = 1;

    println!("{:#?}", chain);
    println!("A whole chain is valid: {}", chain.is_valid_chain());
}
