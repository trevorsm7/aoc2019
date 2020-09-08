use std::io;

pub fn run_program(memory: &mut [usize]) -> io::Result<()> {
    let mut pc = 0;
    loop {
        match memory[pc] {
            1 => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let c = memory[pc + 3];
                memory[c] = memory[a] + memory[b];
            },
            2 => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let c = memory[pc + 3];
                memory[c] = memory[a] * memory[b];
            },
            99 => return Ok(()),
            _ => return Err(io::Error::new(io::ErrorKind::Other,
                format!("Illegal opcode {} at PC {}", memory[pc], pc))),
        };
        pc += 4;
    }
}