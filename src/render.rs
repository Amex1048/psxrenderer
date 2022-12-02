use crate::camera::Camera;
use crate::model::{Instance, Model};

pub struct RenderContext {
    instances: Vec<Instance>,
    camera: Camera,
}

impl Default for RenderContext {
    fn default() -> Self {
        let instances = vec![Instance {
            model: Model::default(),
            transform: cgmath::Matrix4::from_translation(cgmath::vec3(0.0, 0.0, 0.0)),
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

        println!("{:?}", input.mouse_pos);
        println!("{:?}", input.mouse_rel);
        println!("{:?}", mouse);

        self.camera
            .update(front, right, back, left, up, down, mouse);
    }
}
