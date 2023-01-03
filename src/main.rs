use sha256;

#[derive(Debug)]
struct Block {
    data: String,
    pre_hash: String,
    hash: String,
}

fn compute_hash(data: &String, pre_hash: &String) -> String {
    let mut input = data.clone();
    input.push_str(pre_hash);
    // println!("{}", input);
    sha256::digest(input)
}

impl Block {
    fn new(data_arg: &String, pre_hash_arg: &String) -> Block {
        Block { 
            data: data_arg.to_string(), 
            pre_hash: pre_hash_arg.to_string(), 
            hash: compute_hash(data_arg, pre_hash_arg),
        }
    }
}

#[derive(Debug)]
struct Chain {
    chain: Vec<Block>,
}

impl Chain {
    fn new() -> Chain {
        Chain { 
            chain: vec![Block::new(&"root".to_string(), &"".to_string())]
        }
    }

    fn add_block(&mut self, mut blk: Block) {
        blk.pre_hash = self.chain[self.chain.len() - 1].hash.clone();
        blk.hash = compute_hash(&(blk.data), &(blk.pre_hash));
        self.chain.push(blk);
    }

    fn is_valid_chain(&self) -> bool {
        if self.chain.len() == 1 {
            if self.chain[0].hash != compute_hash(&(self.chain[0].data), &(self.chain[0].pre_hash)) {
                return false;
            }
            return true;
        } 
        for iter in 1..self.chain.len() {
            let blk_tmp = &self.chain[iter];
            // println!("{}", blk_tmp.hash);
            // println!("{}", compute_hash(&(blk_tmp.data), &(blk_tmp.pre_hash)));
            if blk_tmp.hash != compute_hash(&(blk_tmp.data), &(blk_tmp.pre_hash)) {
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

fn main() {
    let mut chain = Chain::new();
    let data = "one yuan".to_string();
    let pre_hash = "".to_string();
    let blk = Block::new(&data, &pre_hash);
    chain.add_block(blk);

    let blk1 = Block::new(&"two yuan".to_string(),&"".to_string());
    chain.add_block(blk1);
    println!("{:#?}", chain);

    println!("{}", chain.is_valid_chain());
}
