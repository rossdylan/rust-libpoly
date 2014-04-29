// `libpoly.rs`
#![feature(globs)]
#![crate_id = "poly#0.1"]
#![comment = "A Musical Instrument for Computers "]
#![license = "MIT"]
#![crate_type = "lib"]

extern crate libc;

use libc::*;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum poly_wavetype {
    poly_sine,
    poly_square,
    poly_saw,
    poly_triangle,
    poly_sample,
    poly_loopsample
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[link(name = "poly")]
extern "C" {
    // manage global libPOLY state
    fn poly_init(bitdepth: c_int, channels: c_int, bitrate: c_int, max: c_int, filename: *libc::c_char) -> c_int;
    fn poly_shutdown();

    // start/stop playback
    fn poly_start() -> c_int;
    fn poly_stop();

    // get generator state
    fn poly_get_init(index: c_int) -> c_char;
    fn poly_get_wavetype(index: c_int) -> poly_wavetype;
    fn poly_get_L_amp(index: c_int) -> c_float;
    fn poly_get_R_amp(index: c_int) -> c_float;
    fn poly_get_freq(index: c_int) -> c_float;
    fn poly_get_phase(index: c_int) -> c_float;
    fn poly_get_duty(index: c_int) -> c_float;
    fn poly_get_sample_bitdepth(index: c_int) -> c_int;
    fn poly_get_sample_length(index: c_int) -> c_int;
    fn poly_get_sample(index: c_int) -> *c_char;

    // set generator state
    fn poly_mute(index: c_int);
    fn poly_unmute(index: c_int);
    fn poly_set_wavetype(index: c_int, wave_type: poly_wavetype);
    fn poly_set_amplitude(index: c_int, amplitude: c_float);
    fn poly_set_L_amp(index: c_int, L_amp: c_float);
    fn poly_set_R_amp(index: c_int, R_amp: c_float);
    fn poly_bump_freq(index: c_int, freq: c_float);
    fn poly_set_freq(index: c_int, freq: c_float);
    fn poly_set_phase(index: c_int, phase: c_float);
    fn poly_set_duty(index: c_int, duty: c_float);
    fn poly_set_sample_bitdepth(index: c_int, sample_bitdepth: c_int);
    fn poly_set_sample_length(index: c_int, sample_length: c_int);
    fn poly_set_sample(index: c_int, sample: *c_char);

    // Initialize a generator with usable defaults
    fn poly_init_generator(index: c_int, wavetype: poly_wavetype, amplitute: c_float, freq: c_float);
}


#[link(name = "ao")]
extern "C" {}

pub struct Poly {
    started: bool
}

// Implement Drop so that we stop and shutdown when we go out of scope
impl Drop for Poly {
    fn drop(&mut self) {
        if self.started {
            self.stop();
        }
        unsafe {
            poly_shutdown();
        }
    }
}

impl Poly {
    /*
        Roughly equates to poly_init
    */
    pub fn new(bitdepth: int, channels: int, bitrate: int, max: int, filename: &str) -> Poly {
        let result = unsafe {
            poly_init(bitdepth as c_int, channels as c_int, bitrate as c_int, max as c_int, filename.to_c_str().unwrap()) as int
        };
        if result != 0 {
            fail!("Failed to init libpoly");
        }
        Poly{started: false}
    }

    pub fn start(&mut self) -> bool {
        let result: int = unsafe {
            poly_start() as int
        };
        if result == 0 {
            self.started = true;
        }
        self.started
    }

    pub fn stop(&mut self) {
        unsafe {
            poly_stop();
        }
        self.started = false;
    }

    pub fn get_init(&self, index: int) -> ~str {
        let result = unsafe {
            poly_get_init(index as c_int)
        };
        result.to_str()
    }

    pub fn get_wavetype(&self, index: int) -> poly_wavetype {
        unsafe {
            poly_get_wavetype(index as c_int)
        }
    }

    pub fn set_wavetype(&self, index: int, wavetype: poly_wavetype) {
        unsafe {
            poly_set_wavetype(index as c_int, wavetype)
        }
    }

    pub fn init_generator(&self, index: int, wavetype: poly_wavetype, amplitute: f64, freq: f64) {
        unsafe {
            poly_init_generator(index as c_int, wavetype, amplitute as c_float, freq as c_float)
        }
    }
}

