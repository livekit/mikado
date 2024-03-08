use std::{mem, ptr};

use crate::{MikadoError, Result};

pub mod sys {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]

    include!(concat!(env!("OUT_DIR"), "/coreaudio.rs"));
}

#[derive(Debug)]
pub struct Microphone {
    device: sys::AudioDeviceID,
}

pub fn list_microphones() -> Result<Vec<Microphone>> {
    let property_address = sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioHardwarePropertyDefaultInputDevice,
        mScope: sys::kAudioObjectPropertyScopeGlobal,
        mElement: sys::kAudioObjectPropertyElementMaster,
    };

    let mut data_size = 0;
    let status = unsafe {
        sys::AudioObjectGetPropertyDataSize(
            sys::kAudioObjectSystemObject,
            &property_address,
            0,
            ptr::null(),
            &mut data_size,
        )
    };
    if status != sys::kAudioHardwareNoError as i32 {
        return Err(MikadoError::GeneralError(format!(
            "Error getting data size: {}",
            status
        )));
    }

    let device_count = data_size / mem::size_of::<sys::AudioDeviceID>() as u32;
    let mut audio_devices: Vec<sys::AudioDeviceID> = Vec::with_capacity(device_count as usize);

    let status = unsafe {
        sys::AudioObjectGetPropertyData(
            sys::kAudioObjectSystemObject,
            &property_address,
            0,
            ptr::null(),
            &mut data_size,
            audio_devices.as_mut_ptr() as *mut _,
        )
    };

    if status != sys::kAudioHardwareNoError as i32 {
        return Err(MikadoError::GeneralError(format!(
            "Error getting device ids: {}",
            status
        )));
    }

    unsafe {
        audio_devices.set_len(device_count as usize);
    }

    println!("Found {} audio devices", device_count);
    audio_devices.iter().for_each(|id| {
        println!("Device id: {:?}", id);
    });

    let microphones = audio_devices
        .into_iter()
        .map(|id| Microphone { device: id })
        .collect();

    Ok(microphones)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_microphones() {
        dbg!(list_microphones()).ok();
    }
}
