pub struct CpuRam {
  pub wram: Vec<u8>,
}

impl CpuRam {
	pub fn new()->CpuRam{
		CpuRam {
		  wram: vec![0;2*1024*8]
		}
	}
}