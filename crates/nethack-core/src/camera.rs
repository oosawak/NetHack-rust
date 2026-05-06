//! 3D Camera system with multiple view modes
//! Same 3D world, different camera angles = different visual styles

use glam::{Mat4, Vec3, Quat};

/// Camera view mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Top-down (ASCII-like view)
    TopDown,
    /// 2.5D Isometric (Diablo/Baldur's Gate style)
    Isometric,
    /// First-person (Classic dungeon crawler)
    FirstPerson,
    /// Third-person (follow camera)
    ThirdPerson,
    /// Cinematic (automatic camera movement)
    Cinematic,
}

/// 3D Camera
#[derive(Debug, Clone)]
pub struct Camera3D {
    /// World position
    pub position: Vec3,
    /// Look target
    pub target: Vec3,
    /// Up vector
    pub up: Vec3,
    /// Vertical field of view (degrees)
    pub fov: f32,
    /// Near clip plane
    pub near: f32,
    /// Far clip plane
    pub far: f32,
    /// Current view mode
    pub mode: ViewMode,
}

impl Camera3D {
    /// Create camera for a specific view mode
    pub fn new(center_x: f32, center_y: f32, center_z: f32, mode: ViewMode) -> Self {
        match mode {
            ViewMode::TopDown => Self::top_down(center_x, center_y, center_z),
            ViewMode::Isometric => Self::isometric(center_x, center_y, center_z),
            ViewMode::FirstPerson => Self::first_person(center_x, center_y, center_z),
            ViewMode::ThirdPerson => Self::third_person(center_x, center_y, center_z),
            ViewMode::Cinematic => Self::cinematic(center_x, center_y, center_z),
        }
    }

    /// ASCII-like top-down view (俯瞰)
    /// Looking straight down from above
    pub fn top_down(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3::new(x, 50.0, z),
            target: Vec3::new(x, 0.0, z),
            up: Vec3::new(0.0, 0.0, -1.0),
            fov: 60.0,
            near: 0.1,
            far: 1000.0,
            mode: ViewMode::TopDown,
        }
    }

    /// 2.5D Isometric view (等角投影)
    /// 45-degree angle, like Diablo/Baldur's Gate
    pub fn isometric(x: f32, y: f32, z: f32) -> Self {
        // Camera positioned at 45 degrees around the player
        let distance = 25.0;
        let height = 20.0;
        
        Self {
            position: Vec3::new(
                x + distance * 0.707,  // cos(45°)
                height,
                z + distance * 0.707,  // sin(45°)
            ),
            target: Vec3::new(x, y, z),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 50.0,
            near: 0.1,
            far: 1000.0,
            mode: ViewMode::Isometric,
        }
    }

    /// First-person view (1人称視点)
    /// Camera at player eye level looking forward
    pub fn first_person(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3::new(x, y + 1.7, z),  // Eye height
            target: Vec3::new(x, y + 1.7, z + 10.0),  // Look forward
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 90.0,
            near: 0.1,
            far: 1000.0,
            mode: ViewMode::FirstPerson,
        }
    }

    /// Third-person view (背後視点)
    /// Camera behind and above the player
    pub fn third_person(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3::new(x, y + 2.0, z - 5.0),
            target: Vec3::new(x, y + 1.5, z),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 75.0,
            near: 0.1,
            far: 1000.0,
            mode: ViewMode::ThirdPerson,
        }
    }

    /// Cinematic view (シネマティック)
    /// Dramatic angle for cutscenes
    pub fn cinematic(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3::new(x + 15.0, y + 15.0, z + 15.0),
            target: Vec3::new(x, y + 1.0, z),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 45.0,
            near: 0.1,
            far: 1000.0,
            mode: ViewMode::Cinematic,
        }
    }

    /// Switch view mode (smooth transition)
    pub fn switch_mode(&mut self, mode: ViewMode) {
        let center = self.target;
        *self = Self::new(center.x, center.y, center.z, mode);
    }

    /// Update camera to track player position
    pub fn follow(&mut self, player_x: f32, player_y: f32, player_z: f32) {
        let old_mode = self.mode;
        let offset_x = self.position.x - self.target.x;
        let offset_y = self.position.y - self.target.y;
        let offset_z = self.position.z - self.target.z;

        self.target = Vec3::new(player_x, player_y, player_z);
        self.position = self.target + Vec3::new(offset_x, offset_y, offset_z);
    }

    /// Compute view matrix
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Compute projection matrix
    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::perspective_rh(
            self.fov.to_radians(),
            aspect_ratio,
            self.near,
            self.far,
        )
    }

    /// Compute view-projection matrix
    pub fn view_projection(&self, aspect_ratio: f32) -> Mat4 {
        self.projection_matrix(aspect_ratio) * self.view_matrix()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let cam = Camera3D::new(50.0, 0.0, 50.0, ViewMode::TopDown);
        assert_eq!(cam.mode, ViewMode::TopDown);
        assert_eq!(cam.position.y, 50.0);
    }

    #[test]
    fn test_camera_switch() {
        let mut cam = Camera3D::new(50.0, 0.0, 50.0, ViewMode::TopDown);
        cam.switch_mode(ViewMode::Isometric);
        assert_eq!(cam.mode, ViewMode::Isometric);
    }

    #[test]
    fn test_camera_follow() {
        let mut cam = Camera3D::new(50.0, 0.0, 50.0, ViewMode::TopDown);
        cam.follow(60.0, 0.0, 60.0);
        assert_eq!(cam.target.x, 60.0);
    }
}
