use arandu_model::RiverKind;

/// A device-independent point in logical display units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Logical rectangular drawing area supplied by a platform renderer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    /// Creates a positive viewport.
    ///
    /// # Panics
    ///
    /// Panics if width or height is not finite or is not strictly positive.
    #[must_use]
    pub fn new(width: f32, height: f32) -> Self {
        assert!(
            width.is_finite() && width > 0.0,
            "viewport width must be positive and finite"
        );
        assert!(
            height.is_finite() && height > 0.0,
            "viewport height must be positive and finite"
        );
        Self { width, height }
    }
}

/// Number of application nodes currently present in each river.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RiverCounts {
    pub memory: usize,
    pub open: usize,
    pub frequent: usize,
}

impl RiverCounts {
    #[must_use]
    pub const fn for_kind(self, kind: RiverKind) -> usize {
        match kind {
            RiverKind::Memory => self.memory,
            RiverKind::Open => self.open,
            RiverKind::Frequent => self.frequent,
        }
    }
}

/// Geometric constraints shared by all renderers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeometryConfig {
    /// Minimum distance from any node center to a screen edge.
    pub safe_margin: f32,
    /// Preferred spacing between neighboring application nodes.
    pub preferred_spacing: f32,
    /// Smallest acceptable spacing on constrained screens.
    pub minimum_spacing: f32,
    /// Distance between the Confluence Seed and the first application node.
    pub seed_gap: f32,
    /// Preferred diagonal angle used by FREQUENTES.
    pub frequent_angle_degrees: f32,
}

impl Default for GeometryConfig {
    fn default() -> Self {
        Self {
            safe_margin: 44.0,
            preferred_spacing: 76.0,
            minimum_spacing: 48.0,
            seed_gap: 68.0,
            frequent_angle_degrees: 38.0,
        }
    }
}

/// Result for one adaptive river.
#[derive(Debug, Clone, PartialEq)]
pub struct RiverLayout {
    pub kind: RiverKind,
    pub nodes: Vec<Point>,
    /// Normalized direction vector from the seed toward this river.
    pub direction: Point,
    /// Spacing actually selected after fitting the viewport.
    pub spacing: f32,
}

/// Complete device-independent geometry for the three ARANDU rivers.
#[derive(Debug, Clone, PartialEq)]
pub struct AdaptiveLayout {
    pub seed: Point,
    pub memory: RiverLayout,
    pub open: RiverLayout,
    pub frequent: RiverLayout,
    /// True when at least one river had to use less than preferred spacing.
    pub compacted: bool,
}

impl AdaptiveLayout {
    #[must_use]
    pub fn river(&self, kind: RiverKind) -> &RiverLayout {
        match kind {
            RiverKind::Memory => &self.memory,
            RiverKind::Open => &self.open,
            RiverKind::Frequent => &self.frequent,
        }
    }
}

/// Platform-independent geometry engine for the Confluence and three rivers.
#[derive(Debug, Clone, Copy)]
pub struct GeometryEngine {
    config: GeometryConfig,
}

impl Default for GeometryEngine {
    fn default() -> Self {
        Self::new(GeometryConfig::default())
    }
}

impl GeometryEngine {
    #[must_use]
    pub fn new(config: GeometryConfig) -> Self {
        Self {
            config: normalize_config(config),
        }
    }

    /// Computes positions for all nodes while keeping them inside the viewport.
    ///
    /// The preferred topology is MEMÓRIA horizontal, ABERTOS vertical and
    /// FREQUENTES diagonal. Each river may flip direction at screen edges and
    /// compress its spacing when necessary. The seed itself is clamped into the
    /// safe area before any river is calculated.
    #[must_use]
    pub fn layout(
        &self,
        viewport: Viewport,
        requested_seed: Point,
        counts: RiverCounts,
    ) -> AdaptiveLayout {
        let seed = clamp_seed(viewport, requested_seed, self.config.safe_margin);

        let memory = self.layout_axis(
            viewport,
            seed,
            RiverKind::Memory,
            counts.memory,
            Point::new(-1.0, 0.0),
            Point::new(1.0, 0.0),
        );

        let open = self.layout_axis(
            viewport,
            seed,
            RiverKind::Open,
            counts.open,
            Point::new(0.0, -1.0),
            Point::new(0.0, 1.0),
        );

        let angle = self.config.frequent_angle_degrees.to_radians();
        let primary = normalize_vector(Point::new(angle.cos(), angle.sin()));
        let secondary = Point::new(-primary.x, -primary.y);
        let frequent = self.layout_axis(
            viewport,
            seed,
            RiverKind::Frequent,
            counts.frequent,
            primary,
            secondary,
        );

        let compacted = [memory.spacing, open.spacing, frequent.spacing]
            .into_iter()
            .any(|spacing| spacing + f32::EPSILON < self.config.preferred_spacing);

        AdaptiveLayout {
            seed,
            memory,
            open,
            frequent,
            compacted,
        }
    }

