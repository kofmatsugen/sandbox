use crate::id::{file, pack};
use crate::types::UserData;
use amethyst::ecs::Entity;
use amethyst_sprite_studio::traits::translate_animation::TranslateAnimation;

pub struct FightTranslation;

impl<'s> TranslateAnimation<'s> for FightTranslation {
    type FileId = file::FileId;
    type PackKey = pack::PackKey;
    type AnimationKey = pack::AnimationKey;
    type UserData = UserData;
    type OptionalData = ();

    fn translate_animation(
        _entity: Entity,
        rest_time: f32,
        (&current_pack, &current_anim): (&Self::PackKey, &Self::AnimationKey),
        _user: Option<&Self::UserData>,
        _optional: &Self::OptionalData,
    ) -> Option<(Self::PackKey, Self::AnimationKey, usize)> {
        if rest_time < 0. {
            let next_anim = match current_anim {
                pack::AnimationKey::Stance => pack::AnimationKey::Sit,
                pack::AnimationKey::Sit => pack::AnimationKey::Walk,
                pack::AnimationKey::Walk => pack::AnimationKey::Run,
                pack::AnimationKey::Run => pack::AnimationKey::Defence,
                pack::AnimationKey::Defence => pack::AnimationKey::Dead2,
                pack::AnimationKey::Dead2 => pack::AnimationKey::Dead1,
                pack::AnimationKey::Dead1 => pack::AnimationKey::Kick1,
                pack::AnimationKey::Kick1 => pack::AnimationKey::Kick2,
                pack::AnimationKey::Kick2 => pack::AnimationKey::Punch1,
                pack::AnimationKey::Punch1 => pack::AnimationKey::Punch2,
                pack::AnimationKey::Punch2 => pack::AnimationKey::Sitdown,
                pack::AnimationKey::Sitdown => pack::AnimationKey::Standup,
                pack::AnimationKey::Standup => pack::AnimationKey::Stance,
            };
            log::trace!("default next key: {:?}", (current_pack, next_anim, 0));

            Some((current_pack, next_anim, 0))
        } else {
            None
        }
    }
}
