use crate::prelude::*;

type PointSet = SmallVec<[Vec2; 2]>;

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum FloatCurve {
    Flat(f32),
    Logarithm(f32),
    Exponential(f32),
    LinearPoints(PointSet),
    SmoothedPoints(PointSet),
}

impl Default for FloatCurve {
    fn default() -> Self {
        Self::Flat(1.0)
    }
}

impl FloatCurve {
    pub fn sample(&self, at: f32) -> f32 {
        match self {
            FloatCurve::Flat(v) => *v,
            FloatCurve::Logarithm(e) => at.log10() * e,
            FloatCurve::Exponential(e) => at.powf(*e),
            FloatCurve::LinearPoints(_) => todo!(),
            FloatCurve::SmoothedPoints(_) => todo!(),
        }
    }
}