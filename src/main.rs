extern crate portmidi as pm;

use std::time::Duration;
use std::thread;
use pm::types::MidiEvent;
use pm::OutputPort;

fn replay_buffer(record_buffer: Vec<MidiEvent>, out_port: &mut OutputPort) {
    for event in record_buffer {
        out_port.write_message(event.message).unwrap();
        thread::sleep(Duration::from_millis(400));
    }
}

fn main() {
    let context = pm::PortMidi::new().unwrap();
    let timeout = Duration::from_millis(10);
    let mut record_buffer: Vec<MidiEvent> = vec!();
    let record_buffer_limit = 10;

    let in_info = context.device(1).unwrap();
    println!("Listening on: {} {}", in_info.id(), in_info.name());

    let in_port = context.input_port(in_info, 1024).unwrap();

    while let Ok(_) = in_port.poll() {
        if record_buffer.len() >= record_buffer_limit {
            break;
        }

        if let Ok(Some(current_events)) = in_port.read_n(1024) {
            for event in current_events {
                record_buffer.push(event.clone());
                println!("{:?}", event);
            }
        }

        thread::sleep(timeout);
    }

    let out_info = context.device(0).unwrap();
    println!("Sending recorded events: {} {}", out_info.id(), out_info.name());

    let mut out_port = context.output_port(out_info, 1024).unwrap();

    replay_buffer(record_buffer, &mut out_port);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        assert!(true);
    }
}
