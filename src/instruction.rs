use super::memory;
use super::ppu;
use super::apu;
use super::cpu::Cpu;
use super::rom;

enum Addressing {
	Implied,
	Accumulator,
	Immediate,
	Zeropage,
	Zeropage_X,
	Zeropage_Y,
	Relative,
	Absolute,
	Absolute_X,
	Absolute_Y,
	Indirect,
	Indirect_X,
	Indirect_Y,
}

impl Addressing{
	pub fn read_value(&self)->u8{
		match self{
			Addressing::Implied => panic!("Do not use 'read_value()' to read implied value"),
			_ => return 0,
		}
	}

	pub fn write_value(&self){
		match self{
			Addressing::Implied => panic!("Do not use 'write_value()' to write implied value"),
			_ => println!("writing value is unimplemented"),
		}
	}

	pub fn bytes(&self)->u16{
		match self{
			Addressing::Implied => return 1,
			_ => return 0,
		}
	}
}


impl Cpu {
	pub fn exec(&mut self,divided_rom:&rom::Rom,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu){
		match divided_rom.prg_rom[self.pc as usize]{
			0x78 => self.SEI(2,Addressing::Implied),
			_ => println!("{:x} is unimplemented!!", divided_rom.prg_rom[self.pc as usize]),
		}
    }
    fn SEI(&mut self,cycle:usize,addressing:Addressing){
    	println!("SEI");
    	self.registers[4] = self.registers[4] | 0b0000_0100;
    	self.pc = self.pc + addressing.bytes();
    }
}