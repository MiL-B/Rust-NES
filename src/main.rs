mod cpu;
mod memory;
mod ppu;
mod apu;

fn main() {
	let mut cpu = cpu::Cpu::new();
	let mut memory = memory::CpuRam::new();
	let mut ppu =ppu::Ppu::new();
	let mut apu =apu::Apu::new();

	memory.wram[2]=40;
	ppu.registers[2] = 2;
	println!("{}", cpu.read_memory(2,&memory,&ppu,&apu));
	println!("{}", cpu.read_memory(0x2000+2,&memory,&ppu,&apu));
}