#[derive(Clone)]
pub struct Framebuffer {
    pub addr: *mut u32,
    pub height: usize,
    pub width: usize,
}
