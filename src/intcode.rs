use std::io;

fn load(memory: &[isize], address: usize, mode: isize) -> io::Result<isize> {
    let parameter = memory[address];
    match mode {
        0 => Ok(memory[parameter as usize]),
        1 => Ok(parameter),
        _ => Err(io::Error::new(io::ErrorKind::Other,
            format!("Illegal load mode {}", mode))),
    }
}

fn store(memory: &mut [isize], address: usize, mode: isize, value: isize) -> io::Result<()> {
    let parameter = memory[address];
    match mode {
        0 => {
            memory[parameter as usize] = value;
            Ok(())
        },
        _ => Err(io::Error::new(io::ErrorKind::Other,
            format!("Illegal store mode {}", mode))),
    }
}

pub fn run_program(mut memory: &mut [isize], inputs: &[isize]) -> io::Result<Vec<isize>> {
    let mut pc = 0;
    let mut output = Vec::new();
    println!("Mem: {}, Inputs: {:?}", memory.len(), inputs);
    loop {
        let opcode = memory[pc] % 100;
        let mode1 = memory[pc] / 100 % 10;
        let mode2 = memory[pc] / 1000 % 10;
        let mode3 = memory[pc] / 10000 % 10;
        match opcode {
            1 => { // Addition
                let a = load(&memory, pc + 1, mode1)?;
                let b = load(&memory, pc + 2, mode2)?;
                store(&mut memory, pc + 3, mode3, a + b)?;
                pc += 4;
            },
            2 => { // Multiplication
                let a = load(&memory, pc + 1, mode1)?;
                let b = load(&memory, pc + 2, mode2)?;
                store(&mut memory, pc + 3, mode3, a * b)?;
                pc += 4;
            },
            3 => { // Input
                store(&mut memory, pc + 1, mode1, inputs[0])?;
                pc += 2;
            },
            4 => { // Output
                let a = load(&memory, pc + 1, mode1)?;
                output.push(a);
                pc += 2;
            },
            99 => return Ok(output),
            _ => return Err(io::Error::new(io::ErrorKind::Other,
                format!("Illegal opcode {} at PC {}", memory[pc], pc))),
        };
    }
}