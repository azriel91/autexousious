use derivative::Derivative;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, IntoStaticStr};
use typename_derive::TypeName;

/// `Character` Sequence IDs.
#[derive(
    Clone,
    Copy,
    Debug,
    Derivative,
    Deserialize,
    Display,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    TypeName,
)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CharacterSequenceId {
    /// Default sequence for characters.
    #[derivative(Default)]
    Stand,
    /// Attack while standing.
    #[serde(rename = "stand_attack_0")]
    StandAttack0,
    /// Attack while standing, second sequence.
    ///
    /// This is generally used when pressing attack near the end of the `StandAttack0` sequence.
    #[serde(rename = "stand_attack_1")]
    StandAttack1,
    /// Walking sequence.
    Walk,
    /// Running sequence.
    Run,
    /// Running stop sequence.
    RunStop,
    /// Dodge while running.
    Dodge,
    /// Character is about to jump.
    Jump,
    /// Character has just jumped off the ground.
    ///
    /// The beginning of this sequence is when the jump velocity is applied. This is separate from
    /// `JumpAscend` as there may be characters that have a separate ascending animation that
    /// repeats while the velocity is still upwards.
    JumpOff,
    /// Character is moving upwards while jumping.
    ///
    /// This is distinct from the `JumpDescend` state as this is when the jump velocity is
    /// effective, and characters may have different animations and attacks when moving upwards from
    /// a jump.
    JumpAscend,
    /// Character is descending from a jump.
    ///
    /// This sequence may also be used when the character has walked off a platform.
    JumpDescend,
    /// Character landed from jumping.
    JumpDescendLand,
    /// Attack while jumping.
    JumpAttack,
    /// Character has dashed off the ground facing forward.
    ///
    /// The beginning of this sequence is when the dash velocity is applied. This is separate from
    /// `DashAscendForward` as there may be characters that have a separate ascending animation that
    /// repeats while the velocity is still upwards.
    DashForward,
    /// Character is moving upwards while dashing facing forward.
    ///
    /// This is distinct from the `DashAscendForward` state as this is when the dash velocity is
    /// effective, and characters may have different animations and attacks when moving upwards from
    /// a dash.
    DashForwardAscend,
    /// Character is descending from a dash facing forward.
    DashForwardDescend,
    /// Character has dashed off the ground facing back.
    ///
    /// The beginning of this sequence is when the dash velocity is applied. This is separate from
    /// `DashForwardAscend` as there may be characters that have a separate ascending animation that
    /// repeats while the velocity is still upwards.
    DashBack,
    /// Character is moving upwards while dashing facing back.
    ///
    /// This is distinct from the `DashBackAscend` state as this is when the dash velocity is
    /// effective, and characters may have different animations and attacks when moving upwards from
    /// a dash.
    DashBackAscend,
    /// Character is descending from a dash facing back.
    DashBackDescend,
    /// Character landed from dashing.
    DashDescendLand,
    /// Attack while dashing.
    DashAttack,
    /// Character is hit while on ground.
    #[serde(rename = "flinch_0")]
    Flinch0,
    /// Character is hit while on ground, alternate sequence.
    #[serde(rename = "flinch_1")]
    Flinch1,
    /// Character is dazed / seeing stars.
    Dazed,
    /// Knocked off balance, moving upwards.
    FallForwardAscend,
    /// Knocked off balance, moving downwards.
    FallForwardDescend,
    /// Knocked off balance, landed on ground (bounce).
    FallForwardLand,
    /// Lying on ground face down.
    LieFaceDown,
}

impl SequenceName for CharacterSequenceId {}
