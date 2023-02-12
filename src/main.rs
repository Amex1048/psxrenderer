use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Mod, Scancode};
use sdl2::video::GLProfile;

const SCENE_PATH: &str = "scenes/Phasmophobia/Phasmophobia.gltf";

const GL_MAJOR_VERSION: u8 = 3;
const GL_MINOR_VERSION: u8 = 3;

// TODO: Configure VSYNC instead
const FRAMERATE: u32 = 60;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(GL_MAJOR_VERSION, GL_MINOR_VERSION);

    let mut window = video_subsystem
        .window("Window", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    window.set_grab(true);
    sdl_context.mouse().set_relative_mouse_mode(true);

    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(
        gl_attr.context_version(),
        (GL_MAJOR_VERSION, GL_MINOR_VERSION)
    );

    let mut render = renderer::render::World::from_gltf_file(SCENE_PATH);
    let mut input_state = renderer::InputState::default();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut timer = std::time::Instant::now();

    'running: loop {
        input_state.mouse_rel = (0, 0);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    scancode: Some(Scancode::W),
                    keymod,
                    ..
                } => {
                    input_state.w = true;
                    input_state.shift = keymod == Mod::LSHIFTMOD;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::A),
                    keymod,
                    ..
                } => {
                    input_state.a = true;
                    input_state.shift = keymod == Mod::LSHIFTMOD;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::S),
                    keymod,
                    ..
                } => {
                    input_state.s = true;
                    input_state.shift = keymod == Mod::LSHIFTMOD;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::D),
                    keymod,
                    ..
                } => {
                    input_state.d = true;
                    input_state.shift = keymod == Mod::LSHIFTMOD;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::E),
                    keymod,
                    ..
                } => {
                    input_state.e = true;
                    input_state.shift = keymod == Mod::LSHIFTMOD;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Q),
                    keymod,
                    ..
                } => {
                    input_state.q = true;
                    input_state.shift = keymod == Mod::LSHIFTMOD;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::W),
                    ..
                } => input_state.w = false,
                Event::KeyUp {
                    scancode: Some(Scancode::A),
                    ..
                } => input_state.a = false,
                Event::KeyUp {
                    scancode: Some(Scancode::S),
                    ..
                } => input_state.s = false,
                Event::KeyUp {
                    scancode: Some(Scancode::D),
                    ..
                } => input_state.d = false,
                Event::KeyUp {
                    scancode: Some(Scancode::E),
                    ..
                } => input_state.e = false,
                Event::KeyUp {
                    scancode: Some(Scancode::Q),
                    ..
                } => input_state.q = false,
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    input_state.mouse_pos = (x, y);
                    input_state.mouse_rel = (xrel, yrel);
                }
                // ---------------------------//
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => {
                        render.update_viewport(0, 0, width, height)
                    }
                    WindowEvent::SizeChanged(width, height) => {
                        render.update_viewport(0, 0, width, height)
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let delta = timer.elapsed().as_secs_f32();
        timer = std::time::Instant::now();

        render.update(&input_state, delta);

        render.render();
        window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / FRAMERATE));
    }
}
