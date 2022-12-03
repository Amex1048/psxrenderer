// use crate::shader::{Program, Shader};

// use cgmath::prelude::*;

use crate::shader::Program;

#[derive(Debug)]
pub struct Material {
    pub(crate) shader: Option<usize>,
    pub(crate) albedo: Option<usize>,
    pub(crate) base_color: Option<cgmath::Vector3<f32>>,
}

impl Material {
    pub fn choose_shader(&self, _programs: &[Program]) -> usize {
        if self.albedo.is_some() {
            1
        } else {
            0
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            shader: None,
            albedo: None,
            base_color: Some(cgmath::vec3(0.5, 0.0, 0.2)),
        }
    }
}
