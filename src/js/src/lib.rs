use bytemuck::cast_slice;
use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{require, wasm_input};

static mut BOARD: [u32; 16] = [
    1, 0, 1, 0,
    1, 1, 0, 0,
    1, 0, 0, 1,
    1, 1, 0, 0,
];

static mut CURRENCY: i32 = 20;

#[wasm_bindgen]
pub unsafe fn setBoard(index: usize, b: u32) {
    BOARD[index] = b;
}

#[wasm_bindgen]
pub unsafe fn getBoard(index: usize) -> u32 {
    BOARD[index]
}

#[wasm_bindgen]
pub unsafe fn setCurrency(n: i32) {
    CURRENCY = n;
}

#[wasm_bindgen]
pub unsafe fn getCurrency() -> i32 {
    CURRENCY
}

unsafe fn random_fill() {
    let sum: u32 = BOARD.iter().sum();
    let zero_count = BOARD.iter().filter(|&x| *x == 0).count();
    // let min = BOARD.iter().min();

    if zero_count == 0 {
        return;
    }

    let mut c = sum % (zero_count as u32);

    for i in 0..BOARD.len() {
        if BOARD[i] != 0 {
            continue;
        }

        if c == 0 {
            BOARD[i] = 1;
            break;
        }

        c -= 1;
    }
}

unsafe fn reward(k: u32) {
    if k > 4 {
        CURRENCY = CURRENCY + (1 << (k - 4));
    }
}

unsafe fn left() {
    for r in 0..4 {
        let mut cur = r * 4;

        /* remove all zeros */
        for i in 0..4 {
            let current = r * 4 + i;
            if BOARD[current] != 0 {
                BOARD[cur] = BOARD[current];
                cur += 1;
            }
        }

        loop {
            if cur >= r * 4 + 4 {
                break;
            }

            BOARD[cur] = 0;
            cur += 1;
        }

        /* patch pairs */
        cur = r * 4;

        let mut s = 0;
        loop {
            if s >= 4 {
                break;
            }

            let current = r * 4 + s;
            let next = current + 1;
            if s != 3 && BOARD[current] == BOARD[next] {
                let r = BOARD[current];
                reward(r);
                BOARD[cur] = r + 1;
                cur += 1;
                s += 2;
            } else {
                BOARD[cur] = BOARD[current];
                cur += 1;
                s += 1;
            }
        }

        // Fill zero for the rest
        loop {
            if cur >= r * 4 + 4 {
                break;
            }
            BOARD[cur] = 0;
            cur += 1;
        }
    }
}

unsafe fn right() {
    for r in 0..4 {
        let mut cur = r * 4 + 3;

        /* remove all zeros */
        for i in 0..4 {
            let current = r * 4 + 3 - i;
            if BOARD[current] != 0 {
                BOARD[cur] = BOARD[current];
                cur -= 1;
            }
        }

        loop {
            if cur < r * 4 {
                break;
            }

            BOARD[cur] = 0;
            cur -= 1;
        }

        /* patch pairs */
        cur = r * 4 + 3;
        let mut s = 3;
        loop {
            if s < 0 {
                break;
            }

            let current = r * 4 + s;
            let next = current - 1;

            if s != 0 && BOARD[current] == BOARD[next] {
                let r = BOARD[current];
                reward(r);
                BOARD[cur] = r + 1;
                cur -= 1;
                s -= 2;
            } else {
                BOARD[cur] = BOARD[current];
                cur -= 1;
                s -= 1;
            }
        }

        // Fill zero for the rest
        loop {
            if cur < r * 4 {
                break;
            }

            BOARD[cur] = 0;
            cur -= 1;
        }
    }
}

unsafe fn top() {
    for c in 0..4 {
        let mut cur = c;

        /* remove all zeros */
        for i in 0..4 {
            let current = i * 4 + c;
            if BOARD[current] != 0 {
                BOARD[cur] = BOARD[current];
                cur += 4;
            }
        }
        loop {
            if cur >= 16 {
                break;
            }
            BOARD[cur] = 0;
            cur += 4;
        }

        cur = c;

        let mut s = 0;
        loop {
            if s >= 4 {
                break;
            }

            let current = s * 4 + c;
            let next = current + 4;

            if s != 3 && BOARD[current] == BOARD[next] {
                let r = BOARD[current];
                reward(r);
                BOARD[cur] = r + 1;
                cur += 4;
                s += 2;
            } else {
                BOARD[cur] = BOARD[current];
                cur += 4;
                s += 1;
            }
        }

        loop {
            if cur >= 16 {
                break;
            }

            BOARD[cur] = 0;
            cur += 4;
        }
    }
}

unsafe fn bottom() {
    for c in 0..4 {
        let mut cur = c + 12;

        /* remove all zeros */
        for i in (0..4).rev() {
            let current = i * 4 + c;
            if BOARD[current] != 0 {
                BOARD[cur] = BOARD[current];
                cur -= 4;
            }
        }
        loop {
            if cur < 0 {
                break;
            }
            BOARD[cur] = 0;
            cur -= 4;
        }

        cur = c + 12;

        let mut s = 3;
        loop {
            if s < 0 {
                break;
            }

            let current = s * 4 + c;
            let next = current - 4;

            if s != 0 && BOARD[current] == BOARD[next] {
                let r = BOARD[current];
                reward(r);
                BOARD[cur] = r + 1;
                cur -= 4;
                s -= 2;
            } else {
                BOARD[cur] = BOARD[current];
                cur -= 4;
                s -= 1;
            }
        }

        loop {
            if cur < 0 {
                break;
            }

            BOARD[cur] = 0;
            cur -= 4;
        }
    }
}


#[wasm_bindgen]
pub unsafe fn step(direction: u8) {
    require(CURRENCY > 0);
    CURRENCY -= 1;

    match direction {
        0 => top(),
        1 => left(),
        2 => bottom(),
        3 => right(),
        _ => {}
    }

    random_fill();
}

#[wasm_bindgen]
pub unsafe fn sell(n: usize) {
    require(n >= 0);
    require(n < 16);

    require(BOARD[n] > 0);
    require(BOARD.iter().all(|&x| x <= BOARD[n]));

    CURRENCY += 1 << BOARD[n];
    BOARD[n] = 0;
}

#[wasm_bindgen]
pub unsafe fn zkmain() -> u64 {
    let u8_len = wasm_input(1);

    let cmd = read_bytes_from_u64(&u8_len, 0);

    let mut iter = cmd.iter();

    loop {
        match iter.next() {
            Some(&c) => {
                if c <= 3_u8 {
                    step(c);
                } else {
                    let target_cell = *iter.next().unwrap();
                    sell(target_cell as usize);
                }
            }
            None => return 0,
        }
    }
}

unsafe fn read_bytes_from_u64(u8_len: &u64, is_public: u32) -> Vec<u8> {
    let mut buffer: Vec<u64> = Vec::new();

    let read_count = match u8_len % 8 {
        0 => u8_len / 8,
        _ => u8_len / 8 + 1,
    };

    for _n in 0..read_count {
        buffer.push(wasm_input(is_public));
    }

    Vec::from(cast_slice(buffer.as_slice()))
}
