use rodio::{source::Source, Decoder, OutputStream};
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;

const spec: hound::WavSpec = hound::WavSpec {
    channels: 1,
    sample_rate: 48000,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
};
fn write_sin() {
    // let spec = hound::WavSpec {
    //     channels: 1,
    //     sample_rate: 48000,
    //     bits_per_sample: 16,
    //     sample_format: hound::SampleFormat::Int,
    // };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0..48000).map(|x| x as f32 / 48000.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = (i16::MAX as f32) / 4.0;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}

fn write_clipped() {
    let mut writer = hound::WavWriter::create("clipped.wav", spec).unwrap();
    for t in (0..48000).map(|x| x as f32 / 48000.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = (i16::MAX as f32) / 2.0;
        writer
            .write_sample(
                (sample * amplitude).clamp((i16::MIN as f32) / 4.0, (i16::MAX as f32) / 4.0) as i16,
            )
            .unwrap();
    }
    writer.finalize().unwrap();
}

fn play_clipped() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("clipped.wav").unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle
        .play_raw(source.convert_samples())
        .expect("Error during play_raw");

    // Keep main thread alive while seperate audio thread plays.
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn main() {
    write_sin();
    write_clipped();
    play_clipped();
}
