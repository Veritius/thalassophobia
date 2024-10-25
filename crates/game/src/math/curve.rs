use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

type PointSet = SmallVec<[Vec2; 2]>;

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum FloatCurve {
    Constant(f32),
    LinearPoints(PointSet),
    SmoothedPoints(PointSet),
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
            FloatCurve::SmoothedPoints(p)
                if p.len() == 0 => f32::NAN,

            // One point is the same as Constant
            FloatCurve::LinearPoints(p) |
            FloatCurve::SmoothedPoints(p)
                if p.len() == 1 => p[0].y,

            // Two points means linear interpolation
            FloatCurve::LinearPoints(p) |
            FloatCurve::SmoothedPoints(p)
                if p.len() == 2 => Self::lerp_pts_slice(p, s),

            // Linear points sample for any length above 2
            FloatCurve::LinearPoints(p) => {
                let l = p.len();

                // If less or greater than the bounds of the points in the set
                // we have to extrapolate based on the two points on the start
                // or end of the set, effectively continuing the line to infinity
                if s < p[0].x { return Self::lerp_pts_slice(&p[..2], s); }
                if s > p[l-1].x { return Self::lerp_pts_slice(&p[l-3..l-1], s); }

                Self::lerp_pts_slice(
                    p.windows(2).find(|p| p[0].x <= s && p[1].x >= s).unwrap(),
                    s
                )
            },

            FloatCurve::SmoothedPoints(p) => {
                let l = p.len();

                // out of bounds
                if s < p[0].x { todo!() }
                if s > p[l-1].x { todo!() }

                Self::qerp_pts_slice(
                    p.windows(3).find(|p| p[0].x <= s && p[2].x >= s).unwrap(),
                    s
                )
            },
        }
    }

    pub fn linear_points<I, P>(iter: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<Vec2>,
    {
        Self::LinearPoints(PointSet::from_iter(iter.into_iter().map(|p| p.into())))
    }

    pub fn smoothed_points<I, P>(iter: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<Vec2>,
    {
        Self::SmoothedPoints(PointSet::from_iter(iter.into_iter().map(|p| p.into())))
    }

    fn lerp_pts(ax: f32, ay: f32, bx: f32, by: f32, s: f32) -> f32 {
        ay.lerp(by, (s - ax) / (bx - ax))
    }

    fn lerp_pts_slice(p: &[Vec2], s: f32) -> f32 {
        Self::lerp_pts(p[0].x, p[0].y, p[1].x, p[1].y, s)
    }

    fn qerp_pts(p: [f32; 6], s: f32) -> f32 {
        // Points that we're working with
        let [
            ax, ay,
            bx, by,
            cx, cy,
        ] = p;

        // Intermediate pass of points
        let [ka,kb] = [
            Self::lerp_pts(ax, ay, bx, by, s),
            Self::lerp_pts(bx, by, cx, cy, s),
        ];

        // Our final desired point
        return ka.lerp(kb, s);
    }

    fn qerp_pts_slice(p: &[Vec2], s: f32) -> f32 {
        Self::qerp_pts([
            p[0].x,
            p[0].y,
            p[1].x,
            p[1].y,
            p[2].x,
            p[2].y
        ], s)
    }
}

#[test]
fn float_curve_tests() {
    // Constant value checks
    assert_eq!(FloatCurve::Constant(0.0).sample(0.0), 0.0);
    assert_eq!(FloatCurve::Constant(1.0).sample(0.0), 1.0);
    assert_eq!(FloatCurve::Constant(0.0).sample(1.0), 0.0);
    assert_eq!(FloatCurve::Constant(1.0).sample(1.0), 1.0);

    // Linear point checks with a set equal to 1 item
    let set = FloatCurve::linear_points([
        [2.0, 0.5],
    ]);

    // Within-bounds checks for =2 set
    assert_eq!(set.sample(2.0), 0.5);

    // Outside-bounds checks for >2 set
    assert_eq!(set.sample(4.0), 0.5);
    assert_eq!(set.sample(0.0), 0.5);

    // Linear point checks with a set equal to 2 items
    let set = FloatCurve::linear_points([
        [0.0, 0.0],
        [1.0, 1.0],
    ]);

    // Within-bounds checks for =2 set
    assert_eq!(set.sample(0.0), 0.0);
    assert_eq!(set.sample(0.1), 0.1);
    assert_eq!(set.sample(0.5), 0.5);
    assert_eq!(set.sample(1.0), 1.0);

    // Outside-bounds checks for >2 set
    assert_eq!(set.sample(-0.3), -0.3);
    assert_eq!(set.sample(1.2), 1.2);

    // Linear point checks with a set bigger than 2 items
    let set = FloatCurve::linear_points([
        [-1.0, 0.0],
        [0.0, 1.0],
        [2.0, 3.0],
    ]);

    // Within-bounds checks for >2 set
    assert_eq!(set.sample(-1.0), 0.0);
    assert_eq!(set.sample(-0.5), 0.5);
    assert_eq!(set.sample(0.0), 1.0);
    assert_eq!(set.sample(0.5), 1.5);
    assert_eq!(set.sample(1.0), 2.0);
    assert_eq!(set.sample(1.5), 2.5);
    assert_eq!(set.sample(2.0), 3.0);

    // Outside-bounds checks for >2 set
    assert_eq!(set.sample(-2.0), -1.0);
    assert_eq!(set.sample(4.0), 5.0);
}

#[test]
fn dump_pts() {
    let set = FloatCurve::smoothed_points([
        Vec2::new(-4.0, -2.0),
        Vec2::new(-3.0, -1.5),
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 3.0),
        Vec2::new(4.0, 9.0),
    ]);

    const LOWER: i32 = -40;
    const UPPER: i32 = 40;

    for x in LOWER..=UPPER {
        println!("{x}: {}", set.sample(x as f32 / 10.0));
    }
}