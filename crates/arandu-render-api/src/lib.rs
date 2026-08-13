use arandu_confluence::ConfluenceState;
use arandu_geometry::{AdaptiveLayout, Point};
use arandu_model::{AccentFamily, ApplicationId, RiverKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderNodeInput {
    pub id: ApplicationId,
    pub name: String,
    pub river: RiverKind,
    pub arandu_symbol: String,
    pub original_icon_reference: Option<String>,
    pub accent: AccentFamily,
    pub focused: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeedPrimitive {
    pub center: Point,
    pub radius: f32,
    pub opacity: f32,
    pub scale: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodePrimitive {
    pub id: ApplicationId,
    pub name: String,
    pub river: RiverKind,
    pub center: Point,
    pub radius: f32,
    pub opacity: f32,
    pub scale: f32,
    pub arandu_symbol: String,
    pub original_icon_reference: Option<String>,
    pub accent: AccentFamily,
    pub focused: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConnectorPrimitive {
    pub river: RiverKind,
    pub from: Point,
    pub to: Point,
    pub opacity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LabelPrimitive {
    pub river: RiverKind,
    pub anchor: Point,
    pub opacity: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderScene {
    pub confluence_state: ConfluenceState,
    pub reveal_factor: f32,
    pub seed: SeedPrimitive,
    pub nodes: Vec<NodePrimitive>,
    pub connectors: Vec<ConnectorPrimitive>,
    pub labels: Vec<LabelPrimitive>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderPolicy {
    pub seed_radius: f32,
    pub node_radius: f32,
    pub seed_idle_opacity: f32,
    pub label_reveal_threshold: f32,
}

impl Default for RenderPolicy {
    fn default() -> Self {
        Self {
            seed_radius: 28.0,
            node_radius: 24.0,
            seed_idle_opacity: 0.72,
            label_reveal_threshold: 0.42,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SceneBuilder {
    policy: RenderPolicy,
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self::new(RenderPolicy::default())
    }
}

impl SceneBuilder {
    #[must_use]
    pub const fn new(policy: RenderPolicy) -> Self {
        Self { policy }
    }

    #[must_use]
    pub fn build(
        &self,
        layout: &AdaptiveLayout,
        state: ConfluenceState,
        reveal_factor: f32,
        inputs: &[RenderNodeInput],
    ) -> RenderScene {
        let reveal = normalized_reveal(state, reveal_factor);
        let seed = SeedPrimitive {
            center: layout.seed,
            radius: self.policy.seed_radius,
            opacity: if state == ConfluenceState::Collapsed {
                self.policy.seed_idle_opacity
            } else {
                1.0
            },
            scale: 0.94 + 0.06 * reveal,
        };

        let mut nodes = Vec::new();
        let mut connectors = Vec::new();
        let mut labels = Vec::new();

        for river in RiverKind::ALL {
            let river_layout = layout.river(river);
            let river_inputs: Vec<&RenderNodeInput> =
                inputs.iter().filter(|input| input.river == river).collect();
            let count = river_layout.nodes.len().min(river_inputs.len());
            let mut previous = layout.seed;

            for (index, input) in river_inputs.iter().take(count).enumerate() {
                let target = river_layout.nodes[index];
                let center = interpolate(layout.seed, target, reveal);

                nodes.push(NodePrimitive {
                    id: input.id.clone(),
                    name: input.name.clone(),
                    river,
                    center,
                    radius: self.policy.node_radius,
                    opacity: reveal,
                    scale: node_scale(reveal, input.focused, input.selected),
                    arandu_symbol: input.arandu_symbol.clone(),
                    original_icon_reference: input.original_icon_reference.clone(),
                    accent: input.accent,
                    focused: input.focused,
                    selected: input.selected,
                });

                connectors.push(ConnectorPrimitive {
                    river,
                    from: previous,
                    to: center,
                    opacity: reveal * 0.82,
                });
                previous = center;
            }

            if count > 0 && reveal >= self.policy.label_reveal_threshold {
                let last = river_layout.nodes[count - 1];
                labels.push(LabelPrimitive {
                    river,
                    anchor: interpolate(layout.seed, last, reveal),
                    opacity: ((reveal - self.policy.label_reveal_threshold)
                        / (1.0 - self.policy.label_reveal_threshold))
                        .clamp(0.0, 1.0),
                });
            }
        }

        RenderScene {
            confluence_state: state,
            reveal_factor: reveal,
            seed,
            nodes,
            connectors,
            labels,
        }
    }
}

fn normalized_reveal(state: ConfluenceState, reveal_factor: f32) -> f32 {
    match state {
        ConfluenceState::Collapsed => 0.0,
        ConfluenceState::Expanded | ConfluenceState::Pinned => 1.0,
        ConfluenceState::Aware | ConfluenceState::Near => reveal_factor.clamp(0.0, 1.0),
    }
}

fn interpolate(origin: Point, target: Point, factor: f32) -> Point {
    Point::new(
        origin.x + (target.x - origin.x) * factor,
        origin.y + (target.y - origin.y) * factor,
    )
}

fn node_scale(reveal: f32, focused: bool, selected: bool) -> f32 {
    let base = 0.72 + 0.28 * reveal;
    if focused {
        base * 1.14
    } else if selected {
        base * 1.08
    } else {
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arandu_geometry::{GeometryEngine, RiverCounts, Viewport};

    fn layout() -> AdaptiveLayout {
        GeometryEngine::default().layout(
            Viewport::new(1200.0, 800.0),
            Point::new(600.0, 400.0),
            RiverCounts {
                memory: 2,
                open: 1,
                frequent: 1,
            },
        )
    }

    fn inputs() -> Vec<RenderNodeInput> {
        vec![
            RenderNodeInput {
                id: ApplicationId::new("memory-a"),
                name: "Memory A".to_string(),
                river: RiverKind::Memory,
                arandu_symbol: "browser".to_string(),
                original_icon_reference: Some("firefox.desktop".to_string()),
                accent: AccentFamily::Teal,
                focused: true,
                selected: false,
            },
            RenderNodeInput {
                id: ApplicationId::new("memory-b"),
                name: "Memory B".to_string(),
                river: RiverKind::Memory,
                arandu_symbol: "terminal".to_string(),
                original_icon_reference: None,
                accent: AccentFamily::Ochre,
                focused: false,
                selected: true,
            },
            RenderNodeInput {
                id: ApplicationId::new("open-a"),
                name: "Open A".to_string(),
                river: RiverKind::Open,
                arandu_symbol: "files".to_string(),
                original_icon_reference: None,
                accent: AccentFamily::Silver,
                focused: false,
                selected: false,
            },
            RenderNodeInput {
                id: ApplicationId::new("frequent-a"),
                name: "Frequent A".to_string(),
                river: RiverKind::Frequent,
                arandu_symbol: "document".to_string(),
                original_icon_reference: None,
                accent: AccentFamily::Amber,
                focused: false,
                selected: false,
            },
        ]
    }

    #[test]
    fn collapsed_scene_keeps_seed_and_hides_rivers() {
        let scene =
            SceneBuilder::default().build(&layout(), ConfluenceState::Collapsed, 0.0, &inputs());
        assert!(scene.reveal_factor.abs() < f32::EPSILON);
        assert!(scene.nodes.iter().all(|node| node.opacity == 0.0));
        assert!(scene.labels.is_empty());
        assert!(scene
            .nodes
            .iter()
            .all(|node| node.center == scene.seed.center));
    }

    #[test]
    fn expanded_scene_places_all_nodes_at_adaptive_targets() {
        let adaptive = layout();
        let scene =
            SceneBuilder::default().build(&adaptive, ConfluenceState::Expanded, 1.0, &inputs());
        assert_eq!(scene.nodes.len(), 4);
        assert_eq!(scene.connectors.len(), 4);
        assert_eq!(scene.labels.len(), 3);
    }

    #[test]
    fn near_scene_interpolates_nodes_from_seed() {
        let adaptive = layout();
        let scene = SceneBuilder::default().build(&adaptive, ConfluenceState::Near, 0.5, &inputs());
        let first = &scene.nodes[0];
        assert_ne!(first.center, scene.seed.center);
        assert!(first.opacity > 0.0 && first.opacity < 1.0);
    }

    #[test]
    fn original_icon_reference_is_preserved_as_secondary_identity() {
        let scene =
            SceneBuilder::default().build(&layout(), ConfluenceState::Expanded, 1.0, &inputs());
        let node = scene
            .nodes
            .iter()
            .find(|node| node.id == ApplicationId::new("memory-a"))
            .expect("memory node exists");
        assert_eq!(
            node.original_icon_reference.as_deref(),
            Some("firefox.desktop")
        );
        assert_eq!(node.arandu_symbol, "browser");
    }

    #[test]
    fn focused_node_receives_stronger_scale_than_normal_node() {
        let scene =
            SceneBuilder::default().build(&layout(), ConfluenceState::Expanded, 1.0, &inputs());
        let focused = scene
            .nodes
            .iter()
            .find(|node| node.id == ApplicationId::new("memory-a"))
            .expect("focused node exists");
        let normal = scene
            .nodes
            .iter()
            .find(|node| node.id == ApplicationId::new("open-a"))
            .expect("normal node exists");
        assert!(focused.scale > normal.scale);
    }
}
