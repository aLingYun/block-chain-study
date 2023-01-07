use sha256;
use rand::prelude::*;
use std::time::SystemTime;

// 计算 block SHA256
fn compute_hash(data: &String, pre_hash: &String, nonce: &String) -> String {
    let mut input = data.clone();
    input.push_str(pre_hash);
    input.push_str(nonce);
    // println!("{}", input);
    sha256::digest(input)
}

fn get_answer(difficult: usize) -> String {
    let mut answer = "".to_string();
    for _i in 0..difficult {
        answer.push('0');
    }
    answer
}

// Block 定义
#[derive(Debug)]
struct Block {
    data: String,
    pre_hash: String,
    nonce: String,
    hash: String,
}

impl Block {
    fn new(data_arg: &String, pre_hash_arg: &String, nonce_arg: &String) -> Block {
        Block { 
            data: data_arg.to_string(), 
            pre_hash: pre_hash_arg.to_string(), 
            nonce: nonce_arg.to_string(), 
            hash: compute_hash(data_arg, pre_hash_arg, nonce_arg),
        }
    }

    // 挖矿
    fn mine(&mut self, difficult: usize) {
        let mut rng = thread_rng();
        let sys_time = SystemTime::now();
        loop {
            if self.hash[0..difficult] != get_answer(difficult) {
                self.nonce = rng.gen_range(0 as usize..18446744073709551615 as usize).to_string();
                self.hash = compute_hash(&(self.data), &(self.pre_hash), &(self.nonce));
                // println!("{}", &(self.nonce));
                // println!("{:#?}", &(self.hash));
                // println!("{:#?}", get_answer(difficult));
            } else {
                break;
            }
        }
        println!("挖矿结束，用时 {:#?} 微秒", sys_time.elapsed().unwrap().as_micros());
    }
}


// Chain 定义
#[derive(Debug)]
struct Chain {
    chain: Vec<Block>,
}

impl Chain {
    fn new() -> Chain {
        Chain { 
            chain: vec![Block::new(&("root".to_string()), &("".to_string()), &("".to_string()))]
        }
    }

    // 添加 block 到 Chain 上
    fn add_block(&mut self, mut blk: Block) {
        blk.pre_hash = self.chain[self.chain.len() - 1].hash.clone();
        blk.hash = compute_hash(&(blk.data), &(blk.pre_hash), &(blk.nonce));
        blk.mine(5);
        self.chain.push(blk);
    }

    fn is_valid_chain(&self) -> bool {
        if self.chain.len() == 1 {
            if self.chain[0].hash != compute_hash(&(self.chain[0].data), &(self.chain[0].pre_hash), &(self.chain[0].nonce)) {
                return false;
            }
            return true;
        } 
        for iter in 1..self.chain.len() {
            let blk_tmp = &self.chain[iter];
            // println!("{}", blk_tmp.hash);
            // println!("{}", compute_hash(&(blk_tmp.data), &(blk_tmp.pre_hash)));
            if blk_tmp.hash != compute_hash(&(blk_tmp.data), &(blk_tmp.pre_hash), &(blk_tmp.nonce)) {
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
    let data = "one yuan".to_string();
    let blk = Block::new(&data, &"".to_string(), &"".to_string());
    chain.add_block(blk);

    let blk1 = Block::new(&"two yuan".to_string(),&"".to_string(), &"".to_string());
    chain.add_block(blk1);
    println!("{:#?}", chain);

    println!("{}", chain.is_valid_chain());

    // proof_of_work(5);
}
