#[derive(Debug)]
struct Tx {
    size: u64,
    fee: f64,
}
impl Tx {
    pub fn new(size: u64, fee: f64) -> Self {
        Tx {
            size: size,
            fee: fee,
        }
    }
}
fn main() {
    let mut txs: Vec<Tx> = Vec::new();
    txs.push(Tx::new(57247, 0.0887));
    txs.push(Tx::new(98732, 0.1856));
    txs.push(Tx::new(134928, 0.2307));
    txs.push(Tx::new(77275, 0.1522));
    txs.push(Tx::new(29240, 0.0532));

    txs.push(Tx::new(15440, 0.0250));
    txs.push(Tx::new(70820, 0.1409));
    txs.push(Tx::new(139603, 0.2541));
    txs.push(Tx::new(63718, 0.1147));
    txs.push(Tx::new(143807, 0.2660));

    txs.push(Tx::new(190457, 0.2933));
    txs.push(Tx::new(40572, 0.0686));

    let mut fee: f64 = 0.0;
    let mut size: u64 = 0;
    //let max_size =  500000;
    let max_size = 1000000;
    let base = 12.5;

    loop {
        txs.sort_by_key(|x| (x.fee / (x.size as f64) * 100000000.0) as u64);
        {
            for b in &txs {
                println!("{:?}", b);
            }
        }

        if txs.len() > 0 {
            {
                let chosen = &txs[txs.len() - 1];
                if size + chosen.size <= max_size {
                    fee += chosen.fee;
                    size += chosen.size;
                    println!("Last {:?}", chosen);
                }
            }
            txs.pop();
        } else {
            break;
        }
    }

    println!("Sum:{}", size);
    println!("Fee Total:{}", fee + base);
}
