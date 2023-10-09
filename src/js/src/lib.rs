use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{wasm_input, require};
use bytemuck::cast_slice;


#[wasm_bindgen]
pub unsafe fn greet(name: &str) {
    wasm_input(1);
    require(true);
}

static mut BOARD: [u32; 16] = [
    1,0,1,0,
    1,1,0,0,
    1,0,0,1,
    1,1,0,0,
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
    let zero_count = BOARD.iter().filter(|&x| *x == 0) .count();
    // let min = BOARD.iter().min();

    if zero_count == 0 {
        return;
    }

    let mut c = sum%(zero_count as u32);

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

unsafe fn reward(k: i32) {
    if  k > 4 {
        CURRENCY = CURRENCY + (1 << (k - 4));
    }
}

fn left() {}
fn right() {}
fn top() {}
fn bottom() {}

/*
void left(void) {
#pragma clang loop unroll(full)
for (int r=0; r<4; r++) {
int cur = r*4;

/* remove all zeros */
for (int i=0; i<4; i++) {
int current = r*4+i;
if (board[current]!=0) {
board[cur] = board[current];
cur = cur+1;
}
}
for (; cur<r*4+4; cur++) {
board[cur] = 0;
}


/* patch pairs */
cur = r*4;
for (int s=0; s<4; ) {
int current = r*4 + s;
int next = current + 1;
if (s!=3 && board[current] == board[next]) {
int r = board[current];
reward(r);
board[cur] = r+1;
++cur;
s = s + 2;
} else {
board[cur] = board[current];
++cur;
s = s + 1;
}
}
// Fill zero for the rest
for (; cur<r*4+4; cur++) {
board[cur] = 0;
}
}
}

void right(void) {
#pragma clang loop unroll(full)
for (int r=0; r<4; r++) {
int cur = r*4+3;

/* remove all zeros */
for (int i=0; i<4; i++) {
int current = r*4+3-i;
if (board[current]!=0) {
board[cur] = board[current];
cur = cur-1;
}
}
for (; cur>=r*4; cur--) {
board[cur] = 0;
}

/* patch pairs */
cur = r*4 + 3;
for (int s=3; s>=0; ) {
int current = r*4+s;
int next = current-1;
if (s!=0 && board[current] == board[next]) {
int r = board[current];
reward(r);
board[cur] = r+1;
cur = cur - 1;
s = s - 2;
} else {
board[cur] = board[current];
cur = cur - 1;
s = s - 1;
}
}
// Fill zero for the rest
for (; cur>=r*4; cur--) {
board[cur] = 0;
}
}
}

void top(void) {
#pragma clang loop unroll(full)
for (int c=0; c<4; c++) {

int cur = c;

/* remove all zeros */
for (int i=0; i<4; i++) {
int current = i*4 + c;
if (board[current]!=0) {
board[cur] = board[current];
cur = cur+4;
}
}
while (cur < 16) {
board[cur] = 0;
cur = cur + 4;
}

cur = c;
for (int s=0; s<4; ) {
int current = s*4 + c;
int next = current + 4;
if (s!=3 && board[current] == board[next]) {
int r = board[current];
reward(r);
board[cur] = r+1;
cur = cur + 4;
s = s + 2;
} else {
board[cur] = board[current];
cur = cur + 4;
s = s + 1;
}
}
// Fill zero for the rest
while (cur<16) {
board[cur] = 0;
cur = cur + 4;
}
}
}

void bottom(void) {
#pragma clang loop unroll(full)
for (int c=0; c<4; c++) {
int cur = 12 + c;

/* remove all zeros */
for (int i=3; i>=0; i--) {
int current = i*4 + c;
if (board[current]!=0) {
board[cur] = board[current];
cur = cur-4;
}
}
while (cur>=0) {
board[cur] = 0;
cur = cur - 4;
}


/* patch pairs  */
cur = 12 + c;
for (int s=3; s>=0;) {
int current = s*4 + c;
int next = current - 4;
if (s!=0 && board[current] == board[next]) {
int r = board[current];
reward(r);
board[cur] = r+1;
cur = cur - 4;
s = s - 2;
} else {
board[cur] = board[current];
cur = cur - 4;
s = s - 1;
}
}
while (cur >= 0) {
board[cur] = 0;
cur = cur - 4;
}
}
}



 */


#[wasm_bindgen]
pub unsafe fn step(direction: u8) {
    require(CURRENCY  >0);
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
            },
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
