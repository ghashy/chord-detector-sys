#![allow(non_camel_case_types)]

use std::marker::PhantomData;
use std::os::raw::{c_double, c_int};

#[link(name = "detector-lib-bridge")]
extern "C" {

    // ───── Chromagram ───────────────────────────────────────────────────── //

    /// Constructor
    pub fn init_chromagram(
        frame_size: c_int,
        sample_rate: c_int,
    ) -> *const Chromagram;
    /// Destructor
    pub fn free_chromagram(o: *const Chromagram);
    pub fn process_audio_frame(o: *const Chromagram, audio: *const c_double);
    pub fn is_ready(o: *const Chromagram) -> c_int;
    /// Returns owned chroma vector
    pub fn get_chromagram(o: *const Chromagram) -> owned_cpp_vector_double;
    pub fn set_input_audio_framesize(o: *const Chromagram, framesize: c_int);
    pub fn set_sampling_frequency(o: *const Chromagram, frequency: c_int);
    pub fn set_chroma_calculation_interval(
        o: *const Chromagram,
        interval: c_int,
    );

    // ───── ChordDetector ────────────────────────────────────────────────── //

    pub fn init_chord_detector() -> *const ChordDetector;
    pub fn free_chord_detector(o: *const ChordDetector);
    /// Takes reference to chroma, and makes deep copy of it
    pub fn detect_chord(
        o: *const ChordDetector,
        chroma: borrowed_cpp_vector_double,
    );
    pub fn fetch(o: *const ChordDetector) -> chord_info;
}

pub enum Chromagram {}
pub enum ChordDetector {}

#[repr(C)]
#[derive(Debug)]
pub struct borrowed_cpp_vector_double<'a> {
    pub ptr: *const c_double,
    pub len: usize,
    phantom: PhantomData<&'a ()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct owned_cpp_vector_double {
    pub ptr: *const c_double,
    pub len: usize,
}

impl owned_cpp_vector_double {
    pub fn borrow(&self) -> borrowed_cpp_vector_double {
        borrowed_cpp_vector_double {
            ptr: self.ptr,
            len: self.len,
            phantom: Default::default(),
        }
    }
}

impl Drop for owned_cpp_vector_double {
    fn drop(&mut self) {
        // Check if the pointer is not null before deallocating
        if !self.ptr.is_null() {
            // Deallocate the memory pointed to by the raw pointer
            unsafe {
                // Convert the raw pointer to a mutable pointer for deallocation
                let ptr = self.ptr as *mut c_double;
                // Deallocate the memory
                libc::free(ptr as *mut std::ffi::c_void);
                // Set the pointer to null to prevent double deallocation
                self.ptr = std::ptr::null();
                self.len = 0;
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct chord_info {
    pub root_note: c_int,
    pub quality: super::ChordQuality,
    pub intervals: c_int,
}
