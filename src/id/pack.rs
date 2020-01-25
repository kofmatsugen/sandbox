#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub enum PackKey {
    Base,
}

impl ToString for PackKey {
    fn to_string(&self) -> String {
        match self {
            PackKey::Base => "sample",
        }
        .to_string()
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub enum AnimationKey {
    Stance,
    Sit,
    Walk,
    Run,
    Defence,
    Dead2,
    Dead1,
    Kick1,
    Kick2,
    Punch1,
    Punch2,
    Sitdown,
    Standup,
}

impl ToString for AnimationKey {
    fn to_string(&self) -> String {
        match self {
            AnimationKey::Stance => "0000_stance",
            AnimationKey::Sit => "0001_sit",
            AnimationKey::Walk => "0002_walk",
            AnimationKey::Run => "0003_run",
            AnimationKey::Defence => "0004_defense",
            AnimationKey::Dead2 => "0005_dead2",
            AnimationKey::Dead1 => "0006_dead1",
            AnimationKey::Kick1 => "0007_kick1",
            AnimationKey::Kick2 => "0008_kick2",
            AnimationKey::Punch1 => "0009_punch1",
            AnimationKey::Punch2 => "0010_punch2",
            AnimationKey::Sitdown => "0011_sitdown",
            AnimationKey::Standup => "0012_standup",
        }
        .to_string()
    }
}
