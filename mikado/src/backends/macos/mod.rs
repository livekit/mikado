use icrate::Foundation::{MainThreadMarker, NSThread};

use crate::{MikadoError, Result};

#[derive(Debug)]
pub struct Screen {
    pub width: f64,
    pub height: f64,
}

pub struct MikadoContext {
    mtm: MainThreadMarker,
}

impl MikadoContext {
    pub fn new() -> Result<Self> {
        // todo: For some reason, doing this the safe way always errors...
        let mtm = unsafe { MainThreadMarker::new_unchecked() }
        // .ok_or_else(|| {
        //     MikadoError::GeneralError("Mikado must be initialized on the main thread".to_string())
        // })?
        ;
        Ok(Self { mtm })
    }

    pub fn list_screens(&self) -> Vec<Screen> {
        icrate::AppKit::NSScreen::screens(self.mtm)
            .into_iter()
            .map(|screen| Screen {
                width: screen.frame().size.width,
                height: screen.frame().size.height,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::MikadoContext;

    #[test]
    fn test_list_screens() {
        let cx = MikadoContext::new().unwrap();
        dbg!(cx.list_screens());
        unimplemented!()
    }
}
