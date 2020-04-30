mod cpu;
mod instruction;
mod memory;
mod ppu;
mod apu;
mod rom;
use rom::load_nes_file;

#[test]
fn cpu_init() {
    let cpu = cpu::Cpu::new();
    assert_eq!(cpu.registers[0], 0);
    assert_eq!(cpu.registers[1], 0);
    assert_eq!(cpu.registers[2], 0);
    assert_eq!(cpu.registers[3], 0);
    assert_eq!(cpu.registers[4], 32);
    assert_eq!(cpu.pc, 0);
}


#[test]
fn cpu_ram_init() {
    let cpu_ram = memory::CpuRam::new();
    assert_eq!(cpu_ram.wram.len(), 2*1024*8);
}

#[test]
fn read_cpu_memory() {
    let mut cpu = cpu::Cpu::new();
	let mut memory = memory::CpuRam::new();
    let mut ppu =ppu::Ppu::new();
    let mut apu =apu::Apu::new();



	memory.wram[1]=1;
    memory.wram[2]=2;
    memory.wram[3]=3;
    memory.wram[4]=4;

    assert_eq!(cpu.read_memory(1,&memory,&ppu,&apu), 1);
    assert_eq!(cpu.read_memory(2+0x0800,&memory,&ppu,&apu), 2);
    assert_eq!(cpu.read_memory(3+0x1000,&memory,&ppu,&apu), 3);
    assert_eq!(cpu.read_memory(4+0x1800,&memory,&ppu,&apu), 4);



    ppu.registers[0] = 1;
    ppu.registers[1] = 2;
    ppu.registers[2] = 3;
    ppu.registers[3] = 4;
    ppu.registers[4] = 5;
    ppu.registers[5] = 6;
    ppu.registers[6] = 7;
    ppu.registers[7] = 8;

    assert_eq!(cpu.read_memory(0x2000+0,&memory,&ppu,&apu), 1);
    assert_eq!(cpu.read_memory(0x2000+1,&memory,&ppu,&apu), 2);
    assert_eq!(cpu.read_memory(0x2000+2,&memory,&ppu,&apu), 3);
    assert_eq!(cpu.read_memory(0x2000+3,&memory,&ppu,&apu), 4);
    assert_eq!(cpu.read_memory(0x2000+4,&memory,&ppu,&apu), 5);
    assert_eq!(cpu.read_memory(0x2000+5,&memory,&ppu,&apu), 6);
    assert_eq!(cpu.read_memory(0x2000+6,&memory,&ppu,&apu), 7);
    assert_eq!(cpu.read_memory(0x2000+7,&memory,&ppu,&apu), 8);
    assert_eq!(cpu.read_memory(0x2000+8,&memory,&ppu,&apu), 1);



    apu.registers[0]=1;
    apu.registers[1]=2;
    apu.registers[23]=3;

    assert_eq!(cpu.read_memory(0x4000+0,&memory,&ppu,&apu), 1);
    assert_eq!(cpu.read_memory(0x4000+1,&memory,&ppu,&apu), 2);
    assert_eq!(cpu.read_memory(0x4000+23,&memory,&ppu,&apu), 3);



    apu.registers_test[0]=1;
    apu.registers_test[1]=2;
    apu.registers_test[7]=3;

    assert_eq!(cpu.read_memory(0x4018,&memory,&ppu,&apu), 1);
    assert_eq!(cpu.read_memory(0x4019,&memory,&ppu,&apu), 2);
    assert_eq!(cpu.read_memory(0x401F,&memory,&ppu,&apu), 3);
}

#[test]
fn divide_rom() {
    let mut divided_rom;
    match load_nes_file("./rom/sample1.nes"){
        Ok(v) => divided_rom = v,
        Err(err) => panic!("{}", err),
    }
    assert_eq!(divided_rom.0.len(), 16);
    assert_eq!(divided_rom.1.len(), 0);
    assert_eq!(divided_rom.2.len(), 32768);
    assert_eq!(divided_rom.3.len(), 8192);
    assert_eq!(divided_rom.4.len(), 0);
}