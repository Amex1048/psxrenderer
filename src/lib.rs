mod buffer;
pub mod gltf;
mod material;
mod mesh;
mod model;
pub mod render;
mod shader;
mod vao;

pub(crate) trait GlObject {
    fn glid(&self) -> gl::types::GLuint;
}
