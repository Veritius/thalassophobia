use crate::prelude::*;

type PointSet = SmallVec<[Vec2; 2]>;

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum FloatCurve {
    Constant(f32),
    Logarithm(f32),
    Exponential(f32),
    LinearPoints(PointSet),
    CubicPoints(PointSet),
}

impl Default for FloatCurve {
    fn default() -> Self {
        Self::Constant(1.0)
    }
}

impl FloatCurve {
    pub fn sample(&self, at: f32) -> f32 {
        match self {
            FloatCurve::Constant(v) => *v,
            FloatCurve::Logarithm(e) => at.log10() * e,
            FloatCurve::Exponential(e) => at.powf(*e),

            // You can't sample a lack of points
            FloatCurve::LinearPoints(p) |
            FloatCurve::CubicPoints(p)
                if p.len() == 0 => f32::NAN,

            // One point is the same as Constant
            FloatCurve::LinearPoints(p) |
            FloatCurve::CubicPoints(p)
                if p.len() == 1 => p[0].y,

            // Two points means linear interpolation
            FloatCurve::LinearPoints(p) |
            FloatCurve::CubicPoints(p)
                if p.len() == 2 => {
                    let [[lx, ly], [rx, ry]] = [p[0].to_array(), p[1].to_array()];
                    ly.lerp(ry, factor(at, lx, rx))
                },

            FloatCurve::LinearPoints(_) => todo!(),

            FloatCurve::CubicPoints(_) => todo!(),
        }
    }

    pub fn linear_points<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Vec2>,
    {
        Self::LinearPoints(PointSet::from_iter(iter))
    }

    pub fn cubic_points<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Vec2>,
    {
        Self::CubicPoints(PointSet::from_iter(iter))
    }
}

fn factor(value: f32, min: f32, max: f32) -> f32 {
    (value - min) / (max - min)
}