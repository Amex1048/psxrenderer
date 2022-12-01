// Vectors

pub trait UniformVec<T, const N: usize>: AsRef<[T; N]> {
    // const COUNT: i32;
    const LOADER: unsafe fn(i32, i32, *const T);
}

impl UniformVec<f32, 1> for cgmath::Vector1<f32> {
    // const COUNT: i32 = 1;
    const LOADER: unsafe fn(i32, i32, *const f32) = gl::Uniform1fv;
}

impl UniformVec<f32, 2> for cgmath::Vector2<f32> {
    // const COUNT: i32 = 2;
    const LOADER: unsafe fn(i32, i32, *const f32) = gl::Uniform2fv;
}

impl UniformVec<f32, 3> for cgmath::Vector3<f32> {
    // const COUNT: i32 = 3;
    const LOADER: unsafe fn(i32, i32, *const f32) = gl::Uniform3fv;
}

impl UniformVec<f32, 4> for cgmath::Vector4<f32> {
    // const COUNT: i32 = 4;
    const LOADER: unsafe fn(i32, i32, *const f32) = gl::Uniform4fv;
}

// Matrices

pub trait UniformMat<T, const N: usize>: AsRef<[T; N]> {
    const LOADER: unsafe fn(i32, i32, u8, *const T);
}

impl UniformMat<f32, 4> for cgmath::Matrix2<f32> {
    const LOADER: unsafe fn(i32, i32, u8, *const f32) = gl::UniformMatrix2fv;
}

impl UniformMat<f32, 9> for cgmath::Matrix3<f32> {
    const LOADER: unsafe fn(i32, i32, u8, *const f32) = gl::UniformMatrix3fv;
}

impl UniformMat<f32, 16> for cgmath::Matrix4<f32> {
    const LOADER: unsafe fn(i32, i32, u8, *const f32) = gl::UniformMatrix4fv;
}
