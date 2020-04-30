use super::memory;
use super::cpu;

pub struct Ppu {
  pub registers: Vec<u8>,
  /*
  registers[0]:PPUCTRL
  registers[1]:PPUMASK
  registers[2]:PPUSTATUS
  registers[3]:OAMADDR
  registers[4]:PPUSCROLL
  registers[5]:PPUADDR
  registers[6]:PPUDATA
  registers[7]:OAMDMA
  */
}

impl Ppu {
	pub fn new()->Ppu{
		Ppu {
			registers: vec![
			0,//PPUCTRL
			0,//PPUMASK
			0,//PPUSTATUS
			0,//OAMADDR
			0,//PPUSCROLL
			0,//PPUADDR
			0,//PPUDATA
			0,//OAMDMA
			],
        }
	}
}