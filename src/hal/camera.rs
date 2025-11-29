use anyhow::{Context, Ok, Result};
use opencv::{core, imgproc, prelude::*, videoio};

pub struct Camera {
    cap: videoio::VideoCapture,
}

impl Camera {
    pub fn new(index: i32) -> Result<Self> {
        let mut cap = videoio::VideoCapture::new(index, videoio::CAP_V4L2)
            .context("Failed to open camera")?;

        cap.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
        cap.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;
        cap.set(videoio::CAP_PROP_FPS, 30.0)?;

        if !videoio::VideoCapture::is_opened(&cap)? {
            anyhow::bail!("Camera was not opened");
        }

        Ok(Camera { cap })
    }

    pub fn frame(&mut self) -> Result<Mat> {
        let mut frame = Mat::default();
        self.cap.read(&mut frame)?;

        if frame.size()?.width == 0 {
            anyhow::bail!("Captured empty frame");
        }

        Ok(frame)
    }

    pub fn grayscale(&mut self) -> Result<Mat> {
        let frame = self.frame()?;
        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        Ok(gray)
    }
}
