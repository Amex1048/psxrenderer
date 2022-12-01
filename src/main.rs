use renderer::gltf;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

const GL_MAJOR_VERSION: u8 = 4;
const GL_MINOR_VERSION: u8 = 5;

fn print_debug_messages() {
    unsafe {
        loop {
            let mut buf: Vec<u8> = Vec::with_capacity(1024);
            let mut sources = 0;
            let mut types = 0;
            let mut ids = 0;
            let mut severities = 0;
            let mut lengths = 0;
            let fetched = gl::GetDebugMessageLog(
                1,
                1024,
                &mut sources,
                &mut types,
                &mut ids,
                &mut severities,
                &mut lengths,
                buf.as_mut_ptr().cast(),
            );

            if fetched == 0 {
                break;
            }

            buf.set_len(lengths.try_into().unwrap());
            let string = String::from_utf8_lossy(&buf);

            println!("{string}");
        }
    }
}

fn main() {
    // gltf::read("models/Duck/Duck.gltf");
    //
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    // gl_attr.set_context_flags().debug().set();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(GL_MAJOR_VERSION, GL_MINOR_VERSION);

    let window = video_subsystem
        .window("Window", 800, 600)
        .opengl()
        // .resizable()
        .build()
        .unwrap();

    // Unlike the other example above, nobody created a context for your window, so you need to create one.
    let ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(
        gl_attr.context_version(),
        (GL_MAJOR_VERSION, GL_MINOR_VERSION)
    );

    let mut render = renderer::render::RenderContext::default();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        render.render();

        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
