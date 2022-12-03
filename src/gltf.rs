use std::collections::HashMap;
use std::path::Path;

use gltf::buffer::Data;
use gltf::Accessor;
use gltf::Semantic;

use crate::material::Material;
use crate::mesh::{Mesh, Primitive};
use crate::texture::Texture2D;

use crate::render::AssetStorage;
use crate::render::Node;

pub fn read_from_file<P: AsRef<Path>>(path: P) -> (AssetStorage, Vec<Node>) {
    let (document, buffers, images) = gltf::import(path).unwrap();
    assert_eq!(buffers.len(), document.buffers().count());
    assert_eq!(images.len(), document.images().count());

    let mut storage = AssetStorage {
        meshes: Vec::with_capacity(document.meshes().len()),
        materials: Vec::with_capacity(document.materials().len()),
        textures2d: Vec::with_capacity(images.len()),
        programs: Vec::new(),
    };

    // let mut meshes: Vec<Mesh> = Vec::with_capacity(document.meshes().len());
    // let mut materials: Vec<Material> = Vec::with_capacity(document.materials().len());
    // let mut nodes: Vec<Node> = Vec::with_capacity(document.nodes().len());

    let mut material_indexes: Vec<Vec<usize>> = Vec::with_capacity(document.meshes().len());
    for gltf_mesh in document.meshes() {
        let mut primitives = Vec::with_capacity(gltf_mesh.primitives().len());
        let mut material_index = Vec::with_capacity(gltf_mesh.primitives().len());

        for primitive in gltf_mesh.primitives() {
            let positions = primitive.get(&Semantic::Positions).unwrap();
            assert!(positions.data_type() == gltf::accessor::DataType::F32);

            let normals = primitive.get(&Semantic::Normals).unwrap();
            assert!(normals.data_type() == gltf::accessor::DataType::F32);

            let tex_coords_0 = primitive.get(&Semantic::TexCoords(0)).unwrap();
            assert!(tex_coords_0.data_type() == gltf::accessor::DataType::F32);

            let indices = primitive.indices().unwrap();
            assert!(
                indices.data_type() == gltf::accessor::DataType::U16
                    || indices.data_type() == gltf::accessor::DataType::U32
            );

            // println!(
            //     "size: {:?}, offset: {:?}, dimensions: {:?}",
            //     positions.size(),
            //     positions.offset(),
            //     positions.dimensions()
            // );

            let positions = get_data::<cgmath::Vector3<f32>>(positions, &buffers);
            let normals = get_data::<cgmath::Vector3<f32>>(normals, &buffers);
            let tex_coords_0 = get_data::<cgmath::Vector2<f32>>(tex_coords_0, &buffers);

            let indices: Vec<u32> = if indices.data_type() == gltf::accessor::DataType::U16 {
                let indices = get_data::<u16>(indices, &buffers);
                indices.into_iter().map(|x| x as u32).collect()
            } else {
                get_data::<u32>(indices, &buffers)
            };

            primitives.push(Primitive::new(positions, normals, tex_coords_0, indices));
            material_index.push(primitive.material().index().unwrap_or_default());
        }

        storage.meshes.push(Mesh(primitives));
        material_indexes.push(material_index);
    }

    for gltf_material in document.materials() {
        let mut material = Material::default();
        let pbr = gltf_material.pbr_metallic_roughness();

        if let Some(albedo) = pbr.base_color_texture() {
            let texture = get_texture(albedo.texture(), &images);

            material.albedo = Some(storage.textures2d.len());
            storage.textures2d.push(texture);
        }

        // TODO

        storage.materials.push(material);
    }

    let default_scene = document.default_scene().unwrap();
    let mut node_data = HashMap::with_capacity(document.nodes().len());
    for gltf_node in default_scene.nodes() {
        let transform = cgmath::Matrix4::from(gltf_node.transform().matrix());
        if let Some(mesh) = gltf_node.mesh() {
            let node = Node {
                mesh: mesh.index(),
                materials: material_indexes[mesh.index()].clone(),
                transform,
            };

            node_data.insert(gltf_node.index(), node);
        }

        for child in gltf_node.children() {
            parse_nodes_recursive(child, transform, &mut node_data, &material_indexes);
        }
    }

    return (storage, node_data.into_values().collect());
    // return (
    //     meshes,
    //     materials,
    //     node_data.into_iter().map(|(_, value)| value).collect(),
    // );

    fn parse_nodes_recursive(
        gltf_node: gltf::Node,
        parent_transform: cgmath::Matrix4<f32>,
        data: &mut HashMap<usize, Node>,
        material_indexes: &[Vec<usize>],
    ) {
        // let transform: cgmath::Matrix4<f32> =
        //     cgmath::Matrix4::from(gltf_node.transform().matrix()) * parent_transform;
        let transform: cgmath::Matrix4<f32> =
            parent_transform * cgmath::Matrix4::from(gltf_node.transform().matrix());

        if let Some(mesh) = gltf_node.mesh() {
            let node = Node {
                mesh: mesh.index(),
                materials: material_indexes[mesh.index()].clone(),
                transform,
            };

            data.insert(gltf_node.index(), node);
        }

        for child_node in gltf_node.children() {
            parse_nodes_recursive(child_node, transform, data, material_indexes);
        }
    }
}

