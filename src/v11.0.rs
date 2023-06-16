use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeOutputBuffer};

const THRESHOLD_DB: f32 = -20.0; // Set your desired threshold in decibels

fn main() {
    // Initialize the audio host
    let host = cpal::default_host();
    let event_loop = host.event_loop();
    let device = host
        .default_output_device()
        .expect("Failed to get default output device");

    // Get the supported audio format from the output device
    let format = device
        .default_output_format()
        .expect("Failed to get default output format");

    // Create the audio stream with the specified format and callback
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id).unwrap();

    // Run the event loop and process the audio stream
    event_loop.run(move |_stream_id, stream_result| {
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(_) => return,
        };

        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                // Process the audio samples in the buffer
                limit_audio_output(&mut buffer);
            }
            StreamData::Input { buffer } => {
                // Ignore the input stream
                println!("Received input stream: {:?}", buffer.len());
            }
            _ => (),
        }
    });
}

fn limit_audio_output(buffer: &mut [f32]) {
    // Iterate over each sample in the buffer
    for sample in buffer.iter_mut() {
        // Convert the sample to decibels (assuming linear scale input)
        let sample_db = 20.0 * sample.abs().log10();

        if sample_db != f32::NEG_INFINITY {
            println!("Sample dB: {}", sample_db)
        } else {
            continue;
        }

        // Check if the sample exceeds the threshold
        if sample_db > THRESHOLD_DB {
            // Apply the limiting by scaling down the sample
            *sample *= 10.0f32.powf((THRESHOLD_DB - sample_db) / 20.0);
        }
    }
}
