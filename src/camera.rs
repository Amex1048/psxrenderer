use cgmath::prelude::*;
use cgmath::Vector3;

pub struct Camera {
    position: Vector3<f32>,
    front: Vector3<f32>,
    // right: Vector3<f32>,
    up: Vector3<f32>,
    yaw: cgmath::Rad<f32>,
    pitch: cgmath::Rad<f32>,

    fovy: cgmath::Deg<f32>,
    aspect: f32,
}

const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 100.0;

const MOVEMENT_SPEED: f32 = 2.0;
const MOUSE_SENSIVITY: f32 = 100.0;
const PITCH_BOUND: f32 = 0.85 * (0.5 * std::f32::consts::PI);

impl Camera {
    pub fn new(
        position: Vector3<f32>,
        target: Vector3<f32>,
        fovy: cgmath::Deg<f32>,
        aspect: f32,
    ) -> Self {
        let front = (position - target).normalize();
        let right = cgmath::vec3(0.0, 1.0, 0.0).cross(front).normalize();
        let up = front.cross(right);

        let front = -front;

        let pitch = front.y.asin();
        let yaw = (front.x / pitch.cos()).acos();
        // let yaw = front.x.acos() / pitch.cos();

        // println!("position: {position:?}");
        // println!("front: {front:?}");
        // println!("right: {right:?}");
        // println!("up: {up:?}");
        // println!("yaw: {yaw:?}");
        // println!("pitch: {pitch:?}");

        Self {
            position,
            front,
            // right,
            up,
            yaw: cgmath::Rad(yaw),
            pitch: cgmath::Rad(pitch),
            fovy,
            aspect,
        }
    }

    pub fn view(&self) -> cgmath::Matrix4<f32> {
        let direction = self.position + self.front;
        cgmath::Matrix4::look_at_rh(
            cgmath::point3(self.position.x, self.position.y, self.position.z),
            cgmath::point3(direction.x, direction.y, direction.z),
            self.up,
        )
    }

    pub fn projection(&self) -> cgmath::Matrix4<f32> {
        cgmath::perspective(self.fovy, self.aspect, Z_NEAR, Z_FAR)
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn update(
        &mut self,
        front: f32,
        right: f32,
        back: f32,
        left: f32,
        up: f32,
        down: f32,
        mouse: (f32, f32),
    ) {
        self.position += MOVEMENT_SPEED * (front - back) * self.front;
        self.position += MOVEMENT_SPEED * (right - left) * self.front.cross(self.up).normalize();
        self.position += MOVEMENT_SPEED * (up - down) * self.up;

        self.yaw += cgmath::Rad(mouse.0 * MOUSE_SENSIVITY);
        self.pitch += cgmath::Rad(mouse.1 * MOUSE_SENSIVITY);

        if self.yaw.0 > 2.0 * std::f32::consts::PI {
            self.yaw.0 -= 2.0 * std::f32::consts::PI;
        }
        if self.yaw.0 < -2.0 * std::f32::consts::PI {
            self.yaw.0 += 2.0 * std::f32::consts::PI;
        }

        self.pitch = cgmath::Rad(self.pitch.0.max(-PITCH_BOUND));
        self.pitch = cgmath::Rad(self.pitch.0.min(PITCH_BOUND));

        self.front = -cgmath::vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
        //
        // println!("front: {:?}", self.front);
        // println!("yaw: {:?}", self.yaw);
        // println!("pitch: {:?}", self.pitch);
    }
}