fn get_data<T: bytemuck::Pod>(accessor: Accessor, buffers: &[Data]) -> Vec<T> {
    // let component_size = accessor.size();
    // let offset = accessor.offset();
    let view = accessor.view().unwrap();

    let buffer = &buffers[view.buffer().index()][view.offset()..view.offset() + view.length()];
    let buffer = &buffer[accessor.offset()..];

    assert!(accessor.size() == view.stride().unwrap_or(accessor.size()));

    bytemuck::cast_slice(buffer).to_vec()
}

fn get_texture<'a>(texture: gltf::Texture<'a>, images: &[gltf::image::Data]) -> Texture2D {
    let sampler = texture.sampler();
    let image = &images[texture.source().index()];
    let (format, gl_type) = match image.format {
        gltf::image::Format::R8 => (gl::R8, gl::UNSIGNED_BYTE),
        gltf::image::Format::R8G8 => (gl::RG, gl::UNSIGNED_BYTE),
        gltf::image::Format::R8G8B8 => (gl::RGB, gl::UNSIGNED_BYTE),
        gltf::image::Format::R8G8B8A8 => (gl::RGBA, gl::UNSIGNED_BYTE),
        gltf::image::Format::B8G8R8 => (gl::BGR, gl::UNSIGNED_BYTE),
        gltf::image::Format::B8G8R8A8 => (gl::BGRA, gl::UNSIGNED_BYTE),
        gltf::image::Format::R16 => (gl::R16, gl::UNSIGNED_SHORT),
        gltf::image::Format::R16G16 => (gl::RG16, gl::UNSIGNED_SHORT),
        gltf::image::Format::R16G16B16 => (gl::RGB16, gl::UNSIGNED_SHORT),
        gltf::image::Format::R16G16B16A16 => (gl::RGBA16, gl::UNSIGNED_SHORT),
    };

    Texture2D::new(
        sampler.wrap_s().as_gl_enum(),
        sampler.wrap_t().as_gl_enum(),
        sampler
            .mag_filter()
            .unwrap_or(gltf::texture::MagFilter::Linear)
            .as_gl_enum(),
        sampler
            .min_filter()
            .unwrap_or(gltf::texture::MinFilter::LinearMipmapLinear)
            .as_gl_enum(),
        &image.pixels,
        format,
        gl_type,
        (image.width, image.height),
    )
}

// pub fn read<P: AsRef<Path>>(path: P) -> Vec<Mesh> {
//     let (document, buffers, images) = gltf::import(path).unwrap();
//     assert_eq!(buffers.len(), document.buffers().count());
//     assert_eq!(images.len(), document.images().count());
//
//     let mut meshes = Vec::new();
//
//     for gltf_mesh in document.meshes() {
//         println!("gltf_mesh");
//         for primitive in gltf_mesh.primitives() {
//             let positions = primitive.get(&Semantic::Positions).unwrap();
//             assert!(positions.data_type() == gltf::accessor::DataType::F32);
//
//             let normals = primitive.get(&Semantic::Normals).unwrap();
//             assert!(normals.data_type() == gltf::accessor::DataType::F32);
//
//             let tex_coords_0 = primitive.get(&Semantic::TexCoords(0)).unwrap();
//             assert!(tex_coords_0.data_type() == gltf::accessor::DataType::F32);
//
//             let indices = primitive.indices().unwrap();
//             assert!(
//                 indices.data_type() == gltf::accessor::DataType::U16
//                     || indices.data_type() == gltf::accessor::DataType::U32
//             );
//
//             println!(
//                 "size: {:?}, offset: {:?}, dimensions: {:?}",
//                 positions.size(),
//                 positions.offset(),
//                 positions.dimensions()
//             );
//
//             let positions = get_data::<cgmath::Vector3<f32>>(positions, &buffers);
//             let normals = get_data::<cgmath::Vector3<f32>>(normals, &buffers);
//             let tex_coords_0 = get_data::<cgmath::Vector2<f32>>(tex_coords_0, &buffers);
//
//             let indices: Vec<u32> = if indices.data_type() == gltf::accessor::DataType::U16 {
//                 let indices = get_data::<u16>(indices, &buffers);
//                 indices.into_iter().map(|x| x as u32).collect()
//             } else {
//                 get_data::<u32>(indices, &buffers)
//             };
//             // let indices = get_buffer::<u32>(indices.view().unwrap(), &buffers);
//
//             // println!("{positions:?}");
//             // println!("{normals:?}");
//             // println!("{tex_coords_0:?}");
//             // println!("{indices:?}");
//
//             meshes.push(Mesh(vec![crate::mesh::Primitive::new(
//                 positions,
//                 normals,
//                 tex_coords_0,
//                 indices,
//             )]));
//         }
//     }
//
//     meshes
// }
