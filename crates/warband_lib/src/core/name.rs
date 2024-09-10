use crate::prelude::*;
use std::borrow::Cow;

// Implements extension methods on `Name` to create prefixed names for different defined game object
// types. Makes it easier to distiguish entities when taking a quick glance at the hierarchy.
// Example: `Name::ui("button_ok")` -> `:ui button_ok`

macro_rules! implement_name_tags {
    ($($tag:tt),*) => {
        pub trait NameTags {
            $(
                fn $tag(name: impl Into<Cow<'static, str>>) -> Name {
                    Name::new(format!(":{} {}", stringify!($tag), name.into()))
                }
            )*
        }

        impl NameTags for Name {}
    };
}

implement_name_tags!(ui, unit, camera, light);
