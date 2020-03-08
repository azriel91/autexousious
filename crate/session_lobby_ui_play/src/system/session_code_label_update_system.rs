use amethyst::{
    ecs::{Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::UiText,
};
use derivative::Derivative;
use derive_new::new;
use network_session_model::play::SessionCode;
use session_lobby_ui_model::loaded::SessionCodeLabel;

/// Updates `SessionCodeLabel` entities' text with the current session code.
#[derive(Debug, new)]
pub struct SessionCodeLabelUpdateSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionCodeLabelUpdateSystemData<'s> {
    /// `SessionCode` resource.
    #[derivative(Debug = "ignore")]
    pub session_code: Read<'s, SessionCode>,
    /// `SessionCodeLabel` components.
    #[derivative(Debug = "ignore")]
    pub session_code_labels: ReadStorage<'s, SessionCodeLabel>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
}

impl<'s> System<'s> for SessionCodeLabelUpdateSystem {
    type SystemData = SessionCodeLabelUpdateSystemData<'s>;

    fn run(
        &mut self,
        SessionCodeLabelUpdateSystemData {
            session_code,
            session_code_labels,
            mut ui_texts,
        }: Self::SystemData,
    ) {
        (&session_code_labels, &mut ui_texts)
            .join()
            .filter(|(_, ui_text)| &ui_text.text != &session_code.0)
            .for_each(|(_, ui_text)| ui_text.text = session_code.0.clone());
    }
}
