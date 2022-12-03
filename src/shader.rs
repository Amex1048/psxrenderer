mod program;
mod shader;

pub mod uniform;

pub use program::Program;
pub use shader::Shader;

pub mod vertex {
    pub(crate) const POSITION_LOCATION: u32 = 0;
    pub(crate) const NORMAL_LOCATION: u32 = 1;
    pub(crate) const TEXTURE_LOCATION: u32 = 2;
}

pub mod fragment {
    pub(crate) const ALBEDO_TEX: u32 = 0;
    pub(crate) const NORMAL_MAP_TEX: u32 = 1;
}
