use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use std::collections::BTreeMap;

lazy_static::lazy_static! {
    static ref FILE_LIST: BTreeMap<FileId, (&'static str, usize)> = {
        let mut list = BTreeMap::new();
        list.insert(FileId::SpriteStudioSplash, ("splash1024", 1));
        list.insert(FileId::Sample, ("sample", 1));
        list
    };
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub enum FileId {
    SpriteStudioSplash,
    Sample,
}

impl AnimationFile for FileId {
    fn to_file_name(&self) -> &'static str {
        FILE_LIST[self].0
    }

    fn sprite_sheet_num(&self) -> usize {
        FILE_LIST[self].1
    }
}
