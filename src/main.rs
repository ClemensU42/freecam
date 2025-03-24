use window::state::State;
use winit::{event::{Event, WindowEvent}, event_loop::EventLoop, window::WindowBuilder};

mod window;

fn main(){
    pollster::block_on(run());
}

async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    event_loop.run(move |event, control_flow| match event{
        Event::WindowEvent { 
            window_id, 
            ref event 
        } if window_id == state.window().id() => if !state.input(event){ match event {
                WindowEvent::CloseRequested => control_flow.exit(),
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                },
                WindowEvent::RedrawRequested => {
                    state.window().request_redraw();

                    // if !surface_configured{
                    //     return;
                    // }

                    state.update();
                    match state.render(){
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                            log::error!("OutOfMemory");
                            control_flow.exit();
                        }
                        Err(wgpu::SurfaceError::Timeout) => {
                            // Frame took too long to present
                            log::warn!("Surface timeout");
                        }
                    }
                }
                _ => {}
            }
        },
        _ => {}
    }).unwrap();
}
