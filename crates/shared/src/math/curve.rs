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
    pub fn sample(&self, s: f32) -> f32 {
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
                if p.len() == 2 => Self::lerp_pts(p, s),

            // Linear points sample for any length above 2
            FloatCurve::LinearPoints(p) => {
                let l = p.len();

                // If less or greater than the bounds of the points in the set
                // we have to extrapolate based on the two points on the start
                // or end of the set, effectively continuing the line to infinity
                if s < p[0].x { return Self::lerp_pts(&p[..2], s); }
                if s > p[l-1].x { return Self::lerp_pts(&p[l-2..l-1], s); }

                Self::lerp_pts(
                    p.windows(2).find(|p| p[0].x <= s && p[1].x >= s).unwrap(),
                    s
                )
            },

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

    fn lerp_pts(p: &[Vec2], s: f32) -> f32 {
        let [[lx, ly], [rx, ry]] = [p[0].to_array(), p[1].to_array()];
        return ly.lerp(ry, (s - lx) / (rx - lx));
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

    // Linear point checks with a set bigger than 2 items
    let mut set = FloatCurve::linear_points([
        [-1.0, 0.0],
        [0.0, 1.0],
        [2.0, 3.0],
    ]);

    // Within-bounds checks for >2 set
    assert_eq!(set.sample(-1.0), 0.0);
    assert_eq!(set.sample(-0.5), 0.5);
    assert_eq!(set.sample(0.0), 1.0);
    assert_eq!(set.sample(1.0), 2.0);
    assert_eq!(set.sample(2.0), 3.0);
}