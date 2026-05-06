use wgpu::{Device, Queue, TextureFormat};

/// Main rendering engine using wgpu
pub struct WgpuRenderer {
    device: Device,
    queue: Queue,
    surface_format: TextureFormat,
}

impl WgpuRenderer {
    /// Create a new renderer
    pub async fn new(device: Device, queue: Queue, surface_format: TextureFormat) -> Self {
        Self {
            device,
            queue,
            surface_format,
        }
    }

    /// Render a frame
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // TODO: Implement rendering
        Ok(())
    }

    /// Get the device
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Get the queue
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Get surface format
    pub fn surface_format(&self) -> TextureFormat {
        self.surface_format
    }
}