    fn layout_axis(
        &self,
        viewport: Viewport,
        seed: Point,
        kind: RiverKind,
        count: usize,
        primary: Point,
        secondary: Point,
    ) -> RiverLayout {
        if count == 0 {
            return RiverLayout {
                kind,
                nodes: Vec::new(),
                direction: primary,
                spacing: self.config.preferred_spacing,
            };
        }

        let primary_room = available_distance(viewport, seed, primary, self.config.safe_margin);
        let secondary_room = available_distance(viewport, seed, secondary, self.config.safe_margin);
        let (direction, room) = if secondary_room > primary_room {
            (secondary, secondary_room)
        } else {
            (primary, primary_room)
        };

        let spacing = fitted_spacing(
            room,
            count,
            self.config.seed_gap,
            self.config.preferred_spacing,
            self.config.minimum_spacing,
        );

        let seed_gap = fitted_seed_gap(room, count, spacing, self.config.seed_gap);
        let mut nodes = Vec::with_capacity(count);
        for index in 0..count {
            let distance = seed_gap + spacing * usize_to_f32(index);
            nodes.push(clamp_point(
                viewport,
                Point::new(
                    seed.x + direction.x * distance,
                    seed.y + direction.y * distance,
                ),
                self.config.safe_margin,
            ));
        }

        RiverLayout {
            kind,
            nodes,
            direction,
            spacing,
        }
    }
}

fn normalize_config(config: GeometryConfig) -> GeometryConfig {
    let safe_margin = finite_non_negative(config.safe_margin, 44.0);
    let minimum_spacing = finite_positive(config.minimum_spacing, 48.0);
    let preferred_spacing = finite_positive(config.preferred_spacing, 76.0).max(minimum_spacing);
    let seed_gap = finite_non_negative(config.seed_gap, 68.0);
    let frequent_angle_degrees = if config.frequent_angle_degrees.is_finite() {
        config.frequent_angle_degrees.clamp(15.0, 75.0)
    } else {
        38.0
    };
    GeometryConfig {
        safe_margin,
        preferred_spacing,
        minimum_spacing,
        seed_gap,
        frequent_angle_degrees,
    }
}

fn finite_positive(value: f32, fallback: f32) -> f32 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        fallback
    }
}

fn finite_non_negative(value: f32, fallback: f32) -> f32 {
    if value.is_finite() && value >= 0.0 {
        value
    } else {
        fallback
    }
}

fn normalize_vector(vector: Point) -> Point {
    let length = vector.x.hypot(vector.y);
    if length <= f32::EPSILON || !length.is_finite() {
        Point::new(1.0, 0.0)
    } else {
        Point::new(vector.x / length, vector.y / length)
    }
}

fn clamp_seed(viewport: Viewport, seed: Point, margin: f32) -> Point {
    clamp_point(viewport, seed, effective_margin(viewport, margin))
}

fn clamp_point(viewport: Viewport, point: Point, margin: f32) -> Point {
    let effective = effective_margin(viewport, margin);
    Point::new(
        point.x.clamp(effective, viewport.width - effective),
        point.y.clamp(effective, viewport.height - effective),
    )
}

fn effective_margin(viewport: Viewport, margin: f32) -> f32 {
    let maximum = (viewport.width.min(viewport.height) / 2.0 - 1.0).max(0.0);
    margin.min(maximum)
}

fn available_distance(viewport: Viewport, origin: Point, direction: Point, margin: f32) -> f32 {
    let effective = effective_margin(viewport, margin);
    let max_x = viewport.width - effective;
    let max_y = viewport.height - effective;
    let min_x = effective;
    let min_y = effective;
    let mut limit = f32::INFINITY;

    if direction.x > f32::EPSILON {
        limit = limit.min((max_x - origin.x) / direction.x);
    } else if direction.x < -f32::EPSILON {
        limit = limit.min((min_x - origin.x) / direction.x);
    }

    if direction.y > f32::EPSILON {
        limit = limit.min((max_y - origin.y) / direction.y);
    } else if direction.y < -f32::EPSILON {
        limit = limit.min((min_y - origin.y) / direction.y);
    }

    if limit.is_finite() {
        limit.max(0.0)
    } else {
        0.0
    }
}

