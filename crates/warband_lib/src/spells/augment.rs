use crate::prelude::*;
use enumflags2::BitFlags;

use super::Tags;

#[derive(Default, Component)]
pub(crate) struct RequiredTags(BitFlags<Tags>);
