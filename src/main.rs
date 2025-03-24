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
        } if window_id == state.window().id() => match event {
            WindowEvent::CloseRequested => control_flow.exit(),
            _ => {}
        },
        _ => {}
    }).unwrap();
}
