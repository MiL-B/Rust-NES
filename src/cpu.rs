use super::memory;
use super::ppu;
use super::apu;
use super::rom;

pub struct Cpu {
  pub registers: Vec<u8>,
  /*
  registers[0]:A
  registers[1]:X
  registers[2]:Y
  registers[3]:S
  registers[4]:P
  */
  pub pc: u16,
  pub cycle:usize,
}

impl Cpu {
	pub fn new()->Cpu{
		Cpu {
            registers: vec![
            0,
            0,
            0,
            0xFD,
            0b0010_0000,
            ],
			pc: 0,
			cycle:0,
        }
	}
	pub fn read_memory(&self, address: u16,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu,rom: &rom::Rom) -> u8{
		println!("{:?}",address);
		if address < 0x2000 {
			return memory.wram[(address % 0x0800) as usize];
		}
		else if address < 0x4000 {
			return ppu.registers[((address - 0x2000) % 8) as usize];
		}else if address < 0x4018{
			return apu.registers[(address - 0x4000) as usize];
		}else if address < 0x4020{
			return apu.registers_test[(address - 0x4018) as usize];
		}else if (0x7000 <= address) && (address < 0x7200){
			if (rom.header[6] >> 2) & 1u8 == 1{
				return rom.trainer[(address - 0x7000) as usize];
			}else {
				panic!("Refered trainer though trainer is disabled!");
			}
		}else if 0x8000 <= address{
			if rom.header[4] == 1{
				if 0xC000 <= address{
					return rom.prg_rom[(address - 0xC000) as usize];
				}else {
					panic!("rom size is 16kb, but refered as if it is 32kb.");
				}
			}else {
				return rom.prg_rom[(address - 0x8000) as usize];
			}
		}
		else{//未実装！！！！！！！！！！！
			panic!("unimplemented!!");
			//return 0;// Cartridge space: PRG ROM, PRG RAM, and mapper registers (See Note)  
		}
	}

	pub fn write_memory(&self, address: u16,memory: &mut memory::CpuRam, ppu: &mut ppu::Ppu,apu: &mut apu::Apu,value:u8){

		if address < 0x2000 {
			memory.wram[(address % 0x0800) as usize] = value;
		}
		else if address < 0x4000 {
			ppu.registers[((address - 0x2000) % 8) as usize] = value;
		}else if address < 0x4018{
			apu.registers[(address - 0x4000) as usize] = value;
		}else if address < 0x4020{
			apu.registers_test[(address - 0x4018) as usize] = value;
		}else if (0x7000 <= address) && (address < 0x7200){
			panic!("Can't write to Trainer(ROM).");
		}else if 0x8000 <= address{
			panic!("Can't write to prg_rom(ROM).");
		}else{//未実装！！！！！！！！！！！
			panic!("unimplemented!!");// Cartridge space: PRG ROM, PRG RAM, and mapper registers (See Note)  
		}
	}

	pub fn stack_push(&mut self,memory: &mut memory::CpuRam, value:u8){
		memory.wram[0x0100 + self.registers[3] as usize] = value;
		self.registers[3] = self.registers[3] - 1;
	}

	pub fn stack_pop(&mut self,memory: &memory::CpuRam)->u8{
		self.registers[3] = self.registers[3] + 1;
		memory.wram[0x0100 + self.registers[3] as usize]
	}


}