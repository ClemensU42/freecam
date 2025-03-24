use winit::{event::{Event, WindowEvent}, event_loop::EventLoop, window::WindowBuilder};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, control_flow| match event{
        Event::WindowEvent { 
            window_id, 
            ref event 
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => control_flow.exit(),
            _ => {}
        },
        _ => {}
    }).unwrap();
}
