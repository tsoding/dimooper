use pm::InputPort;
use sdl2::TimerSubsystem;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::render::Renderer;
use screen::Screen;
use midi::AbsMidiEvent;
use midi;
use hardcode::*;
use std;

pub struct EventLoop<'a> {
    timer_subsystem: TimerSubsystem,
    sdl_event_pump: EventPump,
    midi_input_port: InputPort,
    renderer: Renderer<'a>,
}

impl<'a> EventLoop<'a> {
    pub fn new(timer_subsystem: TimerSubsystem,
           sdl_event_pump: EventPump,
           midi_input_port: InputPort,
           renderer: Renderer<'a>) -> EventLoop<'a> {
        EventLoop {
            timer_subsystem: timer_subsystem,
            sdl_event_pump: sdl_event_pump,
            midi_input_port: midi_input_port,
            renderer: renderer
        }
    }

    pub fn run<T, S: Screen<T>>(&mut self, mut screen: S) -> T {
        let mut previuos_ticks = self.timer_subsystem.ticks();

        loop {
            let current_ticks = self.timer_subsystem.ticks();
            let delta_time = current_ticks - previuos_ticks;
            previuos_ticks = current_ticks;

            let sdl_events: Vec<Event> =
                self.sdl_event_pump.poll_iter().collect();
            screen.handle_sdl_events(&sdl_events);


            if let Ok(Some(raw_midi_events)) = self.midi_input_port.read_n(1024) {
                let midi_events: Vec<AbsMidiEvent> = raw_midi_events
                    .iter()
                    .filter_map(|e| midi::parse_midi_event(e))
                    .collect();
                screen.handle_midi_events(&midi_events);
            }

            if let Some(result) = screen.update(delta_time) {
                return result;
            }

            screen.render(&mut self.renderer);
            self.renderer.present();

            std::thread::sleep(std::time::Duration::from_millis(EVENT_LOOP_SLEEP_TIMEOUT));
        }
    }
}
