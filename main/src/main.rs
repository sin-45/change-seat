use std::collections::HashSet;
use std::env; // エラー終了のため (sys.exit)
use std::fs::File;
use std::process::exit; // sys.exit

// クレートのインポート
use csv::{Reader, Writer};
use hex; // 16進文字列への変換
use rand::seq::{IteratorRandom, SliceRandom}; // shuffle
use rand::SeedableRng; // seed_from_u64
use rand_pcg::Pcg64; // Pythonのrandom.seed()の代替
use sha2::{Digest, Sha256};

use num_bigint::BigInt;
use num_traits::{Num, ToPrimitive}; // BigIntの操作に必要

struct RandPotistion{
    reserved_data: Vec<Vec<i32>>,
    width: usize,
    height: usize,
    s: String,
    mod_val: u64,
    nomal_seat_data: Vec<i32>,
    class_position: Vec<Vec<i32>>,
}

impl RandPotistion {
    fn new(
        reserved_data: Vec<Vec<i32>>,
        width: usize,
        height: usize,
        s: String,
        mod_val: u64,
    ) -> Self {
        let mut nomal_seat_data: Vec<i32> = (1..=45).into_iter().collect();
        // 休学者の番号を削除
        let remove_list: Vec<usize> = vec![20];  // ここにいない人を追加
        for i in 0..remove_list.len() {
            nomal_seat_data.remove(&remove_list[i]-1);
        }
        let mut class_position: Vec<Vec<i32>> = vec![vec![-1; width]; height];
        class_position[height-1][0] = 0;
        class_position[height-1][1] = 0;
        class_position[height-1][4] = 0;

        RandPotistion {
            reserved_data,
            width,
            height,
            s,
            mod_val,
            nomal_seat_data,
            class_position
        }
    }

    fn solve(&mut self) {
        self.position_reserved();
        let shuffle_data = self.rand_seed();
        self.position_set(shuffle_data);
        self.output();
    }

    fn rand_seed(&self) -> Vec<i32> {
        let h: String= self.hash_sha256();
        let hash_int: u64 = self.hash_mod(&h);
        // random.seed(hash_int)
        // seed_from_u64 でRNG (乱数生成器) を初期化
        let mut rng = Pcg64::seed_from_u64(hash_int);

        // random.shuffle(self.nomal_seat_data)
        // Rustでは、データをコピーしてシャッフルする
        let mut shuffled_data = self.nomal_seat_data.clone();
        shuffled_data.shuffle(&mut rng);
        shuffled_data
    }

    fn hash_sha256(&self) -> String {
        let x: &str = self.s.as_str();
        let mut hasher = Sha256::new();
        hasher.update(x.as_bytes());
        let result = hasher.finalize();
        // 16進文字列に変換
        hex::encode(result)
    }

    fn hash_mod(&self, h: &str) -> u64 {
        let hash_bigint = BigInt::from_str_radix(h, 16)
            .expect("ハッシュ文字列のパースに失敗 (BigInt)");

        // 3. mod_val (u64) を BigInt に変換
        let mod_bigint = BigInt::from(self.mod_val);

        // 4. BigInt 同士で剰余 (mod) を計算
        let result_bigint = hash_bigint % mod_bigint;

        result_bigint.to_u64().expect("BigIntからu64への変換に失敗")
    }

    fn position_reserved(&mut self) {
        for row in &self.reserved_data {
            let num: i32 = row[0];
            if !self.nomal_seat_data.contains(&num) {
                println!("Error: 指定が重複しています");
                exit(1);
            }
            self.nomal_seat_data.retain(|&x| x != num);
        }
        for row in &self.reserved_data {
            let num: i32 = row[0];
            let h: usize= row[1] as usize;
            let w: usize = row[2] as usize;
            if self.position_ok(h, w) {
                self.class_position[h][w] = num;
            } else {
                println!("Error: 座席指定がかぶっています");
                exit(1);
            }
        }
    }

    fn position_set(&mut self, shuffle_data: Vec<i32>) {
        let mut cnt: usize = 0;
        for h in 0..self.height {
            for w in 0..self.width {
                if self.position_ok(h, w) {
                    self.class_position[h][w] = self.nomal_seat_data[cnt];
                    cnt += 1;
                }
            }
        }
    }

    fn position_ok(&self, h: usize, w: usize) -> bool {
        self.class_position[h][w] == -1
    }

    fn output(&self) {
        let output_data: Vec<Vec<String>> = self
            .class_position
            .iter()
            .rev()
            .map(|row| {
                row.iter()
                    .map(|&item| {
                        if item == 0 {
                            "".to_string()
                        } else {
                            item.to_string()
                        }
                    }).collect()
            }).collect();
        
        for row in &output_data {
            println!("{}", row.join("\t"));
        }
    }
}

fn main() {
    let s: String = "atcoder".to_string();
    let mod_val: u64 = 1_000_000_000;

    let width: usize = 5;
    let height: usize = 9;

    let mut reserved_seat_data: Vec<Vec<i32>> = Vec::new();
    let filename: &str = "../reserved_seat.csv";

    let file = File::open(filename).expect("指定席ファイル(reserved_seat.csv)が見つかりません");
    let mut reader = Reader::from_reader(file);

    // 一行目
    let mut records = reader.records();
    
    // // ヘッダー行 (Pythonの marker = next(reader))
    // if let Some(Ok(header)) = records.next() {
    //      println!("スキップしたヘッダー行: {:?}", header);
    // } else {
    //      eprintln!("Warning: CSVファイルが空か、読み込めません。");
    //      // 空でも処理を続ける (Pythonの挙動に合わせる)
    // }

    // for row in reader:
    for result in records {
        let record = result.expect("CSVレコードの読み込みに失敗");
        
        // reserved_seat_data.append(list(map(int, row)))
        let row: Vec<i32> = record
            .iter()
            .map(|s| s.trim().parse::<i32>().expect("CSVのパースに失敗 (数値ではありません)"))
            .collect();
        
        println!("指定席データ: {:?}", row);
        reserved_seat_data.push(row);
    }
    let mut solve = RandPotistion::new(reserved_seat_data, width, height, s, mod_val);
    solve.solve();
}