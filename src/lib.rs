mod buffer;
pub mod camera;
mod framebuffer;
pub mod gltf;
mod material;
mod mesh;
pub mod render;
mod shader;
pub mod texture;
mod vao;

pub(crate) trait GlObject {
    fn glid(&self) -> gl::types::GLuint;
}

#[derive(Debug, Default, Clone)]
pub struct InputState {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub e: bool,
    pub q: bool,

    pub shift: bool,

    pub mouse_pos: (i32, i32),
    pub mouse_rel: (i32, i32),
}
