#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Default, PartialEq, Clone, Copy)]
pub enum SpawnSelection {
    None,           // Mothing selected
    Other,          // Anything else
    MultiSelection, // Use when multi-selecting
    #[default]
    StarshipConstructionYard, // Only
    SupportShip,    // Only
    Starbase,       // Only
}
