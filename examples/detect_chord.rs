use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use chord_detector_sys::{AudioFrame, Detector};

pub fn a() {}

fn main() {
    let sound = std::fs::File::open("assets/sound.mp3").unwrap();
    let buffer: Vec<f64> =
        get_samples(sound).into_iter().map(|sample| sample as f64).collect();
    let mut detector = Detector::new(AudioFrame::new(8192), 44100);

    let mut results = Vec::new();
    for chunk in buffer.chunks(8192) {
        if let Err(_) = detector.frame_mut().update_buffer(chunk) {
            continue;
        }
        if let Some(info) = detector.process_audio_frame() {
            results.push(info);
        }
    }
    dbg!(results);
}

fn get_samples(src: std::fs::File) -> Vec<f32> {
    // Create the media source stream.
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // Create a probe hint using the file's extension. [Optional]
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    // Get the instantiated format reader.
    let mut format = probed.format;

    // Find the first audio track with a known (decodeable) codec.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    // Use the default options for the decoder.
    let dec_opts: DecoderOptions = Default::default();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");

    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;

    // This will hold all of our decoded samples.
    let mut all_samples: Vec<f32> = Vec::new();

    // The decode loop.
    loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                // The track list has been changed. Re-examine it and create a new set of decoders,
                // then restart the decode loop. This is an advanced feature and it is not
                // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                // for chained OGG physical streams.
                unimplemented!();
            }
            Err(Error::IoError(err)) => {
                if err.to_string() == "end of stream" {
                    println!("end of stream");
                    break;
                } else {
                    panic!("{}", err);
                }
            }
            Err(err) => {
                // A unrecoverable error occured, halt decoding.
                panic!("{}", err);
            }
        };

        // Consume any new metadata that has been read since the last packet.
        while !format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            format.metadata().pop();

            // Consume the new metadata at the head of the metadata queue.
        }

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples.
        match decoder.decode(&packet) {
            Ok(decoded) => {
                // Consume the decoded audio samples (see below).
                match decoded {
                    AudioBufferRef::F32(ref buf) => {
                        if buf.spec().channels.count() == 2 {
                            // We assume the file is stereo;
                            // chan(0) will get the left channel
                            // chan(1) will get the right channel.
                            let left_channel = buf.chan(0);
                            let right_channel = buf.chan(1);

                            // Interleave the samples: left, right, left, right, etc.
                            for (&left_sample, &right_sample) in
                                left_channel.iter().zip(right_channel)
                            {
                                all_samples.push(left_sample);
                                all_samples.push(right_sample);
                            }
                        } else {
                            // If the file is not stereo, extend as before.
                            all_samples.extend_from_slice(buf.chan(0));
                        }
                    }
                    _ => {}
                }
            }
            Err(Error::IoError(err)) => {
                // The packet failed to decode due to an IO error, panic.
                panic!("{}", err);
            }
            Err(Error::DecodeError(err)) => {
                // The packet failed to decode due to invalid data, panic.
                panic!("{}", err);
            }
            Err(err) => {
                // An unrecoverable error occured, halt decoding.
                panic!("{}", err);
            }
        }
    }

    println!("Success!");
    all_samples
}
