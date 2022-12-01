// // use cgmath::Zero;
//
// // use crate::glenum::DrawType;
// use crate::shader::{Shader, ShaderProgram};
// // use crate::vertex::{Vao, Vbo, Vertex};
//
// // static VERTICES: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
// static VERTICES: [Vertex; 3] = [
//     Vertex {
//         pos: cgmath::Vector3 {
//             x: -0.5,
//             y: -0.5,
//             z: 0.0,
//         },
//         // normal: cgmath::Vector3::default(),
//         // texture: cgmath::Vector2::zero(),
//     },
//     Vertex {
//         pos: cgmath::Vector3 {
//             x: 0.5,
//             y: -0.5,
//             z: 0.0,
//         },
//         // normal: cgmath::Vector3::zero(),
//         // texture: cgmath::Vector2::zero(),
//     },
//     Vertex {
//         pos: cgmath::Vector3 {
//             x: 0.0,
//             y: 0.5,
//             z: 0.0,
//         },
//         // normal: cgmath::Vector3::zero(),
//         // texture: cgmath::Vector2::zero(),
//     },
// ];
//
// pub struct RenderContext {
//     vao: Vao,
//     _vbos: Vec<Vbo>,
//     shader_program: ShaderProgram,
// }
//
// impl RenderContext {
//     // pub fn new(vao: Vao, shader_program: ShaderProgram) -> RenderContext {
//     //     Self {
//     //         vao,
//     //         shader_program,
//     //     }
//     // }
//
//     pub fn sample_scene() -> Self {
//         let vert_shader = Shader::from_file("shaders/vert.glsl", gl::VERTEX_SHADER).unwrap();
//         let frag_shader = Shader::from_file("shaders/frag.glsl", gl::FRAGMENT_SHADER).unwrap();
//         let triangle_program = ShaderProgram::from_shaders([vert_shader, frag_shader]).unwrap();
//
//         let mut vao = Vao::new();
//         let vbos = vao.as_context(|vao_context| {
//             let mut vbo = Vbo::new();
//             vbo.fill_with(vao_context, &VERTICES, DrawType::Static);
//
//             vec![vbo]
//         });
//
//         Self {
//             vao,
//             _vbos: vbos,
//             shader_program: triangle_program,
//         }
//     }
//
//     pub fn render(&mut self) {
//         self.vao.as_context(|vao_context| {
//             self.shader_program.as_context(|program_context| unsafe {
//                 gl::ClearColor(0.6, 0.0, 0.8, 1.0);
//                 gl::Clear(gl::COLOR_BUFFER_BIT);
//
//                 gl::DrawArrays(gl::TRIANGLES, 0, 6);
//             });
//         });
//     }
// }

use crate::buffer::*;
use crate::model::Model;

pub struct RenderContext {
    models: Vec<Model>,
}

impl Default for RenderContext {
    fn default() -> Self {
        let models = vec![Model::default()];

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        Self { models }
    }
}

// static VERTICES: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

impl RenderContext {
    pub fn render(&mut self) {
        // println!("render loop");

        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for model in self.models.iter_mut() {
            model.render();
        }

        // println!("-------");

        // unsafe {
        //     let error = gl::GetError();
        //     println!("{error}");
        // }
    }
}
