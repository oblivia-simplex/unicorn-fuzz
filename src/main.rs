use std::io;

use unicorn::{Cpu, CpuX86, Mode, Protection};
use std::io::Read;

static PAGE_SIZE: usize  = 0x1000;
static NUM_PAGES: usize = 1;
static NUM_BYTES: usize = NUM_PAGES * PAGE_SIZE;

fn fuzz_x86(code: &[u8]) {
    let mut uc = CpuX86::new(Mode::MODE_32).unwrap();
    let code =
        if code.len() > NUM_BYTES {
            &code[0..NUM_BYTES]
        } else {
            code
    };
    // map some memory
    uc.mem_map(0, NUM_BYTES, Protection::ALL).expect("mem_map failed");
    // write the code
    uc.mem_write(0, code).expect("mem_write failed");
    // emulate
    let _ = uc.emu_start(0, NUM_BYTES as u64, 1000 * unicorn::MILLISECOND_SCALE, 0x1000);
}

fn main() {
    let mut input = io::stdin().bytes().collect::<Result::<Vec<u8>, io::Error>>().expect("Failed to read bytes");
    input.truncate(NUM_BYTES);
    fuzz_x86(&input);
}
