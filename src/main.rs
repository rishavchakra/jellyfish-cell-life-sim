#[macro_use]
extern crate lazy_static;
use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};
mod gpu;
mod render_plane;
mod agents;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut state =
        pollster::block_on(gpu::State::new(&window)).expect("GPU Initialization failed");

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input_is_handled(event) {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("The close button was pressed; stopping");
                            elwt.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            println!("Resizing window");
                            state.resize(*physical_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                state.update();
                let render_res = state.render();
                match render_res {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => {
                        eprintln!("ERROR: Swap chain lost, recreating");
                        state.resize(state.window_size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        eprintln!("ERROR: Ran out of memory!");
                        elwt.exit();
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
            }
            _ => (),
        })
        .unwrap();
}
