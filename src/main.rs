#![allow(warnings)]
mod cpu;
mod memory;
mod ppu;
mod apu;
mod instruction;
mod rom;
use rom::load_nes_file;

fn main() {
	let mut cpu = cpu::Cpu::new();
	let mut memory = memory::CpuRam::new();
	let mut ppu =ppu::Ppu::new();
	let mut apu =apu::Apu::new();

	let divided_rom;
	//header,trainer,prg_rom,chr_rom,pc_irom,pc_prom

	match load_nes_file("./rom/sample1.nes"){
		Ok(v) => divided_rom = rom::Rom::new(v),
		Err(err) => panic!("{}", err),
	}
	cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
	cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
}