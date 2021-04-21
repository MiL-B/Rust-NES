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
			Addressing::Immediate => rom.prg_rom[(cpu.pc - 1) as usize],
			Addressing::Accumulator => cpu.registers[0],
			Addressing::Absolute => cpu.read_memory((rom.prg_rom[(cpu.pc - 2) as usize] as u16) + 256 * (rom.prg_rom[(cpu.pc - 1) as usize] as u16),memory,ppu,apu,rom),
			Addressing::Relative => panic!("Addressing mode Relative is not for indicating value."),
			Addressing::Indirect_Y => cpu.read_memory(memory.wram[rom.prg_rom[(cpu.pc - 1) as usize] as usize] as u16 
											+ 256 * memory.wram[rom.prg_rom[(cpu.pc - 1) as usize].wrapping_add(1) as usize] as u16 + cpu.registers[2] as u16,memory,ppu,apu,rom),
			Addressing::Zeropage_X => memory.wram[rom.prg_rom[(cpu.pc - 1) as usize].wrapping_add(cpu.registers[1]) as usize],
			Addressing::Absolute_X => cpu.read_memory((rom.prg_rom[(cpu.pc - 2) as usize] as u16) + 256 * (rom.prg_rom[(cpu.pc - 1) as usize] as u16) + cpu.registers[1] as u16,memory,ppu,apu,rom),
			Addressing::Absolute_Y => cpu.read_memory((rom.prg_rom[(cpu.pc - 2) as usize] as u16) + 256 * (rom.prg_rom[(cpu.pc - 1) as usize] as u16) + cpu.registers[2] as u16,memory,ppu,apu,rom),
			_ => panic!("unimplemented"),
		}
	}

	pub fn write_value(&self,cpu:&mut Cpu, memory:&mut memory::CpuRam, ppu:&mut ppu::Ppu, apu:&mut apu::Apu, rom:&rom::Rom, value:u8){
		match self{
			Addressing::Implied => panic!("Do not use 'write_value()' to write implied value"),
			Addressing::Indirect_X => cpu.write_memory(memory.wram[rom.prg_rom[(cpu.pc - 1) as usize].wrapping_add(cpu.registers[1]) as usize] as u16 
											+ 256 * (memory.wram[rom.prg_rom[(cpu.pc - 1) as usize].wrapping_add(cpu.registers[1]).wrapping_add(1) as usize]) as u16,memory,ppu,apu,value),
			Addressing::Zeropage => memory.wram[rom.prg_rom[(cpu.pc - 1) as usize] as usize] = value,
			Addressing::Immediate => panic!("Immediate value is not writable!"),
			Addressing::Accumulator => cpu.registers[0] = value,
			Addressing::Absolute => cpu.write_memory((rom.prg_rom[(cpu.pc - 2) as usize] as u16) + 256 * (rom.prg_rom[(cpu.pc - 1) as usize] as u16),memory,ppu,apu,value),
			Addressing::Relative => panic!("Addressing mode Relative is not for indicating value."),
			Addressing::Indirect_Y => cpu.write_memory(memory.wram[rom.prg_rom[(cpu.pc - 1) as usize] as usize] as u16 
											+ 256 * (memory.wram[rom.prg_rom[(cpu.pc - 1) as usize].wrapping_add(1) as usize]) as u16 + cpu.registers[2] as u16,memory,ppu,apu,value),
			Addressing::Zeropage_X => memory.wram[rom.prg_rom[(cpu.pc - 1) as usize].wrapping_add(cpu.registers[1]) as usize] = value,
			Addressing::Absolute_X => cpu.write_memory((rom.prg_rom[(cpu.pc - 2) as usize] as u16) + 256 * (rom.prg_rom[(cpu.pc - 1) as usize] as u16) + cpu.registers[1] as u16,memory,ppu,apu,value),
			Addressing::Absolute_Y => cpu.write_memory((rom.prg_rom[(cpu.pc - 2) as usize] as u16) + 256 * (rom.prg_rom[(cpu.pc - 1) as usize] as u16) + cpu.registers[2] as u16,memory,ppu,apu,value),
			_ => println!("writing value is unimplemented"),
		}
	}

	pub fn bytes(&self)->u16{
		match self{
			Addressing::Implied => 1,
			Addressing::Indirect_X => 2,
			Addressing::Zeropage => 2,
			Addressing::Immediate => 2,
			Addressing::Accumulator => 1,
			Addressing::Absolute => 3,
			Addressing::Relative => 2,
			Addressing::Indirect_Y => 2,
			Addressing::Zeropage_X => 2,
			Addressing::Absolute_X => 3,
			Addressing::Absolute_Y => 3,
			_ => 0,
		}
	}
	pub fn extra_cycle(&self,cpu:&Cpu, memory:&memory::CpuRam,rom:&rom::Rom)->usize{
		match self{
			Addressing::Indirect_Y if  memory.wram[rom.prg_rom[(cpu.pc - 1) as usize] as usize].overflowing_add(cpu.registers[2]).1 => 1,
			Addressing::Absolute_X if  rom.prg_rom[(cpu.pc - 2) as usize].overflowing_add(cpu.registers[1]).1 => 1,
			Addressing::Absolute_Y if  rom.prg_rom[(cpu.pc - 2) as usize].overflowing_add(cpu.registers[2]).1 => 1,
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
			0x08 => self.PHP(3,Addressing::Implied,memory,ppu,apu,divided_rom),
			0x09 => self.ORA(2,Addressing::Immediate,memory,ppu,apu,divided_rom),
			0x0A => self.ASL(2,Addressing::Accumulator,memory,ppu,apu,divided_rom),
			0x0D => self.ORA(4,Addressing::Absolute,memory,ppu,apu,divided_rom),
			0x0E => self.ASL(6,Addressing::Absolute,memory,ppu,apu,divided_rom),
			0x10 => self.BPL(2,Addressing::Relative,memory,ppu,apu,divided_rom),
			0x11 => self.ORA(5,Addressing::Indirect_Y,memory,ppu,apu,divided_rom),
			0x15 => self.ORA(4,Addressing::Zeropage_X,memory,ppu,apu,divided_rom),
			0x16 => self.ASL(6,Addressing::Zeropage_X,memory,ppu,apu,divided_rom),
			0x18 => self.CLC(2,Addressing::Implied,memory,ppu,apu,divided_rom),
			0x19 => self.ORA(4,Addressing::Absolute_Y,memory,ppu,apu,divided_rom),
			0x1D => self.ORA(4,Addressing::Absolute_X,memory,ppu,apu,divided_rom),
			0x1E => self.ASL(7,Addressing::Absolute_X,memory,ppu,apu,divided_rom),
			0x20 => self.JSR(6,Addressing::Absolute,memory,ppu,apu,divided_rom),
			0x21 => self.AND(6,Addressing::Indirect_X,memory,ppu,apu,divided_rom),
			0x24 => self.BIT(3,Addressing::Zeropage,memory,ppu,apu,divided_rom),
			0x25 => self.AND(3,Addressing::Zeropage,memory,ppu,apu,divided_rom),
			0x26 => self.ROL(5,Addressing::Zeropage,memory,ppu,apu,divided_rom),
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
    	self.cycle = self.cycle + cycle + addressing.extra_cycle(self,memory,rom);
    }
    fn ASL(&mut self,cycle:usize,addressing:Addressing,memory: &mut memory::CpuRam, ppu: &mut ppu::Ppu,apu: &mut apu::Apu,rom: &rom::Rom){
    	println!("ASL");
    	self.pc = self.pc + addressing.bytes();
    	self.registers[4] = self.registers[4] | (addressing.read_value(&self,memory,ppu,apu,rom) >> 7);
    	addressing.write_value(self,memory,ppu,apu,rom,addressing.read_value(&self,memory,ppu,apu,rom) << 1);
    	
    	self.SetStatusByResult(addressing.read_value(&self,memory,ppu,apu,rom));
    	self.cycle = self.cycle + cycle;
    }
    fn PHP(&mut self,cycle:usize,addressing:Addressing,memory: &mut memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	println!("PHP");
    	self.pc = self.pc + addressing.bytes();
    	self.stack_push(memory,self.registers[4]);
    	self.cycle = self.cycle + cycle;
    }
    fn BPL(&mut self,cycle:usize,addressing:Addressing,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	println!("BPL");
    	self.pc = self.pc + addressing.bytes();
    	if (self.registers[4] >> 7) & 1u8 == 0{
    		self.cycle = self.cycle + 1;
    		let upper_address = (self.pc >> 8) as u8;
    		if (rom.prg_rom[(self.pc - 1) as usize] >> 7) & 1u8 == 0{
    			self.pc = self.pc + rom.prg_rom[(self.pc - 1) as usize] as u16;
    		}else {
    			self.pc = self.pc - (!rom.prg_rom[(self.pc - 1) as usize] as u16 + 1);
    		}
    		if upper_address != (self.pc >> 8) as u8{
    			self.cycle = self.cycle + 1;
    		}
    	}
    	self.cycle = self.cycle + cycle;
    }
    fn CLC(&mut self,cycle:usize,addressing:Addressing,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	println!("CLC");
    	self.pc = self.pc + addressing.bytes();
    	self.registers[4] = self.registers[4] & 0b1111_1110;
    	self.cycle = self.cycle + cycle;
    }
    fn JSR(&mut self,cycle:usize,addressing:Addressing,memory: &mut memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	println!("JSR");
    	self.pc = self.pc + addressing.bytes();
    	self.stack_push(memory,((self.pc - 1) >> 8) as u8);
    	self.stack_push(memory,((self.pc -1) % 256) as u8);
    	self.pc = (rom.prg_rom[(self.pc - 2) as usize] as u16) + 256 * (rom.prg_rom[(self.pc - 1) as usize] as u16);
    	self.cycle = self.cycle + cycle;
    }
    fn AND(&mut self,cycle:usize,addressing:Addressing,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	self.pc = self.pc + addressing.bytes();
    	self.registers[0] = self.registers[0] & addressing.read_value(&self,memory,ppu,apu,rom);
    	
    	self.SetStatusByResult(self.registers[0]);
    	self.cycle = self.cycle + cycle + addressing.extra_cycle(self,memory,rom);
    }
    fn BIT(&mut self,cycle:usize,addressing:Addressing,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	println!("BIT");
    	self.pc = self.pc + addressing.bytes();
		self.SetStatusByResult(self.registers[0] & addressing.read_value(&self,memory,ppu,apu,rom));
		self.registers[4] = (self.registers[4] % 64) + ((addressing.read_value(&self,memory,ppu,apu,rom) >> 6) & 1) * 64;
    	self.cycle = self.cycle + cycle;
    }
    fn ROL(&mut self,cycle:usize,addressing:Addressing,memory: &mut memory::CpuRam, ppu: &mut ppu::Ppu,apu: &mut apu::Apu,rom: &rom::Rom){
    	println!("ROL");
    	self.pc = self.pc + addressing.bytes();
    	self.registers[4] = self.registers[4] | (addressing.read_value(&self,memory,ppu,apu,rom) >> 7);
    	addressing.write_value(self,memory,ppu,apu,rom,(addressing.read_value(&self,memory,ppu,apu,rom) << 1) + (self.registers[4] & 1u8));
    	self.SetStatusByResult(addressing.read_value(&self,memory,ppu,apu,rom));
    	self.cycle = self.cycle + cycle;
    }


    fn OPCODE(&mut self,cycle:usize,addressing:Addressing,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom){
    	println!("OPCODE");
    	self.pc = self.pc + addressing.bytes();
    	self.SetStatusByResult(addressing.read_value(&self,memory,ppu,apu,rom));
    	self.cycle = self.cycle + cycle;
    }
}