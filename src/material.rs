// use crate::shader::{Program, Shader};

// use cgmath::prelude::*;

use crate::shader::Program;

#[derive(Debug)]
pub struct Material {
    pub(crate) shader: Option<usize>,
    pub(crate) albedo: Option<usize>,
}

impl Material {
    pub fn choose_shader(&self, _programs: &[Program]) -> usize {
        0
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            shader: None,
            albedo: None,
        }
    }
}
