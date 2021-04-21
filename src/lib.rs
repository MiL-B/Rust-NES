#![allow(warnings)]
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
    assert_eq!(cpu.registers[3], 0xFD);
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
    let mut divided_rom;
    match load_nes_file("./rom/sample1.nes"){
        Ok(v) => divided_rom = rom::Rom::new(v),
        Err(err) => panic!("{}", err),
    }



	memory.wram[1]=1;
    memory.wram[2]=2;
    memory.wram[3]=3;
    memory.wram[4]=4;

    assert_eq!(cpu.read_memory(1,&memory,&ppu,&apu,&divided_rom), 1);
    assert_eq!(cpu.read_memory(2+0x0800,&memory,&ppu,&apu,&divided_rom), 2);
    assert_eq!(cpu.read_memory(3+0x1000,&memory,&ppu,&apu,&divided_rom), 3);
    assert_eq!(cpu.read_memory(4+0x1800,&memory,&ppu,&apu,&divided_rom), 4);



    ppu.registers[0] = 1;
    ppu.registers[1] = 2;
    ppu.registers[2] = 3;
    ppu.registers[3] = 4;
    ppu.registers[4] = 5;
    ppu.registers[5] = 6;
    ppu.registers[6] = 7;
    ppu.registers[7] = 8;

    assert_eq!(cpu.read_memory(0x2000+0,&memory,&ppu,&apu,&divided_rom), 1);
    assert_eq!(cpu.read_memory(0x2000+1,&memory,&ppu,&apu,&divided_rom), 2);
    assert_eq!(cpu.read_memory(0x2000+2,&memory,&ppu,&apu,&divided_rom), 3);
    assert_eq!(cpu.read_memory(0x2000+3,&memory,&ppu,&apu,&divided_rom), 4);
    assert_eq!(cpu.read_memory(0x2000+4,&memory,&ppu,&apu,&divided_rom), 5);
    assert_eq!(cpu.read_memory(0x2000+5,&memory,&ppu,&apu,&divided_rom), 6);
    assert_eq!(cpu.read_memory(0x2000+6,&memory,&ppu,&apu,&divided_rom), 7);
    assert_eq!(cpu.read_memory(0x2000+7,&memory,&ppu,&apu,&divided_rom), 8);
    assert_eq!(cpu.read_memory(0x2000+8,&memory,&ppu,&apu,&divided_rom), 1);



    apu.registers[0]=1;
    apu.registers[1]=2;
    apu.registers[23]=3;

    assert_eq!(cpu.read_memory(0x4000+0,&memory,&ppu,&apu,&divided_rom), 1);
    assert_eq!(cpu.read_memory(0x4000+1,&memory,&ppu,&apu,&divided_rom), 2);
    assert_eq!(cpu.read_memory(0x4000+23,&memory,&ppu,&apu,&divided_rom), 3);



    apu.registers_test[0]=1;
    apu.registers_test[1]=2;
    apu.registers_test[7]=3;

    assert_eq!(cpu.read_memory(0x4018,&memory,&ppu,&apu,&divided_rom), 1);
    assert_eq!(cpu.read_memory(0x4019,&memory,&ppu,&apu,&divided_rom), 2);
    assert_eq!(cpu.read_memory(0x401F,&memory,&ppu,&apu,&divided_rom), 3);

    assert_eq!(cpu.read_memory(0x8000,&memory,&ppu,&apu,&divided_rom), 0x78);
}

#[test]
fn divide_rom() {
    let mut divided_rom;
    match load_nes_file("./rom/sample1.nes"){
        Ok(v) => divided_rom = rom::Rom::new(v),
        Err(err) => panic!("{}", err),
    }
    assert_eq!(divided_rom.header.len(), 16);
    assert_eq!(divided_rom.trainer.len(), 0);
    assert_eq!(divided_rom.prg_rom.len(), 32768);
    assert_eq!(divided_rom.chr_rom.len(), 8192);
    assert_eq!(divided_rom.pc_irom.len(), 0);
    assert_eq!(divided_rom.pc_prom.len(), 0);
}

