const DEFAULT_BG_COLOR: u8 = 77;
const DEFAULT_DEPTH: f64 = f64::INFINITY;

pub type FrameBuffer = Vec<u8>;
pub type DepthBuffer = Vec<f64>;

#[derive(Default)]
pub struct Buffers {
    pub f_buffer: FrameBuffer,
    pub z_buffer: DepthBuffer,
}

impl Buffers {
    pub fn new(width: u32, height: u32) -> Self {
        let size = width * height;
        Self {
            f_buffer: vec![DEFAULT_BG_COLOR; (size * 4) as usize],
            z_buffer: vec![DEFAULT_DEPTH; size as usize],
        }
    }

    pub fn reset(&mut self) {
        self.f_buffer.fill(DEFAULT_BG_COLOR);
        self.z_buffer.fill(DEFAULT_DEPTH);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let size = width * height;
        self.f_buffer.resize((size * 4) as usize, DEFAULT_BG_COLOR);
        self.z_buffer.resize(size as usize, DEFAULT_DEPTH);
    }

    pub fn mut_buffers(&mut self) -> (&mut FrameBuffer, &mut DepthBuffer) {
        (&mut self.f_buffer, &mut self.z_buffer)
    }
}
