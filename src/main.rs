use byteorder::{BigEndian, ReadBytesExt};
use std::{
    env,
    fs::{self, File},
    io::BufReader,
    usize,
};

pub mod hw;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./vm <file_path>")
    }

    let mut vm = hw::vm::VM::new();
    // update_cond gets value from input register so this will set ZERO flag
    vm.registers.update_cond_register(0);

    let f: File = File::open(args[1].clone()).expect("Unable to read file");
    let mut f_reader = BufReader::new(f);

    let base_addr: u16 = f_reader
        .read_u16::<BigEndian>()
        .expect("Error reading file as u16");

    let mut addr = base_addr as usize;
    loop {
        match f.read_u16::<BigEndian>() {
            Ok(instruction) => {
                vm.write_memory(addr, instruction);
                addr += 1;
            }
            Err(_) => break,
        }
    }
}
