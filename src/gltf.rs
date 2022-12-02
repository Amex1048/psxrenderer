use std::path::Path;

use gltf::buffer::Data;
use gltf::buffer::View;
use gltf::Accessor;
use gltf::Semantic;

use crate::mesh::Mesh;

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

            meshes.push(Mesh::new(positions, normals, tex_coords_0, indices));
        }
    }

    meshes
}
