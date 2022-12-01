use crate::buffer::*;
use crate::material::Material;
use crate::vao::Vao;

use crate::shader::vertex::{NORMAL_LOCATION, POSITION_LOCATION, TEXTURE_LOCATION};

use cgmath::{Vector2, Vector3, Zero};

const POS_VBO: usize = 0;
const NOR_VBO: usize = 1;
const TEX_VBO: usize = 2;

#[repr(C)]
pub struct Mesh {
    vertices: Vec<Vector3<f32>>,
    normals: Vec<Vector3<f32>>,
    tex_coords: Vec<Vector2<f32>>,

    indices: Vec<u32>,
    vbos: [Vbo; 3],
    ebo: Ebo,
    vao: Vao,
}

impl Mesh {
    pub(crate) fn new(
        vertices: Vec<Vector3<f32>>,
        normals: Vec<Vector3<f32>>,
        tex_coords: Vec<Vector2<f32>>,
        indices: Vec<u32>,
    ) -> Self {
        let mut vao = Vao::new();
        let mut vbos = [Vbo::new(), Vbo::new(), Vbo::new()];
        let mut ebo = Ebo::new();

        vao.as_context(|| {
            vbos[POS_VBO].fill_with(bytemuck::cast_slice(vertices.as_slice()), DrawType::Static);
            vbos[POS_VBO].set_attrib_ptr(POSITION_LOCATION, 3, ObjectType::Float, false);

            vbos[NOR_VBO].fill_with(bytemuck::cast_slice(normals.as_slice()), DrawType::Static);
            vbos[NOR_VBO].set_attrib_ptr(NORMAL_LOCATION, 3, ObjectType::Float, false);

            vbos[TEX_VBO].fill_with(
                bytemuck::cast_slice(tex_coords.as_slice()),
                DrawType::Static,
            );
            vbos[TEX_VBO].set_attrib_ptr(TEXTURE_LOCATION, 2, ObjectType::Float, false);

            ebo.fill_with(bytemuck::cast_slice(indices.as_slice()), DrawType::Static);
        });

        Self {
            vertices,
            normals,
            tex_coords,
            indices,
            vbos,
            ebo,
            vao,
        }
    }

    pub(crate) fn render(&mut self) {
        self.vao.as_context(|| unsafe {
            // println!("draw {} vertices", self.indices.len());
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len().try_into().unwrap(),
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        });
    }
}

impl Default for Mesh {
    fn default() -> Self {
        let vertices = vec![
            Vector3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            Vector3 {
                x: 0.5,
                y: 0.5,
                z: -0.5,
            },
            Vector3 {
                x: -0.5,
                y: 0.5,
                z: -0.5,
            },
            Vector3 {
                x: -0.5,
                y: 0.5,
                z: 0.5,
            },
            Vector3 {
                x: 0.5,
                y: -0.5,
                z: 0.5,
            },
            Vector3 {
                x: 0.5,
                y: -0.5,
                z: -0.5,
            },
            Vector3 {
                x: -0.5,
                y: -0.5,
                z: -0.5,
            },
            Vector3 {
                x: -0.5,
                y: -0.5,
                z: 0.5,
            },
        ];
        let colors = vec![
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vector3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vector3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        ];
        // let normals = vec![Vector3::zero(), Vector3::zero(), Vector3::zero()];
        let tex_coords = vec![
            Vector2::zero(),
            Vector2::zero(),
            Vector2::zero(),
            Vector2::zero(),
            Vector2::zero(),
            Vector2::zero(),
            Vector2::zero(),
            Vector2::zero(),
        ];
        let indices = vec![
            0, 2, 1, 0, 2, 3, 0, 5, 4, 0, 5, 1, 0, 7, 3, 0, 7, 4, 6, 3, 2, 6, 3, 7, 6, 1, 2, 6, 1,
            5, 6, 4, 7, 6, 4, 5,
        ];

        Mesh::new(vertices, colors, tex_coords, indices)
    }
}
