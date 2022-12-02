use std::collections::HashMap;
use std::path::Path;

use gltf::buffer::Data;
use gltf::Accessor;
use gltf::Semantic;

use crate::material::Material;
use crate::mesh::{Mesh, Primitive};
use crate::render::Node;

pub fn read_from_file<P: AsRef<Path>>(path: P) -> (Vec<Mesh>, Vec<Material>, Vec<Node>) {
    let (document, buffers, images) = gltf::import(path).unwrap();
    assert_eq!(buffers.len(), document.buffers().count());
    assert_eq!(images.len(), document.images().count());

    let mut meshes: Vec<Mesh> = Vec::with_capacity(document.meshes().len());
    let mut material_indexes: Vec<Vec<usize>> = Vec::with_capacity(document.meshes().len());
    let mut materials: Vec<Material> = Vec::with_capacity(document.materials().len());
    // let mut nodes: Vec<Node> = Vec::with_capacity(document.nodes().len());

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
            material_index.push(primitive.material().index().unwrap());
        }

        meshes.push(Mesh(primitives));
        material_indexes.push(material_index);
    }

    for gltf_material in document.materials() {
        // TODO
        materials.push(Material::default());
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

    return (
        meshes,
        materials,
        node_data.into_iter().map(|(_, value)| value).collect(),
    );

    fn parse_nodes_recursive(
        gltf_node: gltf::Node,
        parent_transform: cgmath::Matrix4<f32>,
        data: &mut HashMap<usize, Node>,
        material_indexes: &[Vec<usize>],
    ) {
        let transform: cgmath::Matrix4<f32> =
            cgmath::Matrix4::from(gltf_node.transform().matrix()) * parent_transform;
        // let transform: cgmath::Matrix4<f32> =
        //     parent_transform * cgmath::Matrix4::from(gltf_node.transform().matrix());

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

// fn get_buffer<T: bytemuck::Pod>(view: View, buffers: &[Data]) -> Vec<T> {
//     println!(
//         "index: {}, offset: {}, len: {}",
//         view.index(),
//         view.offset(),
//         view.length()
//     );
//     let buffer = &buffers[view.buffer().index()][view.offset()..view.offset() + view.length()];
//     bytemuck::cast_slice(buffer).to_vec()
// }

fn get_data<T: bytemuck::Pod>(accessor: Accessor, buffers: &[Data]) -> Vec<T> {
    // let component_size = accessor.size();
    // let offset = accessor.offset();
    let view = accessor.view().unwrap();

    let buffer = &buffers[view.buffer().index()][view.offset()..view.offset() + view.length()];
    let buffer = &buffer[accessor.offset()..];

    assert!(accessor.size() == view.stride().unwrap_or(accessor.size()));

    bytemuck::cast_slice(buffer).to_vec()
}

pub fn read<P: AsRef<Path>>(path: P) -> Vec<Mesh> {
    let (document, buffers, images) = gltf::import(path).unwrap();
    assert_eq!(buffers.len(), document.buffers().count());
    assert_eq!(images.len(), document.images().count());

    let mut meshes = Vec::new();

    for gltf_mesh in document.meshes() {
        println!("gltf_mesh");
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

            println!(
                "size: {:?}, offset: {:?}, dimensions: {:?}",
                positions.size(),
                positions.offset(),
                positions.dimensions()
            );

            let positions = get_data::<cgmath::Vector3<f32>>(positions, &buffers);
            let normals = get_data::<cgmath::Vector3<f32>>(normals, &buffers);
            let tex_coords_0 = get_data::<cgmath::Vector2<f32>>(tex_coords_0, &buffers);

            let indices: Vec<u32> = if indices.data_type() == gltf::accessor::DataType::U16 {
                let indices = get_data::<u16>(indices, &buffers);
                indices.into_iter().map(|x| x as u32).collect()
            } else {
                get_data::<u32>(indices, &buffers)
            };
            // let indices = get_buffer::<u32>(indices.view().unwrap(), &buffers);

            // println!("{positions:?}");
            // println!("{normals:?}");
            // println!("{tex_coords_0:?}");
            // println!("{indices:?}");

            meshes.push(Mesh(vec![crate::mesh::Primitive::new(
                positions,
                normals,
                tex_coords_0,
                indices,
            )]));
        }
    }

    meshes
}
