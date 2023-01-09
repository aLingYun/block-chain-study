use rand::prelude::*;
use sha256;
use std::time::{SystemTime, UNIX_EPOCH};

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
    let ms = since_the_epoch.as_secs() as i64 * 1000i64
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}

#[derive(Debug, Clone)]
struct Transaction {
    from: String,
    to: String,
    amount: u32,
}

impl Transaction {
    fn new(from_arg: String, to_arg: String, amount_arg: u32) -> Transaction {
        Transaction {
            from: from_arg,
            to: to_arg,
            amount: amount_arg,
        }
    }
}

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

    fn to_string_for_hash(&self) -> String {
        let mut data_string = "".to_string();
        for iter in &self.data {
            data_string.push_str(&iter.to_string()[..]);
        }
        format!("{}{}{}{}", data_string, self.pre_hash, self.time_stamp, self.nonce)
    }

    // 挖矿
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
            miner_reward: 50_u8,
            diffculty: 3_u8,
        }
    }

    fn add_transaction(&mut self, tran: Transaction) {
        self.transaction_pool.push(tran);
    }

    fn mine_transaction_pool(&mut self, miner: String) {
        let tran = Transaction::new(
            "".to_string(),
            miner,
            self.miner_reward as u32,
        );
        self.add_transaction(tran);

        let blk = Block::new(self.transaction_pool.clone());
        self.add_block(blk);
        self.transaction_pool = vec![];      
    }

    // 添加 block 到 Chain 上
    fn add_block(&mut self, mut blk: Block) {
        blk.pre_hash = self.chain[self.chain.len() - 1].hash.clone();
        blk.hash = sha256::digest(blk.to_string_for_hash());
        blk.mine(self.diffculty);
        self.chain.push(blk);
    }

    fn is_valid_chain(&self) -> bool {
        if self.chain.len() == 1 {
            if self.chain[0].hash != sha256::digest(self.chain[0].to_string_for_hash())
            {
                return false;
            }
            return true;
        }
        for iter in 1..self.chain.len() {
            let blk_tmp = &self.chain[iter];
            if blk_tmp.hash != sha256::digest(blk_tmp.to_string_for_hash())
            {
                println!("数据被篡改");
                return false;
            }

            if blk_tmp.pre_hash != self.chain[iter - 1].hash {
                println!("区块断裂");
                return false;
            }
        }
        return true;
    }
}

// fn proof_of_work(difficult: usize) {
//     let data = "Hello".to_string();
//     let mut x = 1;
//     loop {
//         let mut s = data.clone();
//         s.push_str(&x.to_string());
//         let result = sha256::digest(s.clone());
//         if &(result[0..difficult]) != "00000" {
//             x += 1;
//         } else {
//             println!("{}", result);
//             println!("{}", x);
//             break;
//         }
//     }
// }

fn main() {
    let mut chain = Chain::new();

    let tran1 = Transaction::new("addr1".to_string(),
        "addr2".to_string(), 10_u32);
    let tran2 = Transaction::new("addr2".to_string(),
        "addr3".to_string(), 10_u32);
    let tran3 = Transaction::new("addr3".to_string(),
        "addr1".to_string(), 10_u32);
    chain.add_transaction(tran1);
    chain.add_transaction(tran2);
    chain.add_transaction(tran3);
    chain.mine_transaction_pool("miner1".to_string());

    let tran4 = Transaction::new("XiaoMing".to_string(),
        "XiaoHong".to_string(), 10_u32);
    let tran5 = Transaction::new("XiaoHong".to_string(),
        "XiaoGang".to_string(), 10_u32);
    let tran6 = Transaction::new("XiaoGang".to_string(),
        "XiaoMing".to_string(), 10_u32);
    chain.add_transaction(tran4);
    chain.add_transaction(tran5);
    chain.add_transaction(tran6);
    chain.mine_transaction_pool("miner1".to_string());

    println!("{:#?}", chain);
    println!("{}", chain.is_valid_chain());
}
