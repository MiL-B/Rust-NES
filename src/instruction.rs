use super::memory;
use super::ppu;
use super::apu;
use super::cpu::Cpu;
use super::rom;


impl Cpu {
	pub fn exec(&mut self,divided_rom:&rom::Rom,memory: &memory::CpuRam, ppu: &ppu::Ppu,apu: &apu::Apu){
		//アドレッシングモードをどうするかは検討中。
		match divided_rom.prg_rom[self.pc as usize]{
			0x78 => self.SEI(),
			_ => println!("{:x} is unimplemented!!", divided_rom.prg_rom[self.pc as usize]),
		}
    }
    fn SEI(&mut self){
    	println!("SEI");
    	self.registers[4] = self.registers[4] | 0b0000_0100;
    	self.pc = self.pc + 1;
    }
}