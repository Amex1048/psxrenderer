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
    screen: (i32, i32, i32, i32),
}

const BASE_RENDER_WIDTH: u32 = 320;
const BASE_RENDER_HEIGHT: u32 = 240;
const BASE_PIXEL_COUNT: u32 = BASE_RENDER_WIDTH * BASE_RENDER_HEIGHT;

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
            material.shader = Some(shader);
        }

        World {
            assets: storage,
            nodes,
            camera: Camera::new(
                cgmath::vec3(0.0, 0.0, 5.0),
                cgmath::vec3(0.0, 0.0, 0.0),
                cgmath::Deg(45.0),
                (BASE_RENDER_WIDTH, BASE_RENDER_HEIGHT),
            ),
            screen: (0, 0, 800, 600),
        }
    }

    pub fn render(&mut self) {
        let view = self.camera.view();
        let projection = self.camera.projection();
        let dimensions = self.camera.dimensions;
        let view_projection = projection * view;

        self.camera
            .framebuffer
            .as_context(|| {
                unsafe {
                    // gl::Disable(gl::DITHER);
                    gl::Viewport(0, 0, dimensions.0 as i32, dimensions.1 as i32);
                    gl::ClearColor(0.6, 0.0, 0.8, 1.0);
                    gl::Enable(gl::DEPTH_TEST);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }

                for node in self.nodes.iter_mut() {
                    let mesh = &mut self.assets.meshes[node.mesh];

                    for (i, primitive) in mesh.0.iter_mut().enumerate() {
                        let material = &mut self.assets.materials[node.materials[i]];
                        let program = &mut self.assets.programs[material.shader.unwrap_or(0)];

                        let mvp = view_projection * node.transform;

                        program.load_uniform_mat("mvp", false, mvp);
                        program.load_uniform_vec(
                            "renderResolution",
                            cgmath::vec2(dimensions.0 as f32, dimensions.1 as f32),
                        );

                        if let Some(albedo_index) = material.albedo {
                            let texture = &self.assets.textures2d[albedo_index];
                            program.load_uniform_texture2d(
                                texture,
                                crate::shader::fragment::ALBEDO_TEX,
                            );
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
                    gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
                    gl::BlitFramebuffer(
                        0,
                        0,
                        dimensions.0 as i32,
                        dimensions.1 as i32,
                        self.screen.0,
                        self.screen.1,
                        self.screen.2,
                        self.screen.3,
                        gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT,
                        gl::NEAREST,
                    );
                }
            })
            .unwrap();

        unsafe {
            let error = gl::GetError();
            assert!(error == gl::NO_ERROR, "{error:x?}");
        }
    }

    pub fn update_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.screen.0 = x;
        self.screen.1 = y;
        self.screen.2 = width;
        self.screen.3 = height;

        let area_width = (width - x).abs();
        let area_height = (height - y).abs();
        let aspect = area_width as f32 / area_height as f32;
        let height = (BASE_PIXEL_COUNT as f32 / aspect).sqrt();
        let width = BASE_PIXEL_COUNT as f32 / height;

        self.camera
            .set_dimensions((width.round() as u32, height.round() as u32));
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
            input.mouse_rel.0 as f32 / self.screen.2 as f32 * delta,
            input.mouse_rel.1 as f32 / self.screen.3 as f32 * delta,
        );

        self.camera
            .update(front, right, back, left, up, down, mouse);
    }
}
