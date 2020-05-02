use std::io;
use std::fs::File;
use std::io::{Read,Error, ErrorKind};

pub struct Rom {
  pub header: Vec<u8>,
  pub trainer: Vec<u8>,
  pub prg_rom: Vec<u8>,
  pub chr_rom: Vec<u8>,
  pub pc_irom: Vec<u8>,
  pub pc_prom: Vec<u8>,
}

impl Rom {
    pub fn new(loaded_rom:(Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>))->Rom{
        Rom {
            header: loaded_rom.0,
            trainer: loaded_rom.1,
            prg_rom: loaded_rom.2,
            chr_rom: loaded_rom.3,
            pc_irom: loaded_rom.4,
            pc_prom: loaded_rom.5,
        }
    }
}


pub fn load_nes_file(path:&str)->io::Result<(Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>)>{
	let mut file = File::open(path)?;
    let mut header= Vec::new();
    file.read_to_end(&mut header)?;

    if (header[0],header[1],header[2],header[3])!=(0x4E,0x45,0x53,0x1A){
    	return Err(Error::new(ErrorKind::Other, "Loaded file is broken!!"))
    }

    let mut trainer = header.split_off(16);

    let mut prg_rom;
    if (header[6] >> 2) & 1u8 == 1{
    	prg_rom = trainer.split_off(512);
    }else {
    	prg_rom = trainer;
    	trainer = [].to_vec();
    }

    let mut chr_rom = prg_rom.split_off(16384 * header[4] as usize);
    let mut pc_irom = chr_rom.split_off(8192 * header[5] as usize);

    let mut pc_prom;
    if pc_irom.len()>0{
    	pc_prom = pc_irom.split_off(8192);
    }else {
    	pc_prom = [].to_vec();
    }
	
	Ok((header,trainer,prg_rom,chr_rom,pc_irom,pc_prom))
}