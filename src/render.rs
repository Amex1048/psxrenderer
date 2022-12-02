use crate::camera::Camera;
use crate::model::{Instance, Model};

use crate::material::Material;
use crate::mesh::Mesh;
use crate::shader::Program;

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) mesh: usize,
    pub(crate) materials: Vec<usize>,
    // pub(crate) programs: Vec<usize>,
    pub(crate) transform: cgmath::Matrix4<f32>,
}

#[derive(Debug)]
pub struct World {
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
    programs: Vec<Program>,
    nodes: Vec<Node>,
    camera: Camera,
}

impl World {
    pub fn from_gltf_file<P: AsRef<std::path::Path>>(path: P) -> Self {
        let vert_shader =
            crate::shader::Shader::from_file("shaders/vert.glsl", gl::VERTEX_SHADER).unwrap();
        let frag_shader =
            crate::shader::Shader::from_file("shaders/frag.glsl", gl::FRAGMENT_SHADER).unwrap();
        let shader = crate::shader::Program::from_shaders([vert_shader, frag_shader]).unwrap();

        let data = crate::gltf::read_from_file(path);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        World {
            meshes: data.0,
            materials: data.1,
            programs: vec![shader],
            nodes: data.2,
            camera: Camera::new(
                cgmath::vec3(0.0, 0.0, 5.0),
                cgmath::vec3(0.0, 0.0, 0.0),
                cgmath::Deg(45.0),
                4.0 / 3.0,
            ),
        }
    }

    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let view = self.camera.view();
        let projection = self.camera.projection();

        for node in self.nodes.iter_mut() {
            let mesh = &mut self.meshes[node.mesh];

            for (i, primitive) in mesh.0.iter_mut().enumerate() {
                let material = &mut self.materials[node.materials[i]];
                let program = &mut self.programs[material.shader.unwrap_or(0)];

                let mvp = projection * view * node.transform;

                program.load_uniform_mat("mvp", false, mvp);

                program.as_context(|| {
                    primitive.render();
                });
            }
        }
    }

    pub fn update_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
        self.camera.set_aspect(width as f32 / height as f32);
    }

    pub fn update(&mut self, input: &crate::InputState, delta: f32) {
        const SHIFT_MULTIPLIER: f32 = 3.5;

        let shift = if input.shift { SHIFT_MULTIPLIER } else { 1.0 };
        let front = if input.w { 1.0 } else { 0.0 } * delta * shift;
        let right = if input.d { 1.0 } else { 0.0 } * delta * shift;
        let back = if input.s { 1.0 } else { 0.0 } * delta * shift;
        let left = if input.a { 1.0 } else { 0.0 } * delta * shift;
        let up = if input.e { 1.0 } else { 0.0 } * delta;
        let down = if input.q { 1.0 } else { 0.0 } * delta;

        let mouse = (
            input.mouse_rel.0 as f32 / 800.0 * delta,
            input.mouse_rel.1 as f32 / 600.0 * delta,
        );

        // println!("{:?}", input.mouse_pos);
        // println!("{:?}", input.mouse_rel);
        // println!("{:?}", mouse);

        self.camera
            .update(front, right, back, left, up, down, mouse);
    }
}

pub struct RenderContext {
    instances: Vec<Instance>,
    camera: Camera,
}

impl Default for RenderContext {
    fn default() -> Self {
        let data = crate::gltf::read_from_file("models/TestBlender/TestBlender.gltf");
        // let data = crate::gltf::read_from_file("models/Duck/Duck.gltf");
        println!("{:?}", data.0);
        println!("{:?}", data.1);
        println!("{:?}", data.2);

        let meshes = crate::gltf::read("models/TestBlender/TestBlender.gltf");
        // assert!(meshes.len() == 1);

        let vert_shader =
            crate::shader::Shader::from_file("shaders/vert.glsl", gl::VERTEX_SHADER).unwrap();
        let frag_shader =
            crate::shader::Shader::from_file("shaders/frag.glsl", gl::FRAGMENT_SHADER).unwrap();
        let shader = crate::shader::Program::from_shaders([vert_shader, frag_shader]).unwrap();

        let instances = vec![Instance {
            model: Model {
                meshes,
                materials: vec![
                    crate::material::Material::default(),
                    crate::material::Material::default(),
                ],
                program: shader,
            },
            transform: cgmath::Matrix4::from_scale(1.0),
        }];

        let camera = Camera::new(
            cgmath::vec3(0.0, 0.0, 5.0),
            cgmath::vec3(0.0, 0.0, 0.0),
            cgmath::Deg(45.0),
            4.0 / 3.0,
        );

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        Self { instances, camera }
    }
}

// static VERTICES: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

impl RenderContext {
    pub fn new(instances: Vec<Instance>) -> Self {
        let camera = Camera::new(
            cgmath::vec3(0.0, 0.0, 5.0),
            cgmath::vec3(0.0, 0.0, 0.0),
            cgmath::Deg(45.0),
            4.0 / 3.0,
        );

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        Self { instances, camera }
    }

    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // let view = cgmath::Matrix4::from_translation(cgmath::vec3(0.0, 0.0, -5.0));
        let view = self.camera.view();
        let projection = self.camera.projection();

        for instance in self.instances.iter_mut() {
            let transform = instance.transform;
            let mvp = projection * view * transform;

            instance.model.program.load_uniform_mat("mvp", false, mvp);
            instance.model.render();
        }

        // unsafe {
        //     let error = gl::GetError();
        //     println!("{error}");
        // }
    }

    pub fn update_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
        self.camera.set_aspect(width as f32 / height as f32);
    }

    pub fn update(&mut self, input: &crate::InputState, delta: f32) {
        const SHIFT_MULTIPLIER: f32 = 3.5;

        let shift = if input.shift { SHIFT_MULTIPLIER } else { 1.0 };
        let front = if input.w { 1.0 } else { 0.0 } * delta * shift;
        let right = if input.d { 1.0 } else { 0.0 } * delta * shift;
        let back = if input.s { 1.0 } else { 0.0 } * delta * shift;
        let left = if input.a { 1.0 } else { 0.0 } * delta * shift;
        let up = if input.e { 1.0 } else { 0.0 } * delta;
        let down = if input.q { 1.0 } else { 0.0 } * delta;

        let mouse = (
            input.mouse_rel.0 as f32 / 800.0 * delta,
            input.mouse_rel.1 as f32 / 600.0 * delta,
        );

        // println!("{:?}", input.mouse_pos);
        // println!("{:?}", input.mouse_rel);
        // println!("{:?}", mouse);

        self.camera
            .update(front, right, back, left, up, down, mouse);
    }
}
