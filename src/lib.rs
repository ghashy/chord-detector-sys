mod raw;

pub struct AudioFrame {
    size: i32,
    buffer: Option<Vec<f64>>,
}

impl AudioFrame {
    pub fn new(frame_size: i32) -> Self {
        AudioFrame {
            size: frame_size,
            buffer: None,
        }
    }

    pub fn update_buffer(&mut self, buf: &[f64]) -> Result<(), ()> {
        if buf.len() != self.size as usize {
            return Err(());
        } else {
            self.buffer = Some(buf.to_owned());
            Ok(())
        }
    }
}

pub struct Detector {
    raw_c: *const raw::Chromagram,
    raw_d: *const raw::ChordDetector,
    frame: AudioFrame,
}

impl Detector {
    pub fn new(frame: AudioFrame, sample_rate: i32) -> Self {
        Detector {
            raw_c: unsafe { raw::init_chromagram(frame.size, sample_rate) },
            raw_d: unsafe { raw::init_chord_detector() },
            frame,
        }
    }

    pub fn frame_mut(&mut self) -> &mut AudioFrame {
        &mut self.frame
    }

    pub fn process_audio_frame(&self) -> Option<ChordInfo> {
        let Some(ref buf) = self.frame.buffer else {
            panic!("Empty frame");
        };

        unsafe {
            raw::process_audio_frame(self.raw_c, buf.as_ptr());
            if self.is_ready() {
                let chroma = raw::get_chromagram(self.raw_c);
                raw::detect_chord(self.raw_d, chroma.borrow());
                let info = raw::fetch(self.raw_d);

                Some(ChordInfo {
                    root_note: info.root_note,
                    quality: info.quality,
                    intervals: info.intervals,
                })
            } else {
                None
            }
        }
    }

    /// This function resets current audio frame
    pub fn set_input_audio_framesize(&mut self, size: i32) {
        self.frame.size = size;
        if let Some(ref mut buf) = self.frame.buffer {
            *buf = Vec::with_capacity(size as usize);
        }
        unsafe { raw::set_input_audio_framesize(self.raw_c, size) };
    }

    pub fn set_sampling_frequency(&self, freq: i32) {
        unsafe { raw::set_sampling_frequency(self.raw_c, freq) }
    }

    pub fn set_chroma_calculation_interval(&self, interval: i32) {
        unsafe { raw::set_chroma_calculation_interval(self.raw_c, interval) }
    }

    fn is_ready(&self) -> bool {
        unsafe { raw::is_ready(self.raw_c) != 0 }
    }
}

impl Drop for Detector {
    fn drop(&mut self) {
        unsafe {
            raw::free_chromagram(self.raw_c);
            raw::free_chord_detector(self.raw_d)
        }
    }
}

#[derive(Debug)]
pub struct ChordInfo {
    pub root_note: i32,
    pub quality: ChordQuality,
    pub intervals: i32,
}

#[repr(C)]
#[derive(Debug)]
pub enum ChordQuality {
    Minor,
    Major,
    Suspended,
    Dominant,
    Dimished5th,
    Augmented5th,
}
