use std::{marker::PhantomData, rc::Rc};

use crate::{MikadoError, Result, ScreenDisplay};

use core_graphics::display::{CGDirectDisplayID, CGDisplayBounds, CGGetActiveDisplayList};

#[derive(Debug)]
pub struct Screen {
    pub platform_id: CGDirectDisplayID,
}

pub struct MikadoContext {
    not_send: PhantomData<Rc<()>>,
}

impl MikadoContext {
    /// SAFETY: This must be created on your application's main thread
    pub unsafe fn new() -> Self {
        Self {
            not_send: PhantomData,
        }
    }

    pub fn list_screens(&self) -> Result<Vec<Screen>> {
        let mut displays: Vec<CGDirectDisplayID> = Vec::with_capacity(32);

        unsafe {
            let mut display_count = 0u32;
            let result = CGGetActiveDisplayList(
                displays.capacity() as u32,
                displays.as_mut_ptr(),
                &mut display_count,
            );
            if result != 0 {
                return Err(MikadoError::GeneralError(format!(
                    "CGSetActiveDisplayList returned {}",
                    result
                )));
            }
            displays.set_len(display_count as usize);
        };

        Ok(displays
            .into_iter()
            .map(|id| Screen { platform_id: id })
            .collect())
    }
}

impl Screen {
    pub fn bounds(&self) -> ScreenDisplay {
        unsafe {
            // CGDisplayBounds is in "global display" coordinates, where 0 is
            // the top left of the primary display.
            let bounds = CGDisplayBounds(self.platform_id);

            ScreenDisplay {
                origin: (bounds.origin.x as f32, bounds.origin.y as f32),
                size: (bounds.size.width as f32, bounds.size.height as f32),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MikadoContext;

    #[test]
    fn test_list_screens() {
        let cx = unsafe { MikadoContext::new() };
        let screens = cx.list_screens().unwrap();
        assert!(screens.len() >= 1);
    }
}
