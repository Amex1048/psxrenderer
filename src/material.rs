// use crate::shader::{Program, Shader};

// use cgmath::prelude::*;

pub struct Material {
    // pub(crate) shader: Program,
}

impl Default for Material {
    fn default() -> Self {
        Self {}
        // let vert_shader = Shader::from_file("shaders/vert.glsl", gl::VERTEX_SHADER).unwrap();
        // let frag_shader = Shader::from_file("shaders/frag.glsl", gl::FRAGMENT_SHADER).unwrap();
        // let mut shader = Program::from_shaders([vert_shader, frag_shader]).unwrap();
        // let time = std::time::SystemTime::now();

        // let model = cgmath::Matrix4::from_translation(cgmath::vec3(0.0, 1.0, 0.0));
        // let model = cgmath::Matrix4::from_angle_y(cgmath::Deg(30.0)) * model;
        // shader.load_uniform_mat("model", false, model);
        //
        // let view = cgmath::Matrix4::from_translation(cgmath::vec3(0.0, 0.0, -5.0));
        // shader.load_uniform_mat("view", false, view);
        //
        // let projection = cgmath::perspective(cgmath::Deg(45.0), 800.0 / 600.0, 0.1, 100.0);
        // shader.load_uniform_mat("projection", false, projection);

        // Self { shader }
    }
}
