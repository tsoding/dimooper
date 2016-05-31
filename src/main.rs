extern crate portmidi as pm;

use std::time::Duration;
use std::thread;
use pm::types::MidiEvent;
use pm::OutputPort;

fn replay_buffer_forever(record_buffer: &Vec<MidiEvent>, out_port: &mut OutputPort) {
    loop {
        let mut some_previous_event: Option<MidiEvent> = None;
        for event in record_buffer {
            if let Some(previous_event) = some_previous_event {
                thread::sleep(Duration::from_millis((event.timestamp - previous_event.timestamp) as u64));
            }

            out_port.write_message(event.message).unwrap();
            some_previous_event = Some(event.clone())
        }
    }
}

fn main() {
    let context = pm::PortMidi::new().unwrap();
    let timeout = Duration::from_millis(10);
    let mut record_buffer: Vec<MidiEvent> = vec!();

    let in_info = context.device(1).unwrap();
    println!("Listening on: {} {}", in_info.id(), in_info.name());

    let in_port = context.input_port(in_info, 1024).unwrap();
    let mut recording = true;

    while let Ok(_) = in_port.poll() {
        if !recording {
            break;
        }

        if let Ok(Some(current_events)) = in_port.read_n(1024) {
            for event in current_events {
                let channel = event.message.status & 15;
                println!("Channel: {}", channel);
                if channel == 9 {
                    recording = false;
                } else {
                    record_buffer.push(event.clone());
                }
                println!("{:?}", event);
            }
        }

        thread::sleep(timeout);
    }

    let out_info = context.device(0).unwrap();
    println!("Sending recorded events: {} {}", out_info.id(), out_info.name());

    let mut out_port = context.output_port(out_info, 1024).unwrap();

    replay_buffer_forever(&record_buffer, &mut out_port);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        assert!(true);
    }
}
