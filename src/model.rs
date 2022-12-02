use crate::material::Material;
use crate::mesh::Mesh;
use crate::shader::{Program, Shader};

pub struct Model {
    pub(crate) meshes: Vec<Mesh>,
    pub(crate) materials: Vec<Material>,
    pub(crate) program: Program,
}

pub struct Instance {
    pub model: Model,
    pub transform: cgmath::Matrix4<f32>,
}

impl Default for Model {
    fn default() -> Self {
        let vert_shader = Shader::from_file("shaders/vert.glsl", gl::VERTEX_SHADER).unwrap();
        let frag_shader = Shader::from_file("shaders/frag.glsl", gl::FRAGMENT_SHADER).unwrap();
        let mut shader = Program::from_shaders([vert_shader, frag_shader]).unwrap();

        // let model = cgmath::Matrix4::from_translation(cgmath::vec3(0.0, 1.0, 0.0));
        // let model = cgmath::Matrix4::from_angle_y(cgmath::Deg(30.0)) * model;
        // shader.load_uniform_mat("model", false, model);
        //
        // let view = cgmath::Matrix4::from_translation(cgmath::vec3(0.0, 0.0, -5.0));
        // shader.load_uniform_mat("view", false, view);
        //
        // let projection = cgmath::perspective(cgmath::Deg(45.0), 800.0 / 600.0, 0.1, 100.0);
        // shader.load_uniform_mat("projection", false, projection);

        Self {
            meshes: vec![Mesh::default()],
            materials: vec![Material::default()],
            program: shader,
        }
    }
}

impl Model {
    pub(crate) fn render(&mut self) {
        self.program.as_context(|| {
            for (mesh, material) in self.meshes.iter_mut().zip(self.materials.iter_mut()) {
                mesh.render();
            }
        });
    }
}
