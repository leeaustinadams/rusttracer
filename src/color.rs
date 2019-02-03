#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub fn black() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    pub fn white() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }

    pub fn grey(v: f32) -> Color {
        Color { r: v, g: v, b: v, a: 1.0 }
    }

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn saturate(&self) -> Color {
        Color { r: self.r.min(1.0).max(0.0),
                g: self.g.min(1.0).max(0.0),
                b: self.b.min(1.0).max(0.0),
                a: self.a.min(1.0).max(0.0) }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {r: self.r * scalar,
               g: self.g * scalar,
               b: self.b * scalar,
               a: self.a}
    }
}

impl std::ops::Div<f32> for Color {
    type Output = Color;

    fn div(self, scalar: f32) -> Color {
        Color {r: self.r / scalar,
               g: self.g / scalar,
               b: self.b / scalar,
               a: self.a}
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {r: self.r * rhs.r,
               g: self.g * rhs.g,
               b: self.b * rhs.b,
               a: self.a}
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {r: self.r + rhs.r,
               g: self.g + rhs.g,
               b: self.b + rhs.b,
               a: self.a}
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
