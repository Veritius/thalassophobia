use crate::prelude::*;

type PointSet = SmallVec<[Vec2; 2]>;

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum FloatCurve {
    Constant(f32),
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
            // Constants always return the same value
            FloatCurve::Constant(v) => *v,

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
                    ly.lerp(ry, Self::factor(at, lx, rx))
                },

            FloatCurve::LinearPoints(_) => todo!(),

            FloatCurve::CubicPoints(_) => todo!(),
        }
    }

    pub fn linear_points<I, P>(iter: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<Vec2>,
    {
        Self::LinearPoints(PointSet::from_iter(iter.into_iter().map(|p| p.into())))
    }

    pub fn cubic_points<I, P>(iter: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<Vec2>,
    {
        Self::CubicPoints(PointSet::from_iter(iter.into_iter().map(|p| p.into())))
    }

    fn factor(value: f32, min: f32, max: f32) -> f32 {
        (value - min) / (max - min)
    }
}

#[test]
fn float_curve_tests() {
    // Constant value checks
    assert_eq!(FloatCurve::Constant(0.0).sample(0.0), 0.0);
    assert_eq!(FloatCurve::Constant(1.0).sample(0.0), 1.0);
    assert_eq!(FloatCurve::Constant(0.0).sample(1.0), 0.0);
    assert_eq!(FloatCurve::Constant(1.0).sample(1.0), 1.0);

    // Linear point checks (within range)
    assert_eq!(FloatCurve::linear_points([[0.0, 0.0], [1.0, 1.0]]).sample(0.0), 0.0);
    assert_eq!(FloatCurve::linear_points([[0.0, 0.0], [1.0, 1.0]]).sample(0.1), 0.1);
    assert_eq!(FloatCurve::linear_points([[0.0, 0.0], [1.0, 1.0]]).sample(0.5), 0.5);
    assert_eq!(FloatCurve::linear_points([[0.0, 0.0], [1.0, 1.0]]).sample(1.0), 1.0);

    // Linear point checks (outside range)
    assert_eq!(FloatCurve::linear_points([[0.0, 0.0], [1.0, 1.0]]).sample(-0.3), -0.3);
    assert_eq!(FloatCurve::linear_points([[0.0, 0.0], [1.0, 1.0]]).sample(1.2), 1.2);
}