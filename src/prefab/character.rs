use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::Transform,
    derive::PrefabData,
    ecs::Entity,
    Error,
};
use fight_game::components::{Direction, PlayerTag, SkillSet};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct CharacterPrefab {
    transform: Transform,
    direction: Direction,
    player_tag: PlayerTag,
    skill_set: SkillSet,
}
