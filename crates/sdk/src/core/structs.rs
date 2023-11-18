//---------------------------------------------------------------------------------------------------------------------
// UE-4 CoreUObject Structs
//---------------------------------------------------------------------------------------------------------------------

/// Struct CoreUObject.Color
/// Size -> 0x0004
#[derive(Debug, Clone, Copy)]
pub struct FColor {
    pub b: u8,
    // 0x00(0x01)
    pub g: u8,
    // 0x01(0x01)
    pub r: u8,
    // 0x02(0x01)
    pub a: u8, // 0x03(0x01)
}

/// Struct CoreUObject.LinearColor
/// Size -> 0x0010
#[derive(Debug, Clone, Copy)]
pub struct FLinearColor {
    pub r: f32,
    // 0x00(0x04)
    pub g: f32,
    // 0x04(0x04)
    pub b: f32,
    // 0x08(0x04)
    pub a: f32, // 0x0C(0x04)
}

/// Struct CoreUObject.Vector2D
/// Size -> 0x0008
#[derive(Debug, Clone, Copy)]
pub struct FVector2D {
    pub x: f32,
    // 0x00(0x04)
    pub y: f32, // 0x04(0x04)
}

impl FVector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

impl std::ops::Add<f32> for FVector2D {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        FVector2D {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

/// ScriptStruct CoreUObject.Vector
/// Size -> 0x000C
#[derive(Debug, Clone, Copy)]
pub struct FVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FVector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
}

impl std::ops::Add<f32> for FVector {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        FVector {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl std::ops::Sub<f32> for FVector {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        FVector {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl std::ops::Add for FVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        FVector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for FVector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        FVector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::AddAssign<f32> for FVector {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl std::ops::SubAssign<f32> for FVector {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl std::ops::AddAssign for FVector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::SubAssign for FVector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
