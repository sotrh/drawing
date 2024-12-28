use glam::vec2;

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Line {
    pub a: glam::Vec2,
    pub b: glam::Vec2,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DrawMode {
    Color = 0,
    SDF = 1,
}

impl DrawMode {
    pub fn increment(&mut self) -> Self {
        *self = match *self {
            DrawMode::Color => DrawMode::SDF,
            DrawMode::SDF => DrawMode::Color,
        };

        *self
    }
}

unsafe impl bytemuck::Pod for DrawMode {}
unsafe impl bytemuck::Zeroable for DrawMode {}

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct GeometryInfo {
    pub preview_line: Line,
    pub num_lines: u32,
    pub mode: DrawMode,
    // _padding0: u32,
    // _padding1: u32,
}

impl GeometryInfo {
    pub fn new(num_lines: u32, mode: DrawMode) -> Self {
        Self {
            preview_line: Line {
                a: vec2(0.0, 0.0),
                b: vec2(0.0, 0.0),
            },
            num_lines,
            mode,
            // _padding0: 0,
            // _padding1: 0,
        }
    }
}
