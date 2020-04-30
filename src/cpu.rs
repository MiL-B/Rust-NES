use super::memory;
use super::ppu;
use super::apu;

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
}

impl Cpu {
	pub fn new()->Cpu{
		Cpu {
            registers: vec![
            0,
            0,
            0,
            0,
            0b0010_0000,
            ],
			pc: 0,
        }
	}
	pub fn read_memory(&self, address: u16,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu) -> u8
{

		if address < 0x2000 {
			return memory.wram[(address % 0x0800) as usize];
		}
		else if address < 0x4000 {
			return ppu.registers[((address - 0x2000) % 8) as usize];
		}else if address < 0x4018{
			return apu.registers[(address - 0x4000) as usize];
		}else if address < 0x4020{
			return apu.registers_test[(address - 0x4018) as usize];
		}else{//未実装！！！！！！！！！！！
			return 0;// Cartridge space: PRG ROM, PRG RAM, and mapper registers (See Note)  
		}
	}
}