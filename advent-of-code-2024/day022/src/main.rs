use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Read,
    sync::Arc,
};

use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let data = read("input_1.txt");

    solve1(&data).await;

    solve2(&data).await;
}

fn read(p: &str) -> Vec<i64> {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

fn mix(given: i64, secret: i64) -> i64 {
    given ^ secret
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

fn next_secret(mut secret: i64) -> i64 {
    // multiplying the secret number by 64
    let mut step_1 = secret * 64;
    // mix this result into the secret number
    step_1 = mix(step_1, secret);
    secret = step_1;
    //  prune the secret number
    step_1 = prune(secret);
    secret = step_1;

    // dividing the secret number by 32. Round the result down to the nearest integer
    let mut step_2 = (secret as f64 / 32.0).floor() as i64;
    //  mix this result into the secret number
    step_2 = mix(step_2, secret);
    secret = step_2;
    // prune the secret number
    step_2 = prune(secret);
    secret = step_2;

    // multiplying the secret number by 2048
    let mut step_3 = secret * 2048;
    // mix this result into the secret number
    step_3 = mix(step_3, secret);
    secret = step_3;
    // prune the secret number
    step_3 = prune(secret);
    secret = step_3;

    secret
}

async fn solve1(data: &[i64]) {
    let res = Arc::new(Mutex::new(0));
    let mut futures = vec![];
    for d in data {
        let mut secret = *d;
        let async_res = res.clone();
        futures.push(tokio::spawn(async move {
            for _ in 0..2000 {
                secret = next_secret(secret)
            }
            *async_res.lock().await += secret
        }));
    }
    for f in futures {
        f.await.unwrap();
    }

    println!("solve1: {}", res.lock().await)
}

async fn solve2(data: &[i64]) {
    let seqs_to_prices = Arc::new(Mutex::new(Vec::new()));
    let mut futures = vec![];
    for d in data {
        let mut secret = *d;
        let async_seqs_to_prices = seqs_to_prices.clone();
        futures.push(tokio::spawn(async move {
            let mut sequence_to_price = HashMap::<[i64; 4], i64>::new();
            let mut prices = Vec::<i64>::with_capacity(2000);
            prices.push(secret % 10);
            for _ in 0..2000 {
                secret = next_secret(secret);
                prices.push(secret % 10);
            }
            let mut changes = Vec::<i64>::with_capacity(2000);
            for i in 1..prices.len() {
                changes.push(prices[i] - prices[i - 1]);
            }
            // println!("prices: {:?}", prices);

            // println!("changes: {:?}", changes);

            for seq in changes.windows(4).enumerate() {
                match sequence_to_price.get_mut(seq.1) {
                    None => {
                        sequence_to_price
                            .insert([seq.1[0], seq.1[1], seq.1[2], seq.1[3]], prices[seq.0 + 4]);
                    }
                    Some(_) => {
                        continue;
                    }
                }
            }
            async_seqs_to_prices.lock().await.push(sequence_to_price);
            // println!("seq_to_prices: {:?}", sequence_to_price)
        }));
    }
    for f in futures {
        f.await.unwrap();
    }

    let unique_sequence = seqs_to_prices
        .lock()
        .await
        .iter()
        .flat_map(|hm| hm.keys())
        .cloned()
        .collect::<Vec<[i64; 4]>>();
    let unique_sequence: HashSet<[i64; 4]> = HashSet::from_iter(unique_sequence.iter().cloned());
    // println!("uniques: {:?}", unique_sequence);

    let solve_2 = Arc::new(Mutex::new(0));
    let mut futures = vec![];
    for seq in unique_sequence {
        let seqs_to_prices = seqs_to_prices.lock().await.clone();
        let async_seq = seq;
        let async_solve_2 = solve_2.clone();
        futures.push(tokio::spawn(async move {
            let mut res = 0;
            for hm in seqs_to_prices {
                if let Some(v) = hm.get(&async_seq) {
                    res += *v;
                }
            }
            let mut v = async_solve_2.lock().await;
            *v = v.max(res);
        }));
    }
    for f in futures {
        f.await.unwrap();
    }

    println!("solve2: {:?}", solve_2.lock().await)
}
