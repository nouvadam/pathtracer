pub struct BlueNoiseTexture {
	blue_noise: Vec<u8>,
	size: usize
}

impl BlueNoiseTexture {
	pub fn new(file: &str, size: usize) -> BlueNoiseTexture {
		
		//reading blue noise texture
		use std::fs::File;
		let decoder = png::Decoder::new(File::open(file).unwrap());
		let (info, mut reader) = decoder.read_info().unwrap();
		// Allocate the output buffer.
		let mut blue_noise = vec![0; info.buffer_size()];
		// Read the next frame. Currently this function should only called once.
		// The default options
		reader.next_frame(&mut blue_noise).unwrap();
		
		BlueNoiseTexture{
			blue_noise,
			size
		}
	}
	
	pub fn read(&self, x: u32, y: u32) -> u8 {
		//wrapping read from blue noise texture
		self.blue_noise[((x%(self.size as u32)) as usize)+(((y%(self.size as u32)) as usize))*self.size]
	}
}