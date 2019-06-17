use amethyst::{assets::Handle, Error};
use energy_model::{
    config::EnergyDefinition,
    loaded::{Energy, EnergyObjectWrapper},
};

/// Loads assets specified by energy configuration into the loaded energy model.
#[derive(Debug)]
pub enum EnergyLoader {}

impl EnergyLoader {
    /// Returns the loaded `Energy`.
    ///
    /// # Parameters
    ///
    /// * `energy_definition`: Energy definition asset.
    /// * `object_wrapper_handle`: Handle to the loaded `Object` for this energy.
    pub fn load(
        _energy_definition: &EnergyDefinition,
        object_wrapper_handle: Handle<EnergyObjectWrapper>,
    ) -> Result<Energy, Error> {
        Ok(Energy::new(object_wrapper_handle))
    }
}
