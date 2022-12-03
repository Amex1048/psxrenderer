use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Mod, Scancode};
use sdl2::video::GLProfile;

const GL_MAJOR_VERSION: u8 = 4;
const GL_MINOR_VERSION: u8 = 5;

const FRAMERATE: u32 = 60;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    // gl_attr.set_context_flags().debug().set();
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
    // sdl_context.mouse().warp_mouse_in_window(&window, 400, 300);
    // sdl_context.mouse().show_cursor(false);

    // Unlike the other example above, nobody created a context for your window, so you need to create one.
    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    // let meshes = renderer::gltf::read("models/Cube/Cube.gltf");
    // assert!(meshes.len() == 1);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(
        gl_attr.context_version(),
        (GL_MAJOR_VERSION, GL_MINOR_VERSION)
    );

    // ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / FRAMERATE));
    // sdl_context.mouse().warp_mouse_in_window(&window, 400, 300);

    // let mut render = renderer::render::RenderContext::default();
    let mut render = renderer::render::World::from_gltf_file("models/Scene/Scene.gltf");
    let mut input_state = renderer::InputState::default();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut timer = std::time::Instant::now();

    'running: loop {
        input_state.mouse_rel = (0, 0);
        // input_state.shift = false;

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
                // Event::KeyDown {
                //     keymod: Mod::LSHIFTMOD,
                //     ..
                // } => input_state.shift = true,
                // ---------------------------//
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
                // Event::KeyUp {
                //     keymod: Mod::LSHIFTMOD,
                //     ..
                // } => input_state.shift = false,
                // ---------------------------//
                Event::MouseMotion {
                    // which,
                    // mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                    ..
                } => {
                    input_state.mouse_pos = (x, y);
                    input_state.mouse_rel = (xrel, yrel);
                }
                // ---------------------------//
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => {
                        // println!("Resized: {width}, {height}");
                        render.update_viewport(0, 0, width, height)
                    }
                    WindowEvent::SizeChanged(width, height) => {
                        // println!("Size changed: {width}, {height}");
                        render.update_viewport(0, 0, width, height)
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // println!("{input_state:?}");

        let delta = timer.elapsed().as_secs_f32();
        timer = std::time::Instant::now();

        // println!("{delta:?}");

        let to_render = std::time::Instant::now();

        render.update(&input_state, delta);

        render.render();
        window.gl_swap_window();

        let _render_delta = to_render.elapsed().as_nanos();

        // println!("Framerate: {}", 1_000_000_000 / render_delta);

        ::std::thread::sleep(::std::time::Duration::new(
            0,
            1_000_000_000u32 / FRAMERATE, /*- render_delta as u32*/
        ));
    }
}
