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
	pub fn read_value(&self,cpu:&Cpu, memory:&memory::CpuRam, ppu:&ppu::Ppu, apu:&apu::Apu, rom:&rom::Rom)->u8{
		match self{
			Addressing::Implied => panic!("Do not use 'read_value()' to read implied value"),
			Addressing::Indirect_X => cpu.read_memory(memory.wram[rom.prg_rom[(cpu.pc - 1) as usize].wrapping_add(cpu.registers[1]) as usize] as u16 
											+ 256 * (memory.wram[rom.prg_rom[(cpu.pc - 1) as usize].wrapping_add(cpu.registers[1]).wrapping_add(1) as usize]) as u16,memory,ppu,apu,rom),
			Addressing::Zeropage => memory.wram[rom.prg_rom[(cpu.pc - 1) as usize] as usize],
			_ => 0,
		}
	}

	pub fn write_value(&self,cpu:&mut Cpu, memory:&mut memory::CpuRam, ppu:&mut ppu::Ppu, apu:&mut apu::Apu, rom:&rom::Rom, value:u8){
		match self{
			Addressing::Implied => panic!("Do not use 'write_value()' to write implied value"),
			Addressing::Indirect_X => cpu.write_memory(memory.wram[rom.prg_rom[(cpu.pc - 1) as usize].wrapping_add(cpu.registers[1]) as usize] as u16 
											+ 256 * (memory.wram[rom.prg_rom[(cpu.pc - 1) as usize].wrapping_add(cpu.registers[1]).wrapping_add(1) as usize]) as u16,memory,ppu,apu,value),
			Addressing::Zeropage => memory.wram[rom.prg_rom[(cpu.pc - 1) as usize] as usize] = value,
			_ => println!("writing value is unimplemented"),
		}
	}

	pub fn bytes(&self)->u16{
		match self{
			Addressing::Implied => 1,
			Addressing::Indirect_X => 2,
			Addressing::Zeropage => 2,
			_ => 0,
		}
	}
}


impl Cpu {
	pub fn exec(&mut self,divided_rom:&rom::Rom,memory: &mut memory::CpuRam, ppu: &mut ppu::Ppu,apu: &mut apu::Apu){
		match divided_rom.prg_rom[self.pc as usize]{
			0x78 => self.SEI(2,Addressing::Implied,memory,ppu,apu,divided_rom),
			0x00 => self.BRK(7,Addressing::Implied,memory,ppu,apu,divided_rom),
			0x01 => self.ORA(6,Addressing::Indirect_X,memory,ppu,apu,divided_rom),
			0x05 => self.ORA(3,Addressing::Zeropage,memory,ppu,apu,divided_rom),
			0x06 => self.ASL(5,Addressing::Zeropage,memory,ppu,apu,divided_rom),
			_ => println!("{:x} is unimplemented!!", divided_rom.prg_rom[self.pc as usize]),
		}
    }
    fn SetStatusByResult(&mut self,result:u8){
    	if result == 0{
    		self.registers[4] = self.registers[4] | 0b0000_0010;
    	}else {
    		self.registers[4] = self.registers[4] & 0b1111_1101;
    	}
    	self.registers[4] = (self.registers[4] & 0b0111_1111) | (result & 0b1000_0000u8);
    }

    fn SEI(&mut self,cycle:usize,addressing:Addressing,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	println!("SEI");
    	self.pc = self.pc + addressing.bytes();
    	self.registers[4] = self.registers[4] | 0b0000_0100;
    	self.cycle = self.cycle + cycle;
    }

    fn BRK(&mut self,cycle:usize,addressing:Addressing,memory: &mut memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	println!("BRK");
    	self.pc = self.pc + addressing.bytes();
    	self.stack_push(memory,((self.pc & 0b11111111_00000000) / 16) as u8);
    	self.stack_push(memory,(self.pc & 0b00000000_11111111) as u8);
    	self.stack_push(memory,self.registers[4]);
    	self.registers[4] = self.registers[4] | 0b0001_0000;
    	self.pc = (self.read_memory(0xFFFF,memory,ppu,apu,rom) as u16) * 0b100000000 + (self.read_memory(0xFFFF,memory,ppu,apu,rom) as u16);
    	self.cycle = self.cycle + cycle;
    }
    fn ORA(&mut self,cycle:usize,addressing:Addressing,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	println!("ORA");
    	self.pc = self.pc + addressing.bytes();
    	self.registers[0] = self.registers[0] | addressing.read_value(&self,memory,ppu,apu,rom);
    	
    	self.SetStatusByResult(self.registers[0]);
    	self.cycle = self.cycle + cycle;
    }
    fn ASL(&mut self,cycle:usize,addressing:Addressing,memory: &mut memory::CpuRam, ppu: &mut ppu::Ppu,apu: &mut apu::Apu,rom: &rom::Rom){
    	println!("ASL");
    	self.pc = self.pc + addressing.bytes();
    	self.registers[4] = self.registers[4] | (addressing.read_value(&self,memory,ppu,apu,rom) >> 7);
    	addressing.write_value(self,memory,ppu,apu,rom,addressing.read_value(&self,memory,ppu,apu,rom) << 1);
    	
    	self.SetStatusByResult(addressing.read_value(&self,memory,ppu,apu,rom));
    	self.cycle = self.cycle + cycle;
    }


    fn OPCODE(&mut self,cycle:usize,addressing:Addressing,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	println!("OPCODE");
    	self.pc = self.pc + addressing.bytes();
    	self.cycle = self.cycle + cycle;
    }
}