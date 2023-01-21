#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn xor(x1: i32, x2: i32) -> i32 {
    return tb(x1) ^ tb(x2)
}

fn tb(x: i32) -> i32 {
    if x != 0 { return 1 }
    else { return 0 }
}

fn step(val: f64) -> i32 {
    return ((val.abs() / val + 1.0) / 2.0) as i32
}

fn numerical_diff<F>(f: &mut F, xs: &Vec<f64>) -> Vec<f64>
    where F: FnMut(&Vec<f64>) -> f64
{
    const H: f64 = 0.0001;
    let mut ret: Vec<f64> = vec![];
    for (i, &x) in xs.iter().enumerate() {
        let mut diffX: Vec<f64> = xs.to_vec();
        diffX[i] += H;
        let a1: f64 = f(&diffX);
        diffX[i] -= 2.0 * H;
        let a2: f64 = f(&diffX);
        ret[i] = a1 - a2 / 2.0 * H;
    }
    return ret
}

fn affine(x1: i32, x2: i32, w1: f64, w2: f64, bias: f64) -> f64 {
    return x1  as f64 * w1 + x2 as f64 * w2 + bias
}

fn allok() -> impl FnMut(usize, i32) -> bool {
    let mut flag = true;

    return move |c: usize, yminust: i32| -> bool {
        if yminust == 0 { //誤差が無い
            if c == 0 {
                flag = true;
            } else if c == 3 {
                if flag {
                    return false
                };
            }
        } else { // 誤差がある
            if c == 0 {
                flag = false
            } else if c == 1 {
                if flag {flag = false}
            } else if c == 2 {
                if flag {flag = false}
            } else if c == 3 {
                if flag {flag = false}
            }
        }
        return true
    }
}

struct Config {
    w1: f64,
    w2: f64,
    bias: f64,
    llate: f64
}

struct Parameter {
    w1: f64,
    w2: f64,
    bias: f64
}

struct Input(i32, i32);

fn main() {
    let config = Config {
        w1: 1.0, w2: 1.0, bias: 1.0, llate: 0.5
    };

    let mut pram: Parameter = Parameter {
        w1: config.w1, w2: config.w2, bias: config.bias
    };

    let x1 = 0;
    let x2 = 0;
    const PATTERN: [Input; 4] = [
        Input(0, 0),
        Input(1, 0),
        Input(0, 1),
        Input(1, 1),
    ];

    let mut counter: usize = 0;
    let mut _allok = allok();
    loop {
        let c = counter % 4;
        // Parameter表示
        println!("w1={}, w2={}", pram.w1, pram.w2);
        println!("bias={}", pram.bias);
        // アフィン変換
        let aff: f64 = affine(PATTERN[c].0, PATTERN[c].1,
                              pram.w1, pram.w2, pram.bias);
        // 活性化関数
        let y: i32 = step(aff);

        // 学習
        // 正解データ
        let lerning_data = PATTERN[c].0 * PATTERN[c].1;

        // 誤差
        let yminust = y - lerning_data;
        // 学習終了判定
        if _allok(c, yminust) != true {
            break;
        }
        // 重み、bias更新
        let dw1: f64 = config.llate * lerning_data as f64 * PATTERN[c].0 as f64;
        let dw2: f64 = config.llate * lerning_data as f64 * PATTERN[c].1 as f64;
        let db: f64  = config.llate * lerning_data as f64;
        pram.w1 += dw1;
        pram.w2 += dw2;
        pram.bias += db;
        counter += 1;
    }
}