#[test]
fn instructions() {
    /*
    [120, 162, 255, 154, 169, 0, 141, 0, 32, 141, 1, 32, 
    169, 63, 141, 6, 32, 169, 0, 141, 6, 32, 
    162, 0, 160, 16, 189, 81, 128, 141, 7, 32,
    232, 136, 208, 246, 169, 33, 141, 6, 32, 169,
    201, 141, 6, 32, 162, 0, 160, 13, 189, 97, 
    128, 141, 7, 32, 232, 136, 208, 246, 169, 0, 
    141, 5, 32, 141, 5, 32, 169, 8, 141, 0, 
    32, 169, 30, 141, 1, 32, 76, 78, 128, 15, 
    0, 16, 32, 15, 6, 22, 38, 15, 8, 24, 
    40, 15, 10, 26, 42, 72, 69, 76, 76, 79,
    44, 32, 87, 79, 82, 76, 68, 33,
    ......  128, 0, 0]
    */
    let mut cpu = cpu::Cpu::new();
    let mut memory = memory::CpuRam::new();
    let mut ppu =ppu::Ppu::new();
    let mut apu =apu::Apu::new();

    let mut divided_rom;
    //header,trainer,prg_rom,chr_rom,pc_irom,pc_prom

    match load_nes_file("./rom/sample1.nes"){
        Ok(v) => divided_rom = rom::Rom::new(v),
        Err(err) => panic!("{}", err),
    }

    //0x78
    divided_rom.prg_rom[0] = 0x78;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.cycle,2);
    assert_eq!(cpu.registers[4], 0b100100);
    cpu.pc = 0;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[4], 0b100100);
    assert_eq!(cpu.pc,1);
    

    //0x00
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x00;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.pc,0);
    assert_eq!((cpu.registers[4] >> 4) & 1u8,1);
    assert_eq!(cpu.cycle,7);

    //0x01
    cpu = cpu::Cpu::new();
    cpu.registers[1] = 4;
    memory.wram[166] = 0x01;
    memory.wram[167] = 0x80;
    divided_rom.prg_rom[0] = 0x01;
    divided_rom.prg_rom[1] = 0xA2;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],162);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,6);

    //0x05
    cpu = cpu::Cpu::new();
    memory.wram[167] = 0x83;
    cpu.registers[0] = 8;
    divided_rom.prg_rom[0] = 0x05;
    divided_rom.prg_rom[1] = 0xA7;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],139);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,3);

    //0x06
    cpu = cpu::Cpu::new();
    memory.wram[167] = 0x40;
    divided_rom.prg_rom[0] = 0x06;
    divided_rom.prg_rom[1] = 0xA7;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(memory.wram[167],128);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,5);
    cpu.pc = 0;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(memory.wram[167],0);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,0);

    //0x07
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x08;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(memory.wram[0x01FD],0b0010_0000);
    assert_eq!(cpu.cycle,3);

    //0x09
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x09;
    divided_rom.prg_rom[1] = 0b1000_1000;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0b1000_1000);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,2);

    //0x0A
    cpu = cpu::Cpu::new();
    cpu.registers[0] = 0x40;
    divided_rom.prg_rom[0] = 0x0A;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],128);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,2);
    cpu.pc = 0;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,0);


    //0x0D
    cpu = cpu::Cpu::new();
    memory.wram[0x0701] = 0b1000_1000;
    divided_rom.prg_rom[0] = 0x0D;
    divided_rom.prg_rom[1] = 0x01;
    divided_rom.prg_rom[2] = 0x07;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0b1000_1000);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,4);

    //0x0E
    cpu = cpu::Cpu::new();
    memory.wram[0x0701] = 0b0100_0000;
    divided_rom.prg_rom[0] = 0x0E;
    divided_rom.prg_rom[1] = 0x01;
    divided_rom.prg_rom[2] = 0x07;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(memory.wram[0x0701],128);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,6);
    cpu.pc = 0;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(memory.wram[0x0701],0);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,0);


    //0x10
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x10;
    divided_rom.prg_rom[1] = 0b0000_1000;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.pc,10);
    assert_eq!(cpu.cycle,3);
    divided_rom.prg_rom[10] = 0x10;
    divided_rom.prg_rom[11] = 0b1111_1000;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.pc,4);
    cpu = cpu::Cpu::new();
    cpu.pc = 200;
    divided_rom.prg_rom[200] = 0x10;
    divided_rom.prg_rom[201] = 0b0111_1111;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.pc,329);
    assert_eq!(cpu.cycle,4);
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x10;
    divided_rom.prg_rom[1] = 0b0111_1111;
    cpu.registers[4] = cpu.registers[4] | 0b1000_0000;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.cycle,2);
    assert_eq!(cpu.pc,2);


    //0x11
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x11;
    divided_rom.prg_rom[1] = 0b0000_1000;
    memory.wram[0x0008] = 0x01;
    memory.wram[0x0009] = 0x07;
    memory.wram[0x0705] = 0b1000_1000;
    cpu.registers[0] = 0b1010_0010;
    cpu.registers[2] = 4;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0b1010_1010);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,5);

    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x11;
    divided_rom.prg_rom[1] = 0b0000_1000;
    memory.wram[0x0008] = 0xFF;
    memory.wram[0x0009] = 0x06;
    memory.wram[0x0703] = 0b1000_1000;
    cpu.registers[0] = 0b1010_0010;
    cpu.registers[2] = 4;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0b1010_1010);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,6);

    //0x15
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x15;
    divided_rom.prg_rom[1] = 0b0000_1000;
    memory.wram[0x000C] = 0b1000_1000;
    cpu.registers[0] = 0b1010_0010;
    cpu.registers[1] = 4;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0b1010_1010);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,4);

    //0x16
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x16;
    divided_rom.prg_rom[1] = 0xA3;
    memory.wram[167] = 0x40;
    cpu.registers[1] = 4;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(memory.wram[167],128);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,6);
    cpu.pc = 0;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(memory.wram[167],0);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,0);

    //0x18
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x18;
    cpu.registers[4] = 0b0010_0000;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[4],0b0010_0000);
    assert_eq!(cpu.cycle,2);
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x18;
    cpu.registers[4] = 0b0010_0001;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[4],0b0010_0000);

    //0x19
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x19;
    divided_rom.prg_rom[1] = 0x01;
    divided_rom.prg_rom[2] = 0x07;
    memory.wram[0x0705] = 0b1000_1000;
    cpu.registers[0] = 0b1010_0010;
    cpu.registers[2] = 4;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0b1010_1010);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,4);

    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x19;
    divided_rom.prg_rom[1] = 0xFF;
    divided_rom.prg_rom[2] = 0x06;
    memory.wram[0x0703] = 0b1000_1000;
    cpu.registers[0] = 0b1010_0010;
    cpu.registers[2] = 4;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0b1010_1010);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,5);

    //0x1D
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x1D;
    divided_rom.prg_rom[1] = 0x01;
    divided_rom.prg_rom[2] = 0x07;
    memory.wram[0x0705] = 0b1000_1000;
    cpu.registers[0] = 0b1010_0010;
    cpu.registers[1] = 4;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0b1010_1010);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,4);

    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x1D;
    divided_rom.prg_rom[1] = 0xFF;
    divided_rom.prg_rom[2] = 0x06;
    memory.wram[0x0703] = 0b1000_1000;
    cpu.registers[0] = 0b1010_0010;
    cpu.registers[1] = 4;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0b1010_1010);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,5);

    //0x1E
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x1E;
    divided_rom.prg_rom[1] = 0x01;
    divided_rom.prg_rom[2] = 0x07;
    memory.wram[0x0705] = 0x40;
    cpu.registers[1] = 4;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(memory.wram[0x0705],128);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,7);
    cpu.pc = 0;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(memory.wram[0x0705],0);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,0);

    //0x20
    cpu = cpu::Cpu::new();
    divided_rom.prg_rom[0] = 0x20;
    divided_rom.prg_rom[1] = 0x01;
    divided_rom.prg_rom[2] = 0x07;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.pc,0x701);
    assert_eq!(memory.wram[0x01FD], 0x0000);
    assert_eq!(memory.wram[0x01FC], 0x0002);
    assert_eq!(cpu.cycle,6);

    //0x21
    cpu = cpu::Cpu::new();
    cpu.registers[1] = 4;
    memory.wram[166] = 0x01;
    memory.wram[167] = 0x80;
    divided_rom.prg_rom[0] = 0x21;
    divided_rom.prg_rom[1] = 0xA2;
    cpu.registers[0] = 0b1000_0001;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],128);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,1);
    assert_eq!(cpu.cycle,6);

    //0x24
    cpu = cpu::Cpu::new();
    memory.wram[167] = 0b0100_0010;
    cpu.registers[0] = 0b1000_0001;
    divided_rom.prg_rom[0] = 0x24;
    divided_rom.prg_rom[1] = 0xA7;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 6) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,1);
    assert_eq!(cpu.cycle,3);

    //0x25
    cpu = cpu::Cpu::new();
    memory.wram[167] = 0b1010_1010;
    cpu.registers[0] = 0b0000_1000;
    divided_rom.prg_rom[0] = 0x25;
    divided_rom.prg_rom[1] = 0xA7;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(cpu.registers[0],0b0000_1000);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,0);
    assert_eq!(cpu.cycle,3);

    //0x26
    cpu = cpu::Cpu::new();
    memory.wram[167] = 0b1010_1010;
    divided_rom.prg_rom[0] = 0x26;
    divided_rom.prg_rom[1] = 0xA7;
    cpu.exec(&divided_rom,&mut memory,&mut ppu,&mut apu);
    assert_eq!(memory.wram[167],0b0101_0101);
    assert_eq!((cpu.registers[4] >> 0) & 1u8,1);
    assert_eq!((cpu.registers[4] >> 1) & 1u8,0);
    assert_eq!((cpu.registers[4] >> 7) & 1u8,0);
    assert_eq!(cpu.cycle,5);

    //0x27
    
    


}


#[test]
fn push_pop() {
    let mut cpu = cpu::Cpu::new();
    let mut memory = memory::CpuRam::new();

    cpu.stack_push(&mut memory,1);
    assert_eq!(memory.wram[0x01FD], 1);
    assert_eq!(cpu.registers[3],0xFC);
    assert_eq!(cpu.stack_pop(&memory),1);
    assert_eq!(cpu.registers[3],0xFD);
}