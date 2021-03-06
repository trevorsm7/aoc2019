use std::collections::HashMap;
use std::io;

pub struct Intcode {
    pub memory: Vec<isize>,
    pub output: Vec<isize>,
    sparse_mem: HashMap<usize, isize>,
    pc: usize,
    rel_base: isize,
}

impl Intcode {
    pub fn new(memory: &[isize]) -> Self {
        let memory = Vec::from(memory);
        let output = Vec::new();
        let sparse_mem = HashMap::new();
        Intcode {memory, output, sparse_mem, pc: 0, rel_base: 0}
    }

    pub fn run(&mut self) -> io::Result<bool>
    {
        loop {
            let opcode = self.memory[self.pc] % 100;
            let mode1 = self.memory[self.pc] / 100 % 10;
            let mode2 = self.memory[self.pc] / 1000 % 10;
            let mode3 = self.memory[self.pc] / 10000 % 10;
            let addr1 = self.pc + 1;
            let addr2 = self.pc + 2;
            let addr3 = self.pc + 3;
            match opcode {
                1 => { // Addition
                    let a = self.load(addr1, mode1)?;
                    let b = self.load(addr2, mode2)?;
                    self.store(addr3, mode3, a + b)?;
                    self.pc += 4;
                },
                2 => { // Multiplication
                    let a = self.load(addr1, mode1)?;
                    let b = self.load(addr2, mode2)?;
                    self.store(addr3, mode3, a * b)?;
                    self.pc += 4;
                },
                3 => { // Input
                    // Yield, to be continued by resume()
                    return Ok(true);
                },
                4 => { // Output
                    self.output.push(self.load(addr1, mode1)?);
                    self.pc += 2;
                },
                5 => { // jump-if-true
                    if self.load(addr1, mode1)? != 0 {
                        self.pc = self.load(addr2, mode2)? as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                6 => { // jump-if-false
                    if self.load(addr1, mode1)? == 0 {
                        self.pc = self.load(addr2, mode2)? as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                7 => { // less than
                    let a = self.load(addr1, mode1)?;
                    let b = self.load(addr2, mode2)?;
                    self.store(addr3, mode3, if a < b { 1 } else { 0 })?;
                    self.pc += 4;
                },
                8 => { // equals
                    let a = self.load(addr1, mode1)?;
                    let b = self.load(addr2, mode2)?;
                    self.store(addr3, mode3, if a == b { 1 } else { 0 })?;
                    self.pc += 4;
                },
                9 => { // adjust relative base
                    let a = self.load(addr1, mode1)?;
                    self.rel_base += a;
                    self.pc += 2;
                },
                99 => return Ok(false),
                _ => return Err(io::Error::new(io::ErrorKind::Other,
                    format!("Illegal opcode {} at PC {}", opcode, self.pc))),
            };
        }
    }

    pub fn resume(&mut self, input: isize) -> io::Result<bool> {
        let opcode = self.memory[self.pc] % 100;
        let mode1 = self.memory[self.pc] / 100 % 10;
        let addr1 = self.pc + 1;

        if opcode != 3 {
            return Err(io::Error::new(io::ErrorKind::Other,
                format!("Expected input instruction when resuming at PC {}", self.pc)));
        }

        self.store(addr1, mode1, input)?;
        self.pc += 2;

        self.run()
    }

    fn read(&self, address: usize) -> isize {
        if address < self.memory.len() {
            self.memory[address]
        } else {
            *self.sparse_mem.get(&address).unwrap_or(&0)
        }
    }

    fn write(&mut self, address: usize, value: isize) {
        if address < self.memory.len() {
            self.memory[address] = value;
        } else {
            self.sparse_mem.insert(address, value);
        }
    }

    fn load(&self, address: usize, mode: isize) -> io::Result<isize> {
        let parameter = self.read(address);
        match mode {
            0 => Ok(self.read(parameter as usize)), // Position mode
            1 => Ok(parameter), // Immediate mode
            2 => Ok(self.read((self.rel_base + parameter) as usize)), // Relative mode
            _ => Err(io::Error::new(io::ErrorKind::Other,
                format!("Illegal load mode {} at PC {}", mode, self.pc))),
        }
    }

    fn store(&mut self, address: usize, mode: isize, value: isize) -> io::Result<()> {
        let parameter = self.read(address);
        match mode {
            0 => self.write(parameter as usize, value), // Position mode
            2 => self.write((self.rel_base + parameter) as usize, value), // Relative mode
            _ => return Err(io::Error::new(io::ErrorKind::Other,
                format!("Illegal store mode {}", mode))),
        };
        Ok(())
    }
}