fn usize_to_f32(value: usize) -> f32 {
    let bounded = u16::try_from(value).unwrap_or(u16::MAX);
    f32::from(bounded)
}

fn fitted_spacing(room: f32, count: usize, seed_gap: f32, preferred: f32, minimum: f32) -> f32 {
    if count <= 1 {
        return preferred;
    }
    let usable = (room - seed_gap).max(0.0);
    let fit = usable / usize_to_f32(count - 1);
    fit.clamp(minimum.min(preferred), preferred)
}

fn fitted_seed_gap(room: f32, count: usize, spacing: f32, preferred_gap: f32) -> f32 {
    if count == 0 {
        return 0.0;
    }
    let tail = spacing * usize_to_f32(count.saturating_sub(1));
    preferred_gap.min((room - tail).max(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inside(viewport: Viewport, point: Point, margin: f32) -> bool {
        point.x >= margin
            && point.x <= viewport.width - margin
            && point.y >= margin
            && point.y <= viewport.height - margin
    }

    #[test]
    fn center_seed_preserves_three_distinct_river_directions() {
        let engine = GeometryEngine::default();
        let viewport = Viewport::new(1920.0, 1080.0);
        let layout = engine.layout(
            viewport,
            Point::new(960.0, 540.0),
            RiverCounts {
                memory: 5,
                open: 4,
                frequent: 5,
            },
        );

        assert!(layout.memory.direction.y.abs() <= f32::EPSILON);
        assert!(layout.open.direction.x.abs() <= f32::EPSILON);
        assert!(layout.frequent.direction.x.abs() > 0.1);
        assert!(layout.frequent.direction.y.abs() > 0.1);
    }

    #[test]
    fn seed_near_left_edge_flips_memory_inward() {
        let engine = GeometryEngine::default();
        let viewport = Viewport::new(1366.0, 768.0);
        let layout = engine.layout(
            viewport,
            Point::new(50.0, 500.0),
            RiverCounts {
                memory: 5,
                open: 3,
                frequent: 5,
            },
        );
        assert!(layout.memory.direction.x > 0.0);
        assert!(layout
            .memory
            .nodes
            .iter()
            .all(|point| inside(viewport, *point, 44.0)));
    }

    #[test]
    fn seed_near_bottom_edge_flips_open_river_upward() {
        let engine = GeometryEngine::default();
        let viewport = Viewport::new(1280.0, 720.0);
        let layout = engine.layout(
            viewport,
            Point::new(620.0, 690.0),
            RiverCounts {
                memory: 4,
                open: 5,
                frequent: 4,
            },
        );
        assert!(layout.open.direction.y < 0.0);
        assert!(layout
            .open
            .nodes
            .iter()
            .all(|point| inside(viewport, *point, 44.0)));
    }

    #[test]
    fn constrained_viewport_compacts_spacing_without_cutting_nodes() {
        let engine = GeometryEngine::default();
        let viewport = Viewport::new(640.0, 480.0);
        let layout = engine.layout(
            viewport,
            Point::new(320.0, 240.0),
            RiverCounts {
                memory: 5,
                open: 5,
                frequent: 5,
            },
        );
        assert!(layout.compacted);
        for kind in RiverKind::ALL {
            assert!(layout
                .river(kind)
                .nodes
                .iter()
                .all(|point| inside(viewport, *point, 44.0)));
        }
    }

    #[test]
    fn node_count_directly_controls_each_river_length() {
        let engine = GeometryEngine::default();
        let layout = engine.layout(
            Viewport::new(1600.0, 900.0),
            Point::new(800.0, 450.0),
            RiverCounts {
                memory: 2,
                open: 4,
                frequent: 1,
            },
        );
        assert_eq!(layout.memory.nodes.len(), 2);
        assert_eq!(layout.open.nodes.len(), 4);
        assert_eq!(layout.frequent.nodes.len(), 1);
    }

    #[test]
    fn requested_seed_outside_viewport_is_clamped_safely() {
        let engine = GeometryEngine::default();
        let viewport = Viewport::new(1024.0, 768.0);
        let layout = engine.layout(viewport, Point::new(-500.0, 9000.0), RiverCounts::default());
        assert!(inside(viewport, layout.seed, 44.0));
    }
}
