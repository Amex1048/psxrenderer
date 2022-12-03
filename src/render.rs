use crate::camera::Camera;

use crate::material::Material;
use crate::mesh::Mesh;
use crate::shader::Program;
use crate::texture::Texture2D;

#[derive(Debug)]
pub struct AssetStorage {
    pub(crate) meshes: Vec<Mesh>,
    pub(crate) materials: Vec<Material>,
    pub(crate) textures2d: Vec<Texture2D>,
    pub(crate) programs: Vec<Program>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) mesh: usize,
    pub(crate) materials: Vec<usize>,
    pub(crate) transform: cgmath::Matrix4<f32>,
}

#[derive(Debug)]
pub struct World {
    assets: AssetStorage,
    nodes: Vec<Node>,
    camera: Camera,
}

impl World {
    pub fn from_gltf_file<P: AsRef<std::path::Path>>(path: P) -> Self {
        let vert_shader =
            crate::shader::Shader::from_file("shaders/color/vert.glsl", gl::VERTEX_SHADER).unwrap();
        let frag_shader =
            crate::shader::Shader::from_file("shaders/color/frag.glsl", gl::FRAGMENT_SHADER)
                .unwrap();
        let shader_color =
            crate::shader::Program::from_shaders([vert_shader, frag_shader]).unwrap();

        let vert_shader =
            crate::shader::Shader::from_file("shaders/texture/vert.glsl", gl::VERTEX_SHADER)
                .unwrap();
        let frag_shader =
            crate::shader::Shader::from_file("shaders/texture/frag.glsl", gl::FRAGMENT_SHADER)
                .unwrap();
        let shader_texture =
            crate::shader::Program::from_shaders([vert_shader, frag_shader]).unwrap();

        let (mut storage, nodes) = crate::gltf::read_from_file(path);
        storage.programs = vec![shader_color, shader_texture];

        for material in storage.materials.iter_mut() {
            let shader = material.choose_shader(&storage.programs);
            // println!("{shader}");
            material.shader = Some(shader);
        }

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        // println!("{:?}", storage);

        World {
            // meshes: data.0,
            // materials: data.1,
            // programs: vec![shader],
            assets: storage,
            nodes,
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
            let mesh = &mut self.assets.meshes[node.mesh];

            for (i, primitive) in mesh.0.iter_mut().enumerate() {
                let material = &mut self.assets.materials[node.materials[i]];
                let program = &mut self.assets.programs[material.shader.unwrap_or(0)];

                let mvp = projection * view * node.transform;

                program.load_uniform_mat("mvp", false, mvp);

                if let Some(albedo_index) = material.albedo {
                    let texture = &self.assets.textures2d[albedo_index];
                    program.load_uniform_texture2d(texture, crate::shader::fragment::ALBEDO_TEX);
                    program.load_uniform_vec(
                        "albedo",
                        cgmath::vec1(crate::shader::fragment::ALBEDO_TEX as i32),
                    );
                } else {
                    program.load_uniform_vec("color", material.base_color.unwrap());
                }

                program.as_context(|| {
                    primitive.render();
                });
            }
        }

        unsafe {
            let error = gl::GetError();
            assert!(error == gl::NO_ERROR, "{error:x?}");
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
