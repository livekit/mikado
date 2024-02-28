pub mod coreaudio;

pub fn main() {
    #![cfg(any(target_os = "macos", target_os = "ios"))]
    coreaudio::build();
}
