use shape_model::Volume;

/// Frame for an interactable object.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct CollisionFrame {
    /// Hittable volume of the object.
    #[serde(default)]
    pub body: Option<Vec<Volume>>,
}
