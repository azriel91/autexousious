use shape_model::Volume;

/// Effects of one object on another
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Interaction {
    /// Basic physical attack.
    Physical {
        /// Effect volume.
        bounds: Vec<Volume>,
        /// Amount of health points (HP) to subtract on collision.
        #[serde(default)]
        hp_damage: u32,
        /// Amount of skill points (SP) to subtract on collision.
        #[serde(default)]
        sp_damage: u32,
        /// Whether this will hit multiple objects. Defaults to `false`.
        #[serde(default)]
        multiple: bool,
    },
}
