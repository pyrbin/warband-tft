#![feature(prelude_import)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

mod assets {




    use bevy_asset_loader::{
        asset_collection::AssetCollection,
        loading_state::{config::ConfigureLoadingState, LoadingStateAppExt},
        prelude::LoadingState,
    };
    use crate::{prelude::*, AppState};
    pub fn plugin(app: &mut App) {
        app.register_type::<FontAssets>();
        app.register_type::<ModelAssets>();
        app.register_type::<ImageAssets>();
        ;
        app.add_loading_state(LoadingState::new(AppState::Loading).load_collection::<FontAssets>().load_collection::<ModelAssets>().load_collection::<ImageAssets>().continue_to_state(AppState::InGame));
    }
    #[reflect(Resource)]
    pub struct FontAssets {
        #[asset(path = "fonts/iAWriterQuattroS-Regular.ttf")]
        pub ia_writer_quattro: Handle<Font>,
        #[asset(path = "fonts/CommitMono-400-Regular.otf")]
        pub commit_mono_400: Handle<Font>,
        #[asset(path = "fonts/CommitMono-700-Regular.otf")]
        pub commit_mono_700: Handle<Font>,
    }
    #[automatically_derived]
    #[allow(unused_variables)]
    impl AssetCollection for FontAssets {
        fn create(world: &mut ::bevy::ecs::world::World) -> Self {
            let from_world_fields = ();
            world.resource_scope(|world,
                    asset_keys:
                        ::bevy::prelude::Mut<::bevy_asset_loader::dynamic_asset::DynamicAssets>|
                    {
                        FontAssets {
                            ia_writer_quattro: {
                                let asset_server =
                                    world.get_resource::<::bevy::asset::AssetServer>().expect("Cannot get AssetServer");
                                asset_server.load("fonts/iAWriterQuattroS-Regular.ttf")
                            },
                            commit_mono_400: {
                                let asset_server =
                                    world.get_resource::<::bevy::asset::AssetServer>().expect("Cannot get AssetServer");
                                asset_server.load("fonts/CommitMono-400-Regular.otf")
                            },
                            commit_mono_700: {
                                let asset_server =
                                    world.get_resource::<::bevy::asset::AssetServer>().expect("Cannot get AssetServer");
                                asset_server.load("fonts/CommitMono-700-Regular.otf")
                            },
                        }
                    })
        }
        fn load(world: &mut ::bevy::ecs::world::World)
            -> Vec<::bevy::prelude::UntypedHandle> {
            let mut handles = ::alloc::vec::Vec::new();
            {
                let asset_server =
                    world.get_resource::<::bevy::prelude::AssetServer>().expect("Cannot get AssetServer");
                handles.push(asset_server.load_untyped("fonts/iAWriterQuattroS-Regular.ttf").untyped());
            }
            {
                let asset_server =
                    world.get_resource::<::bevy::prelude::AssetServer>().expect("Cannot get AssetServer");
                handles.push(asset_server.load_untyped("fonts/CommitMono-400-Regular.otf").untyped());
            }
            {
                let asset_server =
                    world.get_resource::<::bevy::prelude::AssetServer>().expect("Cannot get AssetServer");
                handles.push(asset_server.load_untyped("fonts/CommitMono-700-Regular.otf").untyped());
            }
            handles
        }
    }
    impl bevy::ecs::system::Resource for FontAssets where Self: Send + Sync +
        'static {}
    #[automatically_derived]
    impl ::core::default::Default for FontAssets {
        #[inline]
        fn default() -> FontAssets {
            FontAssets {
                ia_writer_quattro: ::core::default::Default::default(),
                commit_mono_400: ::core::default::Default::default(),
                commit_mono_700: ::core::default::Default::default(),
            }
        }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for FontAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<ReflectResource>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {
                    <Handle<Font> as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <Handle<Font> as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <Handle<Font> as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                }
            }
            impl bevy::reflect::Typed for FontAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<Handle<Font>>("ia_writer_quattro").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<Handle<Font>>("commit_mono_400").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<Handle<Font>>("commit_mono_700").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for FontAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str {
                    "warband_lib::assets::FontAssets"
                }
                fn short_type_path() -> &'static str { "FontAssets" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("FontAssets")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::assets".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::assets")
                }
            }
            impl bevy::reflect::Struct for FontAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn field(&self, name: &str)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match name {
                        "ia_writer_quattro" =>
                            ::core::option::Option::Some(&self.ia_writer_quattro),
                        "commit_mono_400" =>
                            ::core::option::Option::Some(&self.commit_mono_400),
                        "commit_mono_700" =>
                            ::core::option::Option::Some(&self.commit_mono_700),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_mut(&mut self, name: &str)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match name {
                        "ia_writer_quattro" =>
                            ::core::option::Option::Some(&mut self.ia_writer_quattro),
                        "commit_mono_400" =>
                            ::core::option::Option::Some(&mut self.commit_mono_400),
                        "commit_mono_700" =>
                            ::core::option::Option::Some(&mut self.commit_mono_700),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index {
                        0usize =>
                            ::core::option::Option::Some(&self.ia_writer_quattro),
                        1usize =>
                            ::core::option::Option::Some(&self.commit_mono_400),
                        2usize =>
                            ::core::option::Option::Some(&self.commit_mono_700),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index {
                        0usize =>
                            ::core::option::Option::Some(&mut self.ia_writer_quattro),
                        1usize =>
                            ::core::option::Option::Some(&mut self.commit_mono_400),
                        2usize =>
                            ::core::option::Option::Some(&mut self.commit_mono_700),
                        _ => ::core::option::Option::None,
                    }
                }
                fn name_at(&self, index: usize)
                    -> ::core::option::Option<&str> {
                    match index {
                        0usize => ::core::option::Option::Some("ia_writer_quattro"),
                        1usize => ::core::option::Option::Some("commit_mono_400"),
                        2usize => ::core::option::Option::Some("commit_mono_700"),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_len(&self) -> usize { 3usize }
                fn iter_fields(&self) -> bevy::reflect::FieldIter {
                    bevy::reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                    let mut dynamic: bevy::reflect::DynamicStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic.insert_boxed("ia_writer_quattro",
                        bevy::reflect::Reflect::clone_value(&self.ia_writer_quattro));
                    dynamic.insert_boxed("commit_mono_400",
                        bevy::reflect::Reflect::clone_value(&self.commit_mono_400));
                    dynamic.insert_boxed("commit_mono_700",
                        bevy::reflect::Reflect::clone_value(&self.commit_mono_700));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for FontAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                {
                                let name =
                                    bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::Struct::field_mut(self, name) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::Struct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::Struct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for FontAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Font>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {
                                    ia_writer_quattro: (||
                                                    <Handle<Font> as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "ia_writer_quattro")?))()?,
                                    commit_mono_400: (||
                                                    <Handle<Font> as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "commit_mono_400")?))()?,
                                    commit_mono_700: (||
                                                    <Handle<Font> as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "commit_mono_700")?))()?,
                                })
                        } else { ::core::option::Option::None }
                }
            }
        };
    #[reflect(Resource)]
    pub struct ModelAssets {
        #[asset(path = "models/frog.glb#Scene0")]
        pub frog: Handle<Scene>,
    }
    #[automatically_derived]
    #[allow(unused_variables)]
    impl AssetCollection for ModelAssets {
        fn create(world: &mut ::bevy::ecs::world::World) -> Self {
            let from_world_fields = ();
            world.resource_scope(|world,
                    asset_keys:
                        ::bevy::prelude::Mut<::bevy_asset_loader::dynamic_asset::DynamicAssets>|
                    {
                        ModelAssets {
                            frog: {
                                let asset_server =
                                    world.get_resource::<::bevy::asset::AssetServer>().expect("Cannot get AssetServer");
                                asset_server.load("models/frog.glb#Scene0")
                            },
                        }
                    })
        }
        fn load(world: &mut ::bevy::ecs::world::World)
            -> Vec<::bevy::prelude::UntypedHandle> {
            let mut handles = ::alloc::vec::Vec::new();
            {
                let asset_server =
                    world.get_resource::<::bevy::prelude::AssetServer>().expect("Cannot get AssetServer");
                handles.push(asset_server.load_untyped("models/frog.glb#Scene0").untyped());
            }
            handles
        }
    }
    impl bevy::ecs::system::Resource for ModelAssets where Self: Send + Sync +
        'static {}
    #[automatically_derived]
    impl ::core::default::Default for ModelAssets {
        #[inline]
        fn default() -> ModelAssets {
            ModelAssets { frog: ::core::default::Default::default() }
        }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for ModelAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Scene>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<ReflectResource>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {
                    <Handle<Scene> as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                }
            }
            impl bevy::reflect::Typed for ModelAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Scene>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<Handle<Scene>>("frog").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for ModelAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str {
                    "warband_lib::assets::ModelAssets"
                }
                fn short_type_path() -> &'static str { "ModelAssets" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("ModelAssets")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::assets".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::assets")
                }
            }
            impl bevy::reflect::Struct for ModelAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Scene>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn field(&self, name: &str)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match name {
                        "frog" => ::core::option::Option::Some(&self.frog),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_mut(&mut self, name: &str)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match name {
                        "frog" => ::core::option::Option::Some(&mut self.frog),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&self.frog),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&mut self.frog),
                        _ => ::core::option::Option::None,
                    }
                }
                fn name_at(&self, index: usize)
                    -> ::core::option::Option<&str> {
                    match index {
                        0usize => ::core::option::Option::Some("frog"),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_len(&self) -> usize { 1usize }
                fn iter_fields(&self) -> bevy::reflect::FieldIter {
                    bevy::reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                    let mut dynamic: bevy::reflect::DynamicStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic.insert_boxed("frog",
                        bevy::reflect::Reflect::clone_value(&self.frog));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for ModelAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Scene>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                {
                                let name =
                                    bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::Struct::field_mut(self, name) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::Struct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::Struct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for ModelAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Scene>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {
                                    frog: (||
                                                    <Handle<Scene> as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "frog")?))()?,
                                })
                        } else { ::core::option::Option::None }
                }
            }
        };
    #[reflect(Resource)]
    pub struct ImageAssets {
        #[asset(path = "images/bevy.png")]
        pub bevy: Handle<Image>,
        #[asset(path = "images/proto_dark.png")]
        pub proto_dark: Handle<Image>,
    }
    #[automatically_derived]
    #[allow(unused_variables)]
    impl AssetCollection for ImageAssets {
        fn create(world: &mut ::bevy::ecs::world::World) -> Self {
            let from_world_fields = ();
            world.resource_scope(|world,
                    asset_keys:
                        ::bevy::prelude::Mut<::bevy_asset_loader::dynamic_asset::DynamicAssets>|
                    {
                        ImageAssets {
                            bevy: {
                                let asset_server =
                                    world.get_resource::<::bevy::asset::AssetServer>().expect("Cannot get AssetServer");
                                asset_server.load("images/bevy.png")
                            },
                            proto_dark: {
                                let asset_server =
                                    world.get_resource::<::bevy::asset::AssetServer>().expect("Cannot get AssetServer");
                                asset_server.load("images/proto_dark.png")
                            },
                        }
                    })
        }
        fn load(world: &mut ::bevy::ecs::world::World)
            -> Vec<::bevy::prelude::UntypedHandle> {
            let mut handles = ::alloc::vec::Vec::new();
            {
                let asset_server =
                    world.get_resource::<::bevy::prelude::AssetServer>().expect("Cannot get AssetServer");
                handles.push(asset_server.load_untyped("images/bevy.png").untyped());
            }
            {
                let asset_server =
                    world.get_resource::<::bevy::prelude::AssetServer>().expect("Cannot get AssetServer");
                handles.push(asset_server.load_untyped("images/proto_dark.png").untyped());
            }
            handles
        }
    }
    impl bevy::ecs::system::Resource for ImageAssets where Self: Send + Sync +
        'static {}
    #[automatically_derived]
    impl ::core::default::Default for ImageAssets {
        #[inline]
        fn default() -> ImageAssets {
            ImageAssets {
                bevy: ::core::default::Default::default(),
                proto_dark: ::core::default::Default::default(),
            }
        }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for ImageAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Image>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Image>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<ReflectResource>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {
                    <Handle<Image> as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <Handle<Image> as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                }
            }
            impl bevy::reflect::Typed for ImageAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Image>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Image>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<Handle<Image>>("bevy").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<Handle<Image>>("proto_dark").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for ImageAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str {
                    "warband_lib::assets::ImageAssets"
                }
                fn short_type_path() -> &'static str { "ImageAssets" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("ImageAssets")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::assets".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::assets")
                }
            }
            impl bevy::reflect::Struct for ImageAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Image>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Image>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn field(&self, name: &str)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match name {
                        "bevy" => ::core::option::Option::Some(&self.bevy),
                        "proto_dark" =>
                            ::core::option::Option::Some(&self.proto_dark),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_mut(&mut self, name: &str)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match name {
                        "bevy" => ::core::option::Option::Some(&mut self.bevy),
                        "proto_dark" =>
                            ::core::option::Option::Some(&mut self.proto_dark),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&self.bevy),
                        1usize => ::core::option::Option::Some(&self.proto_dark),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&mut self.bevy),
                        1usize =>
                            ::core::option::Option::Some(&mut self.proto_dark),
                        _ => ::core::option::Option::None,
                    }
                }
                fn name_at(&self, index: usize)
                    -> ::core::option::Option<&str> {
                    match index {
                        0usize => ::core::option::Option::Some("bevy"),
                        1usize => ::core::option::Option::Some("proto_dark"),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_len(&self) -> usize { 2usize }
                fn iter_fields(&self) -> bevy::reflect::FieldIter {
                    bevy::reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                    let mut dynamic: bevy::reflect::DynamicStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic.insert_boxed("bevy",
                        bevy::reflect::Reflect::clone_value(&self.bevy));
                    dynamic.insert_boxed("proto_dark",
                        bevy::reflect::Reflect::clone_value(&self.proto_dark));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for ImageAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Image>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Image>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                {
                                let name =
                                    bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::Struct::field_mut(self, name) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::Struct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::Struct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for ImageAssets where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                Handle<Image>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Handle<Image>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {
                                    bevy: (||
                                                    <Handle<Image> as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "bevy")?))()?,
                                    proto_dark: (||
                                                    <Handle<Image> as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "proto_dark")?))()?,
                                })
                        } else { ::core::option::Option::None }
                }
            }
        };
}
mod in_game {
    use crate::{
        board, navigation::{self, agent::Agent},
        player::camera::MainCamera, prelude::*, stats::stat, AppState,
    };
    pub fn plugin(app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(Update,
            test_target.run_if(in_state(AppState::InGame)));
        app.add_plugins(stat::plugin::<Health>);
    }
    struct MoveTo;
    impl bevy::ecs::component::Component for MoveTo where Self: Send + Sync +
        'static {
        const STORAGE_TYPE: bevy::ecs::component::StorageType =
            bevy::ecs::component::StorageType::Table;
    }
    fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>) {
        commands.spawn((Name::light("sun"),
                DirectionalLightBundle {
                    directional_light: DirectionalLight {
                        illuminance: 5000.0,
                        color: Color::WHITE,
                        ..default()
                    },
                    transform: Transform::from_xyz(30., 100.,
                            30.).looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                }, Cleanup(OnExit(AppState::InGame))));
        let cube_mesh = meshes.add(Cuboid::default());
        commands.spawn((Name::unit("floor"),
                SpatialBundle {
                    transform: Transform::from_translation(Vec3::Y * -0.05),
                    ..default()
                }, RigidBody::Static, Collider::cuboid(100.0, 0.1, 100.0)));
        let id =
            commands.spawn((Name::unit("Target"),
                        PbrBundle {
                            mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                            material: materials.add(Color::srgb_u8(124, 144,
                                        255).with_alpha(0.1)),
                            transform: Transform::from_xyz(0.0, 0.5, 0.0),
                            ..default()
                        }, Agent::default(), board::Location::default(),
                        board::Footprint::default(), MoveTo)).id();
        let test_id =
            commands.spawn((Name::unit("Test"),
                        PbrBundle {
                            mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                            material: materials.add(Color::srgb_u8(2, 144, 255)),
                            transform: Transform::from_xyz(4.0, 0.5, 4.0),
                            ..default()
                        }, Agent::default(), board::Location::default(),
                        board::Footprint::default(),
                        navigation::path::Target::Entity(id))).id();
        commands.spawn((Name::unit("obstacle"),
                SpatialBundle {
                    transform: Transform::from_xyz(-2.0, 2.0, -2.0),
                    ..default()
                }, RigidBody::Dynamic, Collider::cuboid(4.0, 4.0, 4.0),
                board::Location::default(), board::Footprint::default(),
                Health::new(200.0)));
    }
    fn test_target(buttons: Res<ButtonInput<MouseButton>>,
        windows: Query<&Window, With<PrimaryWindow>>,
        cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        board: Res<board::Board>,
        mut target: Query<&mut Transform, With<MoveTo>>) {
        let window: &Window =
            match windows.get_single() { Ok(q) => q, _ => { return; } };
        let (camera, cam_transform) =
            match cameras.get_single() { Ok(q) => q, _ => { return; } };
        let mut target =
            match target.get_single_mut() { Ok(q) => q, _ => { return; } };
        let Some(ray) =
            window.cursor_position().and_then(|p|
                    camera.viewport_to_world(cam_transform,
                        p)) else { return; };
        if !buttons.just_pressed(MouseButton::Left) { return; }
        let Some(distance) =
            ray.intersect_plane(Vec3::ZERO,
                InfinitePlane3d::new(Dir3::Y)) else { return; };
        let point = ray.origin + ray.direction * distance;
        let hex = board.layout.world_pos_to_hex(point.xz());
        target.translation = board.layout.hex_to_world_pos(hex).x0y();
    }
    #[clamp(clamp_0_100)]
    struct Health(f32);
    impl crate::stats::stat::Stat for Health {
        fn new(value: f32) -> Self { Self { 0: value, ..Default::default() } }
        fn value(&self) -> f32 { self.0 }
        fn clamp(value: f32) -> f32 { clamp_0_100(value) }
        fn round(value: f32) -> f32 { (value) }
    }
    impl Into<Health> for f32 {
        fn into(self) -> Health { Health::new(self) }
    }
    impl std::ops::Deref for Health {
        type Target = f32;
        fn deref(&self) -> &f32 { &self.0 }
    }
    #[automatically_derived]
    impl ::core::default::Default for Health {
        #[inline]
        fn default() -> Health { Health(::core::default::Default::default()) }
    }
    impl bevy::ecs::component::Component for Health where Self: Send + Sync +
        'static {
        const STORAGE_TYPE: bevy::ecs::component::StorageType =
            bevy::ecs::component::StorageType::Table;
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for Health where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {
                    <f32 as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                }
            }
            impl bevy::reflect::Typed for Health where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[bevy::reflect::UnnamedField::new::<f32>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for Health where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str {
                    "warband_lib::in_game::Health"
                }
                fn short_type_path() -> &'static str { "Health" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("Health")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::in_game".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::in_game")
                }
            }
            impl bevy::reflect::TupleStruct for Health where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn field(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&self.0),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&mut self.0),
                        _ => ::core::option::Option::None,
                    }
                }
                #[inline]
                fn field_len(&self) -> usize { 1usize }
                #[inline]
                fn iter_fields(&self) -> bevy::reflect::TupleStructFieldIter {
                    bevy::reflect::TupleStructFieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicTupleStruct {
                    let mut dynamic: bevy::reflect::DynamicTupleStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic.insert_boxed(bevy::reflect::Reflect::clone_value(&self.0));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for Health where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                = bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                {
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::TupleStruct::field_mut(self, i) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::TupleStruct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::TupleStruct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::TupleStruct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::TupleStruct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::tuple_struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for Health where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                = bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {
                                    0: (||
                                                    <f32 as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::TupleStruct::field(__ref_struct,
                                                                0)?))()?,
                                })
                        } else { ::core::option::Option::None }
                }
            }
        };
    #[automatically_derived]
    impl ::core::marker::Copy for Health { }
    #[automatically_derived]
    impl ::core::clone::Clone for Health {
        #[inline]
        fn clone(&self) -> Health {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    fn clamp_0_100(value: f32) -> f32 { value.clamp(0.0, 100.0) }
}
mod prelude {
    #![allow(unused_imports)]
    pub(crate) use crate::{
        cleanup::*, core::*, despawn::*, name::*, previous::*,
        required_component::*, single, single_mut, stats::stat::Stat,
        util::{math_ext::*, *},
    };
    pub(crate) use anyhow::{
        anyhow, bail, ensure, Context, Error as AnyError, Result as AnyResult,
    };
    pub(crate) use avian3d::prelude::*;
    pub(crate) use bevy::{
        color::palettes::css::{
            AQUA, LIMEGREEN, ORANGE, RED, VIOLET, WHITE, YELLOW,
        },
        ecs::{query::QueryData, schedule::ScheduleLabel},
        log::*, math::{Vec3Swizzles, *},
        prelude::*, reflect::{GetTypeRegistration, TypePath},
        render::{
            mesh::Indices, render_asset::RenderAssetUsages,
            render_resource::PrimitiveTopology,
        },
        utils::{Duration, HashMap, HashSet, Instant},
        window::PrimaryWindow,
    };
    pub(crate) use bon::{bon, builder, Builder};
    pub(crate) use derive_more::{Display, From};
    pub(crate) use itertools::Itertools;
    pub(crate) use rand::prelude::*;
    pub(crate) use smallvec::SmallVec;
    pub(crate) use std::{
        default, f32::consts::PI, marker::PhantomData, sync::Arc,
    };
    pub(crate) use thiserror::Error;
    pub(crate) use tiny_bail::prelude::*;
    pub(crate) use warband_macros::*;
}
mod util {
    pub mod math_ext {
        use avian3d::parry::na::SimdPartialOrd;
        use crate::prelude::*;
        pub(crate) trait Vec3Ext: Copy {
            fn is_approx_zero(self)
            -> bool;
            fn horizontal(self)
            -> Vec3;
        }
        impl Vec3Ext for Vec3 {
            #[inline]
            fn is_approx_zero(self) -> bool { self.length_squared() < 1e-5 }
            #[inline]
            fn horizontal(self) -> Vec3 { Vec3::new(self.x, 0., self.z) }
        }
        pub(crate) trait Vec2Ext: Copy {
            fn is_approx_zero(self)
            -> bool;
            fn x0y(self)
            -> Vec3;
            fn x_y(self, y: f32)
            -> Vec3;
        }
        impl Vec2Ext for Vec2 {
            #[inline]
            fn is_approx_zero(self) -> bool { self.length_squared() < 1e-5 }
            #[inline]
            fn x0y(self) -> Vec3 { Vec3::new(self.x, 0., self.y) }
            #[inline]
            fn x_y(self, y: f32) -> Vec3 { Vec3::new(self.x, y, self.y) }
        }
        pub(crate) trait F32Ext: Copy {
            fn squared(self)
            -> f32;
        }
        impl F32Ext for f32 {
            #[inline]
            fn squared(self) -> f32 { self * self }
        }
        pub(crate) trait Clamp01 {
            fn clamp01(self)
            -> Self;
        }
        impl Clamp01 for f32 {
            #[inline]
            fn clamp01(self) -> Self { self.simd_clamp(0.0, 1.0) }
        }
        impl Clamp01 for Vec2 {
            #[inline]
            fn clamp01(self) -> Self { self.clamp(Vec2::ZERO, Vec2::ONE) }
        }
        impl Clamp01 for Vec3 {
            #[inline]
            fn clamp01(self) -> Self { self.clamp(Vec3::ZERO, Vec3::ONE) }
        }
        impl Clamp01 for Vec4 {
            #[inline]
            fn clamp01(self) -> Self { self.clamp(Vec4::ZERO, Vec4::ONE) }
        }
        pub(crate) trait IntoTransform {
            /// Convert this type into a `Transform`.
            fn into_transform(self)
            -> Transform;
        }
        impl IntoTransform for Vec3 {
            #[inline]
            fn into_transform(self) -> Transform {
                Transform::from_xyz(self.x, self.y, self.z)
            }
        }
        impl IntoTransform for Quat {
            #[inline]
            fn into_transform(self) -> Transform {
                Transform::from_rotation(self)
            }
        }
        pub(crate) trait LerpRadius {
            /// Linearly interpolate between two values, but if the distance between them is less than the
            /// radius, return the other value.
            fn lerp_radius(self, other: Self, t: f32, radius: f32)
            -> Self;
        }
        impl LerpRadius for f32 {
            #[inline]
            fn lerp_radius(self, other: Self, t: f32, radius: f32) -> Self {
                let mut result =
                    bevy::prelude::FloatExt::lerp(self, other, t);
                if (result - other).abs() < radius { result = other; }
                result
            }
        }
        impl LerpRadius for Vec3 {
            #[inline]
            fn lerp_radius(self, other: Self, t: f32, radius: f32) -> Self {
                let mut result = self.lerp(other, t);
                if (result - other).length_squared() < radius {
                        result = other;
                    }
                result
            }
        }
        impl LerpRadius for Quat {
            #[inline]
            fn lerp_radius(self, other: Self, t: f32, radius: f32) -> Self {
                let mut result = self.lerp(other, t);
                if (result - other).length_squared() < radius {
                        result = other;
                    }
                result
            }
        }
        pub trait Reset: Default {
            fn reset(&mut self);
        }
        impl<T: Default> Reset for T {
            #[inline]
            fn reset(&mut self) { *self = T::default(); }
        }
        pub trait Zero: PartialEq + Sized {
            const ZERO: Self;
            fn is_zero(&self) -> bool { *self == Self::ZERO }
        }
        impl Zero for Vec3 {
            const ZERO: Self = Self::ZERO;
        }
        impl Zero for f32 {
            const ZERO: Self = 0.0;
        }
    }
    pub mod pipe {
        use crate::prelude::*;
        pub(crate) fn error(In(result): In<Result<(), AnyError>>) {
            if let Err(err) = result {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite =
                            {
                                static META: ::tracing::Metadata<'static> =
                                    {
                                        ::tracing_core::metadata::Metadata::new("event crates\\warband_lib\\src\\util\\pipe.rs:5",
                                            "warband_lib::util::pipe", ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("crates\\warband_lib\\src\\util\\pipe.rs"),
                                            ::core::option::Option::Some(5u32),
                                            ::core::option::Option::Some("warband_lib::util::pipe"),
                                            ::tracing_core::field::FieldSet::new(&["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE)),
                                            ::tracing::metadata::Kind::EVENT)
                                    };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                        let enabled =
                            ::tracing::Level::ERROR <=
                                        ::tracing::level_filters::STATIC_MAX_LEVEL &&
                                    ::tracing::Level::ERROR <=
                                        ::tracing::level_filters::LevelFilter::current() &&
                                {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never() &&
                                        ::tracing::__macro_support::__is_enabled(__CALLSITE.metadata(),
                                            interest)
                                };
                        if enabled {
                                (|value_set: ::tracing::field::ValueSet|
                                            {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::ERROR {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            } <= ::tracing::log::STATIC_MAX_LEVEL {
                                                        if !::tracing::dispatcher::has_been_set() {
                                                                {
                                                                    use ::tracing::log;
                                                                    let level =
                                                                        match ::tracing::Level::ERROR {
                                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                            _ => ::tracing::log::Level::Trace,
                                                                        };
                                                                    if level <= log::max_level() {
                                                                            let meta = __CALLSITE.metadata();
                                                                            let log_meta =
                                                                                log::Metadata::builder().level(level).target(meta.target()).build();
                                                                            let logger = log::logger();
                                                                            if logger.enabled(&log_meta) {
                                                                                    ::tracing::__macro_support::__tracing_log(meta, logger,
                                                                                        log_meta, &value_set)
                                                                                }
                                                                        }
                                                                }
                                                            } else { {} }
                                                    } else { {} };
                                            })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE.metadata().fields().value_set(&[(&::core::iter::Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&format_args!("Error: {0:?}",
                                                                            err) as &dyn Value))])
                                    });
                            } else {
                               if match ::tracing::Level::ERROR {
                                               ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                               ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                               ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                               ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                               _ => ::tracing::log::Level::Trace,
                                           } <= ::tracing::log::STATIC_MAX_LEVEL {
                                       if !::tracing::dispatcher::has_been_set() {
                                               {
                                                   use ::tracing::log;
                                                   let level =
                                                       match ::tracing::Level::ERROR {
                                                           ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                           ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                           ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                           ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                           _ => ::tracing::log::Level::Trace,
                                                       };
                                                   if level <= log::max_level() {
                                                           let meta = __CALLSITE.metadata();
                                                           let log_meta =
                                                               log::Metadata::builder().level(level).target(meta.target()).build();
                                                           let logger = log::logger();
                                                           if logger.enabled(&log_meta) {
                                                                   ::tracing::__macro_support::__tracing_log(meta, logger,
                                                                       log_meta,
                                                                       &{
                                                                               #[allow(unused_imports)]
                                                                               use ::tracing::field::{debug, display, Value};
                                                                               let mut iter = __CALLSITE.metadata().fields().iter();
                                                                               __CALLSITE.metadata().fields().value_set(&[(&::core::iter::Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                                                                                                   ::core::option::Option::Some(&format_args!("Error: {0:?}",
                                                                                                                   err) as &dyn Value))])
                                                                           })
                                                               }
                                                       }
                                               }
                                           } else { {} }
                                   } else { {} };
                           }
                    }
                }
        }
    }
    #[macro_export]
    macro_rules! single {
        ($query:expr) =>
        { match $query.get_single() { Ok(q) => q, _ => { return; } } };
    }
    #[macro_export]
    macro_rules! single_mut {
        ($query:expr) =>
        { match $query.get_single_mut() { Ok(q) => q, _ => { return; } } };
    }
}
mod version {
    pub struct Semver {
        pub major: u16,
        pub minor: u16,
        pub patch: u16,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Semver {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(f, "Semver",
                "major", &self.major, "minor", &self.minor, "patch",
                &&self.patch)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Semver { }
    #[automatically_derived]
    impl ::core::clone::Clone for Semver {
        #[inline]
        fn clone(&self) -> Semver {
            let _: ::core::clone::AssertParamIsClone<u16>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Semver {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.major, state);
            ::core::hash::Hash::hash(&self.minor, state);
            ::core::hash::Hash::hash(&self.patch, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Semver {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u16>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Semver { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Semver {
        #[inline]
        fn eq(&self, other: &Semver) -> bool {
            self.major == other.major && self.minor == other.minor &&
                self.patch == other.patch
        }
    }
    impl std::fmt::Display for Semver {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{0}.{1}.{2}", self.major, self.minor,
                    self.patch))
        }
    }
    pub const GIT_VERSION: &str =
        {
            b"0000000000000000000000000000000000000000 e941e10b40cbb4c0f23f5cc19d6aaabf3c45132a frans <31902695+pyrbin@users.noreply.github.com> 1724360522 +0200\tcommit (initial): initial commit \xf0\x9f\x90\xb8\ne941e10b40cbb4c0f23f5cc19d6aaabf3c45132a 0000000000000000000000000000000000000000 frans <31902695+pyrbin@users.noreply.github.com> 1724360530 +0200\tBranch: renamed refs/heads/master to refs/heads/main\n0000000000000000000000000000000000000000 e941e10b40cbb4c0f23f5cc19d6aaabf3c45132a frans <31902695+pyrbin@users.noreply.github.com> 1724360530 +0200\tBranch: renamed refs/heads/master to refs/heads/main\ne941e10b40cbb4c0f23f5cc19d6aaabf3c45132a 77ef9c039932b9d865ead37f1a48563167a84f37 frans <31902695+pyrbin@users.noreply.github.com> 1725491934 +0200\tcommit: enabled git version & use latest hexx\n77ef9c039932b9d865ead37f1a48563167a84f37 904f76038d1d83604a01ebff613cbe379c5e4a0a frans <31902695+pyrbin@users.noreply.github.com> 1725929543 +0200\tcommit: initial board impl. & added common stuff from motte\n904f76038d1d83604a01ebff613cbe379c5e4a0a 1996c7142dca92e64bdabf03a7ddeacabffa02cd frans <31902695+pyrbin@users.noreply.github.com> 1725974325 +0200\tcommit: use schedule input in cleanup component\n1996c7142dca92e64bdabf03a7ddeacabffa02cd d6e2b7716241714b25ab4564067d75cc3b70cc81 frans <31902695+pyrbin@users.noreply.github.com> 1726271425 +0200\tcommit: added dev console + perf ui\nd6e2b7716241714b25ab4564067d75cc3b70cc81 cdf24c68ecbf50727ef5aecd5b5e7ab6ac3126c6 frans <31902695+pyrbin@users.noreply.github.com> 1726271442 +0200\tcommit: working on board footprints & pathfinding\ncdf24c68ecbf50727ef5aecd5b5e7ab6ac3126c6 cdcf4d074167fe0e240e7a483db703ad865bed32 frans <31902695+pyrbin@users.noreply.github.com> 1726358534 +0200\tcommit: wip: board navigation & obstacles\ncdcf4d074167fe0e240e7a483db703ad865bed32 45ff32f88bb790f21694500bc90e7df44441d135 frans <31902695+pyrbin@users.noreply.github.com> 1726522846 +0200\tcommit: pathfinding work & wip integrating motte stat system\n45ff32f88bb790f21694500bc90e7df44441d135 5cd006dcbac14384abcd4e82b369bdcaf32036ca frans <31902695+pyrbin@users.noreply.github.com> 1726611294 +0200\tcommit: refactor stat system to make it more simple (missing derive macro & pool)\n";
            b"DIRC\x00\x00\x00\x02\x00\x00\x00Ff\xc6G@&\xff\x10\x10f\xc6Gu\x0c\xc1\r\x84\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\xe3n\xde3x\xe1HL\x03\xeb (\xce\xccN=E\xdc\xeaH&\x00\x12.cargo/config.toml\x00\x00\x00\x00\x00\x00\x00\x00f\xc6D\xa1\x00\xe4\x13\x80f\xc6D\xa2\x0b\x95 \xa8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1f4\x19\x98\x07\xc3\x96\x9c\xd6\xcbJ,\x8e2\x9b\xeb(\xb0\xf3K\xfb\x00\r.editorconfig\x00\x00\x00\x00\x00f\xc6#\xed2\xf2v\xb4f\xdf\x97\xf8\x0bE)\xf4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xe9}\xa3\x87\x03\x96\x84\xd3\x17\xe6p+\xc3\xe8\xd4\x13\x0b\x9cl\xbc:\x00\x0e.gitattributes\x00\x00\x00\x00f\xc6#\xed3N-8f\x00.\x9e\x07\x07}$\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x06F\xfb\xfdP\xf1l\xb3\xc8\xc0\xa1xs\x14@\x07\xbf\x00,\xd0\xee\x00\n.gitignore\x00\x00\x00\x00\x00\x00\x00\x00f\xc7\x98v\x05\xce\xd3Xf\xc7\x9d\xbb\x1d\xec\xad\x98\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00F\x12>\xea\xb0\x1f\xfa5\x0e\xf3\xb8t\xcc\xecR\xe5$\nC\x11\x00\x00\x17.vscode/extensions.json\x00\x00\x00f\xc7\x98v\x05\xdd<\xccf\xc7\x98\x8c\x07\x8e\xab\xc8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\xbd\xf3\xc0\xc0\xe6\x86\x8b\xc0\xcck\x19O$m=\x7fTl\x83\x19I\x00\x12.vscode/tasks.json\x00\x00\x00\x00\x00\x00\x00\x00f\xc6#\xed4i\x17df\xdf\x97\x0c\x01\x07s\\\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\x83\x0c1^\xa2\xd0\x8f\x0be\xa1)\x1a\xb7h;\x1cHrw\x17\xb5\x00\nCargo.toml\x00\x00\x00\x00\x00\x00\x00\x00f\xc6#\x97\x16\xc7`\x08e\x0cgJ\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x0e\x88\xf7\x00\xb7\xd0H\xf8\x1e2h@2\xfb\t\\\xc0,\\\xd2\xa8B\x00\'assets/fonts/CommitMono-400-Regular.otf\x00\x00\x00f\xc6#\x97\x16\xd6\xa1\xe4e\x0cgK6\x89\xca\xc0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x14\x0cq\xcc\xc4\xa3\x07ru\x17\x95\xceR\xef\x0e\x82=\xe8\xfcq\xc9\x16\x00\'assets/fonts/CommitMono-700-Regular.otf\x00\x00\x00f\xc6#\x97\x16\xee\x83,e\x07]c\x0f\xd46\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\xd3\xdc\xd6\xd2\xd9\xadX_p\x04\xdb^\xb7\xb0\x08\xbe\xc3z1\xc4\x7f\xec\x00)assets/fonts/iAWriterQuattroS-Regular.ttf\x00f\xc6#\x97\x17RQ,e\t\xf6P)\x85\xa8\xf8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1cx\xb6\xc3\xf22\xb5\xc4\x83\x81\xed\x90w\xa4\x83\x17\x08\x9e\x00r\x86~\x00\x16assets/images/bevy.png\x00\x00\x00\x00f\xc6#\x97\x17a\xa1\x18^\x8d\x17$\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02}\x9dv\xfeB\xf8J\x8e7-\x0b\x1a\n\xef\x14\xc77\xcb\x9f?@\x00\x1cassets/images/proto_dark.png\x00\x00\x00\x00\x00\x00f\xc6#\x97\x17\x1c\xd4\x98e5\x01z\x02\xfbT\x1c\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01{<iZ\\\xd7\xe6~9d\x1f\xc2\xce\xe9\xfd>\xa4\x1d\x1f\x7f\xc8>\x00\x16assets/models/frog.glb\x00\x00\x00\x00f\xc6#\x97\x17p\xe3Xf\xc7\x9d\xa99\xfd0\xc0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07]6q\xf7@\x0e4?v}@\x87g\xde\xe7\xc2\xbah\xe0cV\x00\x19assets/shaders/color.wgsl\x00f\xc6#\x97\x17\x87\xd2\xd4f\xc7\x9d\xa1\x19i\x1b\xd0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02|\x18I\xbf\'\x94\x98b\xb8\xa2\x08\x93\xf3\xd4\x03\xdd&.?]\xe4\x00\x18assets/shaders/core.wgsl\x00\x00f\xc6B%\n\xbf\x85\x10e\x10\xc05)\x02q,\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02c\x0f\x8cz\xfb\xf5%`\x85=\xa7\x08\x1eER:2(\xe9\xf1E7\x00\x12build/win/icon.ico\x00\x00\x00\x00\x00\x00\x00\x00f\xc6B%\n\xbf\x85\x10e\x10\xc02\x14\xa1z\xfc\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x18%l\xb7\x95Vh\x11W\x83U\xd5\xae\xd4\xe3\xb0\x80MOig\x00\x11build/win/icon.rc\x00f\xdf\x96\xf6\x16\x7f3xf\xdf\x96\xf70;U0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00F`\xbd<\x9aOd\xa5\xc3\x9a(\xae|\x1f\xad\xb2j\xc8\xc6c\xfa\x00\x0bclippy.toml\x00\x00\x00\x00\x00\x00\x00f\xc6A\xb3*\x06@\x1cf\xdf\x92\x883XZ\xb0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05\xf51\x87\xfe5Bk\x10]\x99*4p\xa9`\xb2\xc4\xee\x82/\xf3\x00\x19crates/warband/Cargo.toml\x00f\xc6B\x1b6;\r\xb8f\xc6O\x94\x13Dat\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xcb\x1e\x15ty=\x0f\xca\x1dCA\xc4}\xab\xe0I\\\xf9\xcd\x06>\x00\x17crates/warband/build.rs\x00\x00\x00f\xc6B\x982\x9e\xf3hf\xe4\xb7\x97#_\x9d\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\n\x1cS8P\xae\xb4\x19\xfb,E\xab\x03\xe8\xc5-\x05\xaa6\xbd\xdf\x9f\x00\x1acrates/warband/src/main.rs\x00\x00\x00\x00\x00\x00\x00\x00f\xc6=!\x0c6\xa8\x9cf\xe5\xc8\xb74\'iT\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08N}\x17\"M\x84\xb8\xf7\n\xc9\x19/i\x93\xf4\xe7\x17\xab%:L\x00\x1dcrates/warband_lib/Cargo.toml\x00\x00\x00\x00\x00f\xc7\x9a\xaa*\xd7\x82lf\xdf\x92c\x07\xa0\x0b\xd4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05p\x19\x02\xd8\x04T\xef\x05\x1dG\x84\x92c\x84ey)\xfc\xb5Dq\x00$crates/warband_lib/src/assets/mod.rs\x00\x00\x00\x00\x00\x00f\xe4\x88r\x10\x1a^\xf4f\xe8!\xcb\x05\xfc\x05\xa8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\xca:G\xea\x93\xfd\xd6\'0\xa54\xa3@u\xbf\xc7[\x17\x9a\xf1P\x00)crates/warband_lib/src/board/footprint.rs\x00f\xdf(\xa6\x05\x11{\x14f\xe6 \xaf1\xdc\x9d\xcc\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07.o\x1fh(\xa5\xedTS\xe4\x91o~\xedb\x89\x0f\xec\xce\xee\xd8\x00(crates/warband_lib/src/board/location.rs\x00\x00f\xda\x12\\\x11+\xcbpf\xe83\x07(\xd4-\xd0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1f;\xe8\x91\xdb x\xbez5W\xf7\xec\x8d\xc47\x1d\x9cg\x9b\x9eJ\x00#crates/warband_lib/src/board/mod.rs\x00\x00\x00\x00\x00\x00\x00f\xe4\x8a\x8f*\xdaZ\x8cf\xe6 \x05!\xd2\xcfx\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00 \xec\xeb\xfe\x0b\x08C\xcc\x96\x88hgl9j\x14\xae$i\xf6,\xfc\x00(crates/warband_lib/src/board/occupied.rs\x00\x00f\xdf\x8b\x16&[f\x04f\xdf\x97\xa0\x14\xf4\xd5\xa8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1b\xc5\xf5-4\xc4BtKkG\x9bj\xda\xcaq\xb5\xda\x1f\x84\xf1=\x00,crates/warband_lib/src/core/camera_driver.rs\x00\x00\x00\x00\x00\x00f\xdf\x89o\x05\x98\x91\x80f\xe6 \x8c\x13ds\x9c\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\xda~O\xd6\x0e\x82\x8c\xa4Z\xac\x04\'%J\xe8\x9dl\x8c\xd1[b\x00&crates/warband_lib/src/core/cleanup.rs\x00\x00\x00\x00f\xdf\x89\xb9 \xfd\x1b\xa8f\xe83\x0e)\xaa\xda\x10\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07Q\xd7\x0f\x85R\x9e~\xbcM[W\xd8\xcb\xa7xH\xd6\xe2\xbe\xbc\xbb\x00&crates/warband_lib/src/core/despawn.rs\x00\x00\x00\x00f\xdf\x86\'-\x11\xa5\xe4f\xe6#\xb8\x00[\xf9P\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05\x01$\x1e@\x97\x10\xc3i\xe2!$Py\x18U\xaa\xd1\xc38\x8b\x0e\x00\"crates/warband_lib/src/core/mod.rs\x00\x00\x00\x00\x00\x00\x00\x00f\xdf\x86\xb4\x0f\xf7\\\xd8f\xdf\x97\x9f\x01{=(\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\xad\x8d\xb7\xf0=PC\xb3\x08\xb2l\xbd\xfa\x83\xf7\xd0-\x1d&\x14\xdd\x00#crates/warband_lib/src/core/name.rs\x00\x00\x00\x00\x00\x00\x00f\xdf\x8a%7\xf1M f\xdf\x97\x9e\x1f>\xcb\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\xcb\xdexL\xe6\x04\xe4\xd4Uo\xbel\x86\x89\xf0m\xf7\x93b,O\x00\'crates/warband_lib/src/core/previous.rs\x00\x00\x00f\xe6!|0>^\xecf\xe6#\t\x1f\x7f}\xe8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\xf66\xe0\x16{\x89\xf5\x9a\\u\\n\xbftD\x08A\xb7\xd8\xbd&\x001crates/warband_lib/src/core/required_component.rs\x00f\xe4\xb7\xe9!\xad>Hf\xe4\xb9F0\x0e\xb2<\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x9c|~\xa2\nI\xfdM\x9a\xe9,w\xa6\x7f\xf8\xfb\xad\xf9\xf8\x90)\x00%crates/warband_lib/src/dev/console.rs\x00\x00\x00\x00\x00f\xe4\x98\xa8%\xcav\x1cf\xe4\xb9\x16\x1c\xdd\xa9\x84\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06\x92Y:\x8c5{\'\xd8\x97\xe1D$\x99Q\x8b\x97\xdc\xef\xec?j\x00(crates/warband_lib/src/dev/gizmos_ext.rs\x00\x00f\xc6Q\x053\xdd\xb4\xf8f\xe5\xc5\xdb*\x0f\x95,\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\n\xfd;;\x1f)\xab_\x9aI\xce\xa3\xa7|\xccu\x8fd\xe2OJ\x96\x00!crates/warband_lib/src/dev/mod.rs\x00f\xe4\xa6\xcb;\x96\x88(f\xe4\xa8\xe7\x08[\x1c\x1c\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\xca\xa4T^N\xb6\x95\x91\xef\x90`\xda\x8b\x8e\x93b\x19\xca\r\x0bw\x00\"crates/warband_lib/src/dev/perf.rs\x00\x00\x00\x00\x00\x00\x00\x00f\xc7\x9b8\x1b\xe4\xf7\x1cf\xea\xd7\x020\x95(\xe4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0f\x03QwI\xe2\xed\xf1\xfbTr?\xbaw\x08\xbc\xeaf\xa2\xac\x0c\x1d\x00%crates/warband_lib/src/in_game/mod.rs\x00\x00\x00\x00\x00f\xc6B\x9e\x1b\xaa\x93tf\xe8\xa5\xa1\x04{\x84t\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x030\x8ccB\x14\xf0\xae2\x1c\x15\xad\xc4\xb6\xb1\x0f\xec\xb1\x1f<\x115\x00\x1dcrates/warband_lib/src/lib.rs\x00\x00\x00\x00\x00f\xe36 0x_\xc0f\xe8!@6;d\x0c\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01m\x7f\xefJS\xc8\xaa\xc8g \xde%\x9a\xf3U\x00\x96\x0fI\x12\xf5\x00*crates/warband_lib/src/navigation/agent.rs\x00\x00\x00\x00\x00\x00\x00\x00f\xe4\x86\x0b\x00\xae|\xc0f\xe8\x11\xe0\x0e\x88\x9fL\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x1f\xdc#.\x1a\x9eGQ\x1a\xa8\xcdn\xf9E\x96\xa1I\x15W\xb1\x00\x00(crates/warband_lib/src/navigation/mod.rs\x00\x00f\xe0P\xe1\x16MJ\x84f\xe8\x11\xe3\x1bO\x94$\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x18{T5\xe2\xf2\x91\xee\x11\x16\x1f~.^\x02\x8czg\xf6\xd3\xd32\x00/crates/warband_lib/src/navigation/navigation.rs\x00\x00\x00f\xe4\xb9\xa3\x10\xbc\xf9\xe8f\xe8!$\x05\xb3\xfc\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13M\xef\x0f\xab\xef\xfc\xabvJ\xc9\xdeO\x89\xcc\xcbD=\xf7\x02\xa3l\x00)crates/warband_lib/src/navigation/path.rs\x00f\xe5\xc8\x1a\x01\xc0l\xe0f\xe8$\x9e!\xca\xe5\xb0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xe8\xb7[\x855\xc2\xb8\x12\x8e0i m\xce\x91#\x9c\x967L\xb3\x00%crates/warband_lib/src/physics/mod.rs\x00\x00\x00\x00\x00f\xe8$\x8a\x0b\xb3\xb2pf\xe8$\xd0\x18\xf0\xa2\xcc\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x97N{\xfb^\x1fP\xa1\x97\xe7(\x05\xf1}\x00\xd0Vvj(\x81\x00\'crates/warband_lib/src/physics/motor.rs\x00\x00\x00f\xdf\x84s\n}#Pf\xe4\xa6V\x17\x8f\xe1T\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\xae\"q\xf2*&\x01\xa9\x0b\xca\x16F\xdar\xfbW\x05\x975\x9f\x85\x00\'crates/warband_lib/src/player/camera.rs\x00\x00\x00f\xdf\x84s\n}#Pf\xdf\x85\xcb*c\xd1\xa0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00v^\x8e=_#>R-\xaf\xd6\xed\x16j\xa1\x00\xf9\xe1Z\xa6\x8d\x00$crates/warband_lib/src/player/mod.rs\x00\x00\x00\x00\x00\x00f\xc6G\xab\x07\xf0\xa6\xfcf\xe8\x9dX\x03\x8a\x11\xe8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\xe8\xa6\xb1-7\xef\xb4\x1f\xf3\xf3\xe2\x12|(\x9cJ <\xa2>\xab\x00!crates/warband_lib/src/prelude.rs\x00f\xe8$\xde\x14\xaf\xc4\x04f\xe9FP\x1e;}\xc4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\xa25\x1b}\x01\xae\x85\xbc^\xed\xcd\x1aN\xc9\xcb\xd8\x85\x90L\x11\xa1\x00(crates/warband_lib/src/stats/_old/mod.rs\x00\x00f\xe8&G\nWA\xa8f\xe8\xa4\xa5\x1d\\\x1c8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1fJe\x9eY\x0b\xa3\xdc\xc6\x87\x8e\x10l\xb1\xbd\xc3\xb3\x12:\xec\xb2\x92\x00-crates/warband_lib/src/stats/_old/modifier.rs\x00\x00\x00\x00\x00f\xe8&\x9b5\x93\xa1,f\xe8CI\x12\x1e\xe9\xb8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13\x17o\x0f\xc6\x83\'9\x8f\"[\x9f\x02\xec\xb4\x0b\xeb\xbbq\xfb4\x91\x00)crates/warband_lib/src/stats/_old/pool.rs\x00f\xe8%\xfe!\t\x11`f\xe8\xa5\xa1\x04lB\x98\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x14<\x17\x85\r\xdeAy\x9c\x9f\x12d\xf4\xba)Mo\xa7I]n\xbc\x00)crates/warband_lib/src/stats/_old/stat.rs\x00f\xe9M(9%f\x90f\xe9S[\x11@5\x9c\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x01\xdb\xc99\xed}\xc0.d%W\xa0\x04\x94$\x06\xec\x96\xcf\xb0\xfb|\x00(crates/warband_lib/src/stats/design.tldr\x00\x00f\xe8$\xde\x14\xaf\xc4\x04f\xe9\xf1Q\x17\xbf\xbc\xe4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\xafF\xb3\xec\xb3D\x0e\xdf~\xfb\xcd\xa7I\x8c\xfe\xd2S\xf7\xd9.\x9c\x00#crates/warband_lib/src/stats/mod.rs\x00\x00\x00\x00\x00\x00\x00f\xe8&G\nWA\xa8f\xe9\xfd!)\x8c\xdd\x1c\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1f\xc2\xfa\x81K\x8eu\xca\xebz\n\xb9%o\xe1\x93\x93L\x0e-Wi\x00(crates/warband_lib/src/stats/modifier.rs\x00\x00f\xe8%\xfe!\t\x11`f\xe9\xff\'0-e8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nI\x01\xfa\x86\xea*\x18\xa7V\x06\xb2\x1d\xea\x82\x03my\x81k!\xf5\x00$crates/warband_lib/src/stats/stat.rs\x00\x00\x00\x00\x00\x00f\xc6R{9\x0c%Tf\xdf\x97\xa7\x12\xf1{0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\rCU\x80\x18u3\xa3~\x934+\xc4\xa4\x0e\xef\xee\x05\xd7\xc62D\x00\'crates/warband_lib/src/util/math_ext.rs\x00\x00\x00f\xc6P\x1d\n\x0fp\xe4f\xe4\xb9\x10\x03?~\x1c\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\xb4D5\x15\n\xcf\xbf\x9fW\xaa:\x1c\x8a\x11\xa7\xd6\x86\xe4h\x83\xc2\x00\"crates/warband_lib/src/util/mod.rs\x00\x00\x00\x00\x00\x00\x00\x00f\xc6O\xeb\n\xcaA\x80f\xdf\x97\xa6\x0e\x1d\\\xb4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x9c:=/T\xdc\xc7\xe9f0\xd4J\xb3qo C,\xf9+\xfb\x00#crates/warband_lib/src/util/pipe.rs\x00\x00\x00\x00\x00\x00\x00f\xc6J\x89$\x11\xed(f\xdf\x97\xa2.\x7fz\xa0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\xa2\xa6\xca\x863o\x90\x06\x88\x9a\xe6\xf5\xb5\x8c-\x05\xdd\xc7\x04\xd7!\x00!crates/warband_lib/src/version.rs\x00f\xc6=\x0c4Z\x95\xb8f\xe9\xfe\xa7\x10\xc21t\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x7f\x02\x12\xd0\xb3\xbe\x98W>\xb4\x9fd\xfaxAH,\xca\xb8f\xea\x00 crates/warband_macros/Cargo.toml\x00\x00f\xc7\x9aO\x05BJ\xc8f\xc7\x9a\x87\x10E_(\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\xf0\xa2u\x90\\n\x1c\xfe\n\xa3]\xb1\xefY\x9a\xac\x9e\x9cM8\x8b\x00!crates/warband_macros/src/bevy.rs\x00f\xc6B\xa45g\x8d\x94f\xe9\xff\x0c\n\x93\xf2`\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\xdfH(z4\x8f\xd1%\xb5\xafm\x1f\xd0\x9f\xcd\xeb\x83j\x9a\xd5\x92\x00 crates/warband_macros/src/lib.rs\x00\x00f\xe8\x88[6gu\xb0f\xe9\xfeh\x17\x1a2`\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x18\xe3\xb8\x1d\xcdI\xc8`\x90\x16\xb4:\xc2A^\x8e!bZ\xa8\x94w\x00!crates/warband_macros/src/stat.rs\x00f\xe2\x19\xde!\x15\xec\xc8f\xe2\x1b\x10\"\xe4\x12\xf0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0f\xae\x1d!\xb6\xfe\x9d\xa1\xf00!&\xe9R\xd6z(/\x153\xa7P\x00\x14docs/navigation.tldr\x00\x00\x00\x00\x00\x00f\xc6#y.c\xc5\xa8f\xe1\xa8:\x05&],\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00 \xbc\xd5\n\xdb\x92MNL\xafu\xcau\xc1\x9fk\x1e\x17\xc7t!\xc1\xf6\x00\x13docs/workspace.tldr\x00\x00\x00\x00\x00\x00\x00f\xc6#:.%\"\x1cf\xc7\x98\xc104\xa7l\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\xfa1\x81\xd4\xf3\xa0\x82\xa3\xe2r\x9d\xc7\xff4`\xf7\xa2p\x10\x8a\x9f\x00\x08justfile\x00\x00f\xc6#2,\xf9\x18\xbcf\x01\xbc\xdb\x138\x84X\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00 \'\x18\x00\xcb/7\x91\xb3\xad\xc2C(\xe7\x1c\x9e%P\xb49\xdb\x00\x13rust-toolchain.toml\x00\x00\x00\x00\x00\x00\x00f\xc6#6\x00\x89\x9dLf\xdf\x97\x84/\xf0\xfeT\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x81\xa4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01!G\xcb:\x08\xdf\xf6\xccF~\x99F\xb1Ha\xbe\xf7\xd3\xd2\x03\xff\x00\x0crustfmt.toml\x00\x00\x00\x00\x00\x00TREE\x00\x00\x03}\x0070 6\n\xe1U\x8e\xd4\xb0/\xb8?\xce!\x83\xbc\x9d\xd2\x05\x93\x9f\\\x9c\xe9docs\x002 0\nm:\xdd[\xe5\xe2\xe1\\\xe3E\x91\x8c\x9dIN\r5\xa52kbuild\x002 1\ne\xd3\x8c\x8a\x06\xcc\xef\x14\xb0\x01\xb3DKz\x8b\xb2+#b$win\x002 0\n\xeb\x94\x946\x18\x18\xeel\xe6x\xbe\xee\xe4\xac\xe3\xf7T\xabc\xa3.cargo\x001 0\ny\x06Y>\xb5\xd6\xb2rPdN\x81\xb1&\xd6O\xf8\x1eW0assets\x008 4\nl*\xecD#\x0cX;!;Q\xca\xfa\nI\xf5rg0\xb3fonts\x003 0\ncE-$\x1c\x0f\xd3\xa8>q\xd8\x1f\xd1\xca\xddx\x7f\xa4\xd8\xc5images\x002 0\n>\x8d\xcf\xf0\x18\x05!\xa4T\x18Ddjq\x84\x9d:\r\x81\xa5models\x001 0\nv\xdb\xb3\xbd\xb4P\x94\x1aXI_{\xda\xf4\xb4\xc7D\xe3\xea\xbashaders\x002 0\n\xd9\xbb\xe4\x81a1\xb9\x99\x10\xa7\xb6`LD\x88{\xd4\xe4\xec\xefcrates\x0047 3\n\xb9\x84\xdd\x17\xa9:q\xd9\x951\x1c\xec\x04<4\x00\x97\xf1x\xc5warband\x003 1\n\xff\xa9u\x98\x10\x82+Q\xaf\x03\xefP\xc5\x9b.\x18o\xd6\xb1\xecsrc\x001 0\n?\xe5\xaf\x85\xf3y\x82\xacn\xb0\x82MU\x99\x04\xcfZ-\x1cqwarband_lib\x0040 1\n\xd4C\xbd\xbbO<\xd3\x8b\x90\xed\xfc\xc8\x11\x83\xc7,\xa7\xa7g\x17src\x0039 10\nu\x13\nT=\x8c\xd9\xc4T\x85)\xd4\x19\x8c\x08\xe2\xc8\n\xc9\xd5dev\x004 0\n\x83\xe4\xa8\xfa\xde\x05\xaa\x7f\xe9\xbb\"3g\x17\xbc\xf7\xfc\xb6\xf7\x14core\x007 0\n\xb7t. V\xbe\x8dM{i\xf3\x07\xfez\xa7\xe2aQ\xd5\xb0util\x003 0\n\x15\xf5\xc5=\"\xe4UX\xa5|r\x1e\x85\xa8\x99\x02\xbe<\xb4{board\x004 0\n\xcd\'4\x91<\x97\x8cf\x8b\x93\xb6\xcb@\t\t\xf7\xfd\x19a/stats\x008 1\n\xec=c\xc6\x96\xc7@\xef\x02\x9e\xc7\xc1\x13\xf4\r\r1JQ{_old\x004 0\n\x95\xd7\xd9\x86\xca\xd6\xd8\xcd\xd0+\x1a\x96e\xbb\x0cS:\x80\x8czassets\x001 0\n\x9a\x934\xe3nhG\x8d\x82\x10\xbc\xe9\xb6\xe7]D/W<\x8eplayer\x002 0\nt\xad$\xfeTw\x00(W\xc3\x7fJ\xe6=\xed\xfb)\x96dnin_game\x001 0\n?\xd1\xadU2\xdd~m\xec{\xd1\xb5-\xea\x16\xdb=\xefPophysics\x002 0\n\x85\x0c\xc38\x02#\x1b\x81/=\xb8\xd6\xf1\xa65\x18\xba\xd7\x08\xd8navigation\x004 0\n@B\xbe\xff\x0b\xae*\xc8% )}\x08#\xbb\xa9PP}\xfawarband_macros\x004 1\n\x8c\xb5\xeb\x10\xa0\xdb9\xf7&0S\xd4\xc4\x92\xc9Q\xbf\xd4v\x14src\x003 0\n\x88}I\xe9\x08\xd0\'8Wr\"\xf0\x05\xa9U\x14\xdf\xe3\x80\xe0.vscode\x002 0\n\x03\x8c\xb1KV[\x86X3\x8bD\xecr|x@\x05\x8b\t\xa9\xe9r\xa8\x01cM\x1c\xf6\x12\xea\xa7\xe93\xc4\x07\xdc\xa2\xa8>\xbb";
            ;
            "5cd006d-modified"
        };
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    pub struct VERSION {
        __private_field: (),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    pub static VERSION: VERSION = VERSION { __private_field: () };
    impl ::lazy_static::__Deref for VERSION {
        type Target = Semver;
        fn deref(&self) -> &Semver {
            #[inline(always)]
            fn __static_ref_initialize() -> Semver {
                Semver {
                    major: "0".parse().unwrap(),
                    minor: "1".parse().unwrap(),
                    patch: "0".parse().unwrap(),
                }
            }
            #[inline(always)]
            fn __stability() -> &'static Semver {
                static LAZY: ::lazy_static::lazy::Lazy<Semver> =
                    ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for VERSION {
        fn initialize(lazy: &Self) { let _ = &**lazy; }
    }
    pub fn version() -> &'static str {
        use const_format::concatcp;
        ::const_format::pmr::__AssertStr {
                x: {
                    use ::const_format::__cf_osRcTFl4A;
                    ({
                            #[doc(hidden)]
                            #[allow(unused_mut, non_snake_case)]
                            const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument]
                                =
                                {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[__cf_osRcTFl4A::pmr::PConvWrapper("0").to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(".").to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("1").to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(".").to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("0").to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("+").to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(GIT_VERSION).to_pargument_display(fmt)]
                                };
                            {
                                #[doc(hidden)]
                                const ARR_LEN: usize =
                                    ::const_format::pmr::PArgument::calc_len(CONCATP_NHPMWYD3NJA);
                                #[doc(hidden)]
                                const CONCAT_ARR:
                                    &::const_format::pmr::LenAndArray<[u8; ARR_LEN]> =
                                    &::const_format::pmr::__priv_concatenate(CONCATP_NHPMWYD3NJA);
                                #[doc(hidden)]
                                #[allow(clippy :: transmute_ptr_to_ptr)]
                                const CONCAT_STR: &str =
                                    unsafe {
                                        let slice =
                                            ::const_format::pmr::transmute::<&[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len]>(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str =
                                                {
                                                    ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }.reff
                                                };
                                            string
                                        }
                                    };
                                CONCAT_STR
                            }
                        })
                },
            }.x
    }
}
use prelude::*;
pub use version::*;
mod board {
    use crate::{prelude::*, AppState};
    use bevy_inspector_egui::prelude::*;
    use bevy_mod_picking::prelude::*;
    use hexx::*;
    pub mod footprint {
        use hexx::Hex;
        use crate::{navigation::agent::Agent, prelude::*};
        use super::{
            occupied::GetPolygon, Board, BoardSettings, Cell, Location,
        };
        #[reflect(Component)]
        pub enum Footprint {

            #[default]
            Empty,
            Cells(SmallVec<[Hex; 8]>),
        }
        impl bevy::ecs::component::Component for Footprint where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl ::core::default::Default for Footprint {
            #[inline]
            fn default() -> Footprint { Self::Empty }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Footprint where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync,
                    SmallVec<[Hex; 8]>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <SmallVec<[Hex; 8]> as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Footprint where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync,
                    SmallVec<[Hex; 8]>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Enum(bevy::reflect::EnumInfo::new::<Self>(&[bevy::reflect::VariantInfo::Unit(bevy::reflect::UnitVariantInfo::new("Empty").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                            bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("Cells",
                                                                        &[bevy::reflect::UnnamedField::new::<SmallVec<[Hex; 8]>>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Footprint where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::board::footprint::Footprint"
                    }
                    fn short_type_path() -> &'static str { "Footprint" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Footprint")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::board::footprint".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::board::footprint")
                    }
                }
                impl bevy::reflect::Enum for Footprint where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync,
                    SmallVec<[Hex; 8]>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, __name_param: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at(&self, __index_param: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self {
                            Footprint::Cells { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, __name_param: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at_mut(&mut self, __index_param: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self {
                            Footprint::Cells { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn index_of(&self, __name_param: &str)
                        -> ::core::option::Option<usize> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn name_at(&self, __index_param: usize)
                        -> ::core::option::Option<&str> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn iter_fields(&self) -> bevy::reflect::VariantFieldIter {
                        bevy::reflect::VariantFieldIter::new(self)
                    }
                    #[inline]
                    fn field_len(&self) -> usize {
                        match self {
                            Footprint::Empty { .. } => 0usize,
                            Footprint::Cells { .. } => 1usize,
                            _ => 0,
                        }
                    }
                    #[inline]
                    fn variant_name(&self) -> &str {
                        match self {
                            Footprint::Empty { .. } => "Empty",
                            Footprint::Cells { .. } => "Cells",
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_index(&self) -> usize {
                        match self {
                            Footprint::Empty { .. } => 0usize,
                            Footprint::Cells { .. } => 1usize,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_type(&self) -> bevy::reflect::VariantType {
                        match self {
                            Footprint::Empty { .. } => bevy::reflect::VariantType::Unit,
                            Footprint::Cells { .. } =>
                                bevy::reflect::VariantType::Tuple,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicEnum {
                        bevy::reflect::DynamicEnum::from_ref::<Self>(self)
                    }
                }
                impl bevy::reflect::Reflect for Footprint where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync,
                    SmallVec<[Hex; 8]>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Enum::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        __value_param:
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(__value_param)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self,
                        __value_param: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Enum(__value_param) =
                                    bevy::reflect::Reflect::reflect_ref(__value_param) {
                                if bevy::reflect::Enum::variant_name(self) ==
                                            bevy::reflect::Enum::variant_name(__value_param) {
                                        match bevy::reflect::Enum::variant_type(__value_param) {
                                            bevy::reflect::VariantType::Struct => {
                                                for field in bevy::reflect::Enum::iter_fields(__value_param)
                                                    {
                                                    let name = field.name().unwrap();
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_mut(self, name) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            bevy::reflect::VariantType::Tuple => {
                                                for (index, field) in
                                                    ::core::iter::Iterator::enumerate(bevy::reflect::Enum::iter_fields(__value_param))
                                                    {
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_at_mut(self, index) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            _ => {}
                                        }
                                    } else {
                                       match bevy::reflect::Enum::variant_name(__value_param) {
                                           "Empty" => { *self = Footprint::Empty {} }
                                           "Cells" => {
                                               *self =
                                                   Footprint::Cells {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("Cells"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <SmallVec<[Hex; 8]> as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<SmallVec<[Hex; 8]> as
                                                                                   bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           name => {
                                               return ::core::result::Result::Err(bevy::reflect::ApplyError::UnknownVariant {
                                                           enum_name: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(self)),
                                                           variant_name: ::core::convert::Into::into(name),
                                                       });
                                           }
                                       }
                                   }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(__value_param),
                                           to_kind: bevy::reflect::ReflectKind::Enum,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Enum
                    }
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Enum(self)
                    }
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Enum(self)
                    }
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Enum(self)
                    }
                    fn reflect_hash(&self) -> ::core::option::Option<u64> {
                        bevy::reflect::enum_hash(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::enum_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Footprint where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync,
                    SmallVec<[Hex; 8]>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(__param0: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Enum(__param0) =
                                    bevy::reflect::Reflect::reflect_ref(__param0) {
                                match bevy::reflect::Enum::variant_name(__param0) {
                                    "Empty" =>
                                        ::core::option::Option::Some(Footprint::Empty {}),
                                    "Cells" =>
                                        ::core::option::Option::Some(Footprint::Cells {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <SmallVec<[Hex; 8]> as
                                                                bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    name => {
                                        ::core::panicking::panic_fmt(format_args!("variant with name `{0}` does not exist on enum `{1}`",
                                                name, <Self as bevy::reflect::TypePath>::type_path()));
                                    }
                                }
                            } else { ::core::option::Option::None }
                    }
                }
            };
        impl Footprint {
            pub fn cells(&self) -> impl Iterator<Item = &Hex> {
                match self {
                    Footprint::Empty => [].iter(),
                    Footprint::Cells(cells) => cells.iter(),
                }
            }
        }
        pub(super) fn agents(mut agents:
                Query<(&mut Footprint, &Location, &Agent),
                (Or<(Changed<Location>, Added<Footprint>)>, Without<Cell>)>,
            board: Res<Board>) {
            agents.par_iter_mut().for_each(|(mut footprint, location, agent)|
                    {
                        let Location::Valid(hex) =
                            location else { *footprint = Footprint::Empty; return; };
                        *footprint =
                            Footprint::Cells(hex.range(agent.size().saturating_sub(1) as
                                                u32).filter(|h| board.bounds.is_in_bounds(*h)).collect());
                    });
        }
        #[inline]
        pub(super) fn obstacles(mut obstacles:
                Query<(&mut Footprint, &Location, &GlobalTransform,
                &Collider),
                (Or<(Changed<Location>, Changed<ColliderAabb>,
                Added<Footprint>)>, Without<Agent>, Without<Cell>)>,
            board: Res<Board>, board_settings: Res<BoardSettings>) {
            let hexes_with_points: Vec<(Hex, Vec<Vec2>)> =
                board.cells().map(|(h, _)|
                            {
                                (h,
                                    std::iter::once(board.layout.hex_to_world_pos(h)).chain(board.layout.all_edge_coordinates(h).map(|d|
                                                    d[0])).collect())
                            }).collect();
            for (mut footprint, location, global_transform, collider) in
                &mut obstacles {
                let Location::Valid(_) =
                    location else { *footprint = Footprint::Empty; return; };
                let polygon =
                    collider.get_polygon(global_transform, &board.transform,
                        (Dir3::Y, board_settings.upward_shift));
                *footprint =
                    Footprint::Cells(hexes_with_points.iter().filter_map(|(h,
                                        p)|
                                    {
                                        const MIN_EDGE_INTERSECTIONS: usize = 2;
                                        if at_least(p.iter(), |p| point_in_poly2d(**p, &polygon),
                                                    MIN_EDGE_INTERSECTIONS) {
                                                Some(*h)
                                            } else { None }
                                    }).collect());
            }
        }
        #[inline]
        fn at_least<I, F>(iter: I, mut predicate: F, count: usize) -> bool
            where I: IntoIterator, F: FnMut(&I::Item) -> bool {
            if !(count > 0) {
                    ::core::panicking::panic("assertion failed: count > 0")
                };
            let mut matches = 0;
            for item in iter {
                if predicate(&item) {
                        matches += 1;
                        if matches >= count { return true; }
                    }
            }
            false
        }
        /// ref: https://github.com/Jondolf/barry/blob/main/src/utils/point_in_poly2d.rs
        #[inline]
        fn point_in_poly2d(pt: Vec2, poly: &[Vec2]) -> bool {
            if poly.is_empty() {
                    false
                } else {
                   let mut sign = 0.0;
                   for i1 in 0..poly.len() {
                       let i2 = (i1 + 1) % poly.len();
                       let seg_dir = poly[i2] - poly[i1];
                       let dpt = pt - poly[i1];
                       let perp = dpt.perp_dot(seg_dir);
                       if sign.is_zero() {
                               sign = perp;
                           } else if sign * perp < 0.0 { return false; }
                   }
                   true
               }
        }
    }
    pub mod location {
        use hexx::Hex;
        use crate::prelude::*;
        use super::Board;
        #[reflect(Component)]
        pub enum Location {

            #[default]
            Invalid,
            Valid(Hex),
        }
        impl bevy::ecs::component::Component for Location where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl ::core::default::Default for Location {
            #[inline]
            fn default() -> Location { Self::Invalid }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Location { }
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Location {
            #[inline]
            fn eq(&self, other: &Location) -> bool {
                let __self_discr =
                    ::core::intrinsics::discriminant_value(self);
                let __arg1_discr =
                    ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr &&
                    match (self, other) {
                        (Location::Valid(__self_0), Location::Valid(__arg1_0)) =>
                            __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Location {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Hex>;
            }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Location where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Hex: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <Hex as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Location where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Hex: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Enum(bevy::reflect::EnumInfo::new::<Self>(&[bevy::reflect::VariantInfo::Unit(bevy::reflect::UnitVariantInfo::new("Invalid").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                            bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("Valid",
                                                                        &[bevy::reflect::UnnamedField::new::<Hex>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Location where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::board::location::Location"
                    }
                    fn short_type_path() -> &'static str { "Location" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Location")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::board::location".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::board::location")
                    }
                }
                impl bevy::reflect::Enum for Location where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Hex: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, __name_param: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at(&self, __index_param: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self {
                            Location::Valid { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, __name_param: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at_mut(&mut self, __index_param: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self {
                            Location::Valid { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn index_of(&self, __name_param: &str)
                        -> ::core::option::Option<usize> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn name_at(&self, __index_param: usize)
                        -> ::core::option::Option<&str> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn iter_fields(&self) -> bevy::reflect::VariantFieldIter {
                        bevy::reflect::VariantFieldIter::new(self)
                    }
                    #[inline]
                    fn field_len(&self) -> usize {
                        match self {
                            Location::Invalid { .. } => 0usize,
                            Location::Valid { .. } => 1usize,
                            _ => 0,
                        }
                    }
                    #[inline]
                    fn variant_name(&self) -> &str {
                        match self {
                            Location::Invalid { .. } => "Invalid",
                            Location::Valid { .. } => "Valid",
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_index(&self) -> usize {
                        match self {
                            Location::Invalid { .. } => 0usize,
                            Location::Valid { .. } => 1usize,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_type(&self) -> bevy::reflect::VariantType {
                        match self {
                            Location::Invalid { .. } =>
                                bevy::reflect::VariantType::Unit,
                            Location::Valid { .. } => bevy::reflect::VariantType::Tuple,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicEnum {
                        bevy::reflect::DynamicEnum::from_ref::<Self>(self)
                    }
                }
                impl bevy::reflect::Reflect for Location where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Hex: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Enum::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        __value_param:
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(__value_param)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self,
                        __value_param: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Enum(__value_param) =
                                    bevy::reflect::Reflect::reflect_ref(__value_param) {
                                if bevy::reflect::Enum::variant_name(self) ==
                                            bevy::reflect::Enum::variant_name(__value_param) {
                                        match bevy::reflect::Enum::variant_type(__value_param) {
                                            bevy::reflect::VariantType::Struct => {
                                                for field in bevy::reflect::Enum::iter_fields(__value_param)
                                                    {
                                                    let name = field.name().unwrap();
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_mut(self, name) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            bevy::reflect::VariantType::Tuple => {
                                                for (index, field) in
                                                    ::core::iter::Iterator::enumerate(bevy::reflect::Enum::iter_fields(__value_param))
                                                    {
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_at_mut(self, index) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            _ => {}
                                        }
                                    } else {
                                       match bevy::reflect::Enum::variant_name(__value_param) {
                                           "Invalid" => { *self = Location::Invalid {} }
                                           "Valid" => {
                                               *self =
                                                   Location::Valid {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("Valid"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <Hex as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<Hex as
                                                                                   bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           name => {
                                               return ::core::result::Result::Err(bevy::reflect::ApplyError::UnknownVariant {
                                                           enum_name: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(self)),
                                                           variant_name: ::core::convert::Into::into(name),
                                                       });
                                           }
                                       }
                                   }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(__value_param),
                                           to_kind: bevy::reflect::ReflectKind::Enum,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Enum
                    }
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Enum(self)
                    }
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Enum(self)
                    }
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Enum(self)
                    }
                    fn reflect_hash(&self) -> ::core::option::Option<u64> {
                        bevy::reflect::enum_hash(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::enum_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Location where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Hex: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(__param0: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Enum(__param0) =
                                    bevy::reflect::Reflect::reflect_ref(__param0) {
                                match bevy::reflect::Enum::variant_name(__param0) {
                                    "Invalid" =>
                                        ::core::option::Option::Some(Location::Invalid {}),
                                    "Valid" =>
                                        ::core::option::Option::Some(Location::Valid {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <Hex as bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    name => {
                                        ::core::panicking::panic_fmt(format_args!("variant with name `{0}` does not exist on enum `{1}`",
                                                name, <Self as bevy::reflect::TypePath>::type_path()));
                                    }
                                }
                            } else { ::core::option::Option::None }
                    }
                }
            };
        pub(super) fn location(mut transforms:
                Query<(&mut Location, &GlobalTransform),
                Or<(Changed<GlobalTransform>, Added<Location>)>>,
            board: Res<Board>) {
            transforms.par_iter_mut().for_each(|(mut location, transform)|
                    {
                        let value = to_location(&board, transform);
                        if *location != value { *location = value; }
                    });
        }
        pub(super) fn on_board_built(mut transforms:
                Query<(&mut Location, &GlobalTransform)>, board: Res<Board>) {
            transforms.par_iter_mut().for_each(|(mut location, transform)|
                    {
                        let value = to_location(&board, transform);
                        if *location != value { *location = value; }
                    });
        }
        #[inline]
        fn to_location(board: &Board, transform: &GlobalTransform)
            -> Location {
            let hex: Hex =
                board.layout.world_pos_to_hex(transform.translation().xz());
            if board.bounds.is_in_bounds(hex) {
                    Location::Valid(hex)
                } else { Location::Invalid }
        }
    }
    pub mod occupied {
        use avian3d::{
            parry::{
                na::{Const, OPoint, Unit, Vector3},
                query::IntersectResult, shape::{Polyline, TriMesh, TypedShape},
            },
            prelude::Collider,
        };
        use dashmap::DashMap;
        use hexx::Hex;
        use crate::prelude::*;
        use super::Footprint;
        pub struct Occupied(Arc<OccupiedBoard>);
        impl bevy::ecs::system::Resource for Occupied where Self: Send +
            Sync + 'static {}
        #[automatically_derived]
        impl ::core::fmt::Debug for Occupied {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f,
                    "Occupied", &&self.0)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Occupied {
            #[inline]
            fn default() -> Occupied {
                Occupied(::core::default::Default::default())
            }
        }
        impl ::std::ops::Deref for Occupied {
            type Target = Arc<OccupiedBoard>;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl ::std::ops::DerefMut for Occupied {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
        pub struct OccupiedBoard(DashMap<Hex, HashSet<Entity>>);
        impl bevy::ecs::system::Resource for OccupiedBoard where Self: Send +
            Sync + 'static {}
        #[automatically_derived]
        impl ::core::fmt::Debug for OccupiedBoard {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f,
                    "OccupiedBoard", &&self.0)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for OccupiedBoard {
            #[inline]
            fn default() -> OccupiedBoard {
                OccupiedBoard(::core::default::Default::default())
            }
        }
        impl ::std::ops::Deref for OccupiedBoard {
            type Target = DashMap<Hex, HashSet<Entity>>;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl ::std::ops::DerefMut for OccupiedBoard {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
        impl OccupiedBoard {
            pub fn cells(&self) -> impl Iterator<Item = Hex> + '_ {
                self.iter().map(|v| *v.key())
            }
        }
        pub(super) fn splat(footprints_changed:
                Query<Entity, Changed<Footprint>>,
            footprints: Query<(Entity, &Footprint)>,
            obstacles: ResMut<Occupied>) {
            if footprints_changed.is_empty() { return; }
            obstacles.clear();
            for (entity, footprint) in &footprints {
                for hex in footprint.cells().copied() {
                    obstacles.entry(hex).or_default().insert(entity);
                }
            }
        }
        /// ref: https://github.com/vleue/vleue_navigator/blob/main/src/obstacles/mod.rs
        pub trait GetPolygon: Component + Clone {
            /// Get the polygon of the obstacle in the local space of the mesh.
            fn get_polygon(&self, obstacle: &GlobalTransform,
            board: &Transform, up: (Dir3, f32))
            -> Vec<Vec2>;
        }
        impl GetPolygon for Collider {
            fn get_polygon(&self, obstacle: &GlobalTransform,
                board: &Transform, up: (Dir3, f32)) -> Vec<Vec2> {
                self.shape_scaled().as_typed_shape().get_polygon(obstacle,
                    board, up)
            }
        }
        trait InnerGetPolygon {
            fn get_polygon(&self, obstacle: &GlobalTransform,
            board: &Transform, up: (Dir3, f32))
            -> Vec<Vec2>;
        }
        impl<'a> InnerGetPolygon for TypedShape<'a> {
            fn get_polygon(&self, obstacle: &GlobalTransform,
                board: &Transform, (up, shift): (Dir3, f32)) -> Vec<Vec2> {
                const RESOLUTION: u32 = 32;
                let mut transform = obstacle.compute_transform();
                transform.scale = Vec3::ONE;
                let world_to_board = board.compute_affine().inverse();
                let to_board =
                    |p: OPoint<f32, Const<3>>|
                        world_to_board.transform_point(vec3(p.x, p.y, p.z)).xz();
                let intersection_to_board =
                    |intersection: IntersectResult<Polyline>|
                        match intersection {
                            IntersectResult::Intersect(i) =>
                                i.segments().map(|s| s.a).map(to_board).collect(),
                            IntersectResult::Negative => ::alloc::vec::Vec::new(),
                            IntersectResult::Positive => ::alloc::vec::Vec::new(),
                        };
                let d =
                    (-up.x * board.translation.x - up.y * board.translation.y -
                                up.z * board.translation.z) /
                        (up.x.powi(2) + up.y.powi(2) + up.z.powi(2)).sqrt();
                let shift: f32 = shift - d;
                const MIN_SHIFT: f32 = 0.01;
                let shift =
                    if shift < 0.0 {
                            shift.min(-MIN_SHIFT)
                        } else { shift.max(MIN_SHIFT) };
                let to_world =
                    |p: &OPoint<f32, Const<3>>|
                        transform.transform_point(vec3(p.x, p.y, p.z));
                let up_axis =
                    Unit::new_normalize(Vector3::new(up.x, up.y, up.z));
                let trimesh_to_world =
                    |vertices: Vec<OPoint<f32, Const<3>>>|
                        {
                            vertices.iter().map(to_world).map(|v|
                                        v.into()).collect::<Vec<OPoint<f32, Const<3>>>>()
                        };
                match self {
                                TypedShape::Cuboid(collider) => {
                                    let (vertices, indices) = collider.to_trimesh();
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::Ball(collider) => {
                                    let (vertices, indices) =
                                        collider.to_trimesh(RESOLUTION, RESOLUTION);
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::Capsule(collider) => {
                                    let (vertices, indices) =
                                        collider.to_trimesh(RESOLUTION, RESOLUTION);
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::TriMesh(collider) => {
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(collider.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::HeightField(collider) => {
                                    let (vertices, indices) = collider.to_trimesh();
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::Compound(collider) =>
                                    collider.shapes().iter().map(|(_iso, shape)|
                                                {
                                                    shape.as_typed_shape().get_polygon(obstacle, board,
                                                        (up, shift))
                                                }).collect(),
                                TypedShape::ConvexPolyhedron(collider) => {
                                    let (vertices, indices) = collider.to_trimesh();
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::Cylinder(collider) => {
                                    let (vertices, indices) = collider.to_trimesh(RESOLUTION);
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::Cone(collider) => {
                                    let (vertices, indices) = collider.to_trimesh(RESOLUTION);
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::RoundCuboid(collider) => {
                                    let (vertices, indices) = collider.inner_shape.to_trimesh();
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::RoundCylinder(collider) => {
                                    let (vertices, indices) =
                                        collider.inner_shape.to_trimesh(RESOLUTION);
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::RoundCone(collider) => {
                                    let (vertices, indices) =
                                        collider.inner_shape.to_trimesh(RESOLUTION);
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                TypedShape::RoundConvexPolyhedron(collider) => {
                                    let (vertices, indices) = collider.inner_shape.to_trimesh();
                                    let trimesh =
                                        TriMesh::new(trimesh_to_world(vertices), indices);
                                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([intersection_to_board(trimesh.intersection_with_local_plane(&up_axis,
                                                            shift, f32::EPSILON))]))
                                }
                                _ => {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite =
                                            {
                                                static META: ::tracing::Metadata<'static> =
                                                    {
                                                        ::tracing_core::metadata::Metadata::new("event crates\\warband_lib\\src\\board\\occupied.rs:216",
                                                            "warband_lib::board::occupied", ::tracing::Level::WARN,
                                                            ::core::option::Option::Some("crates\\warband_lib\\src\\board\\occupied.rs"),
                                                            ::core::option::Option::Some(216u32),
                                                            ::core::option::Option::Some("warband_lib::board::occupied"),
                                                            ::tracing_core::field::FieldSet::new(&["message"],
                                                                ::tracing_core::callsite::Identifier(&__CALLSITE)),
                                                            ::tracing::metadata::Kind::EVENT)
                                                    };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                        let enabled =
                                            ::tracing::Level::WARN <=
                                                        ::tracing::level_filters::STATIC_MAX_LEVEL &&
                                                    ::tracing::Level::WARN <=
                                                        ::tracing::level_filters::LevelFilter::current() &&
                                                {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never() &&
                                                        ::tracing::__macro_support::__is_enabled(__CALLSITE.metadata(),
                                                            interest)
                                                };
                                        if enabled {
                                                (|value_set: ::tracing::field::ValueSet|
                                                            {
                                                                let meta = __CALLSITE.metadata();
                                                                ::tracing::Event::dispatch(meta, &value_set);
                                                                if match ::tracing::Level::WARN {
                                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                                _ => ::tracing::log::Level::Trace,
                                                                            } <= ::tracing::log::STATIC_MAX_LEVEL {
                                                                        if !::tracing::dispatcher::has_been_set() {
                                                                                {
                                                                                    use ::tracing::log;
                                                                                    let level =
                                                                                        match ::tracing::Level::WARN {
                                                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                                            _ => ::tracing::log::Level::Trace,
                                                                                        };
                                                                                    if level <= log::max_level() {
                                                                                            let meta = __CALLSITE.metadata();
                                                                                            let log_meta =
                                                                                                log::Metadata::builder().level(level).target(meta.target()).build();
                                                                                            let logger = log::logger();
                                                                                            if logger.enabled(&log_meta) {
                                                                                                    ::tracing::__macro_support::__tracing_log(meta, logger,
                                                                                                        log_meta, &value_set)
                                                                                                }
                                                                                        }
                                                                                }
                                                                            } else { {} }
                                                                    } else { {} };
                                                            })({
                                                        #[allow(unused_imports)]
                                                        use ::tracing::field::{debug, display, Value};
                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                        __CALLSITE.metadata().fields().value_set(&[(&::core::iter::Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                                                                            ::core::option::Option::Some(&format_args!("Collider not supported for obstacle generation")
                                                                                    as &dyn Value))])
                                                    });
                                            } else {
                                               if match ::tracing::Level::WARN {
                                                               ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                               ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                               ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                               ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                               _ => ::tracing::log::Level::Trace,
                                                           } <= ::tracing::log::STATIC_MAX_LEVEL {
                                                       if !::tracing::dispatcher::has_been_set() {
                                                               {
                                                                   use ::tracing::log;
                                                                   let level =
                                                                       match ::tracing::Level::WARN {
                                                                           ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                           ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                           ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                           ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                           _ => ::tracing::log::Level::Trace,
                                                                       };
                                                                   if level <= log::max_level() {
                                                                           let meta = __CALLSITE.metadata();
                                                                           let log_meta =
                                                                               log::Metadata::builder().level(level).target(meta.target()).build();
                                                                           let logger = log::logger();
                                                                           if logger.enabled(&log_meta) {
                                                                                   ::tracing::__macro_support::__tracing_log(meta, logger,
                                                                                       log_meta,
                                                                                       &{
                                                                                               #[allow(unused_imports)]
                                                                                               use ::tracing::field::{debug, display, Value};
                                                                                               let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                               __CALLSITE.metadata().fields().value_set(&[(&::core::iter::Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                                                                                                                   ::core::option::Option::Some(&format_args!("Collider not supported for obstacle generation")
                                                                                                                           as &dyn Value))])
                                                                                           })
                                                                               }
                                                                       }
                                                               }
                                                           } else { {} }
                                                   } else { {} };
                                           }
                                    };
                                    ::alloc::vec::Vec::new()
                                }
                            }.into_iter().flatten().collect()
            }
        }
    }
    pub use footprint::Footprint;
    pub use location::Location;
    pub use occupied::Occupied;
    pub enum BoardSystems { Build, Location, Footprint, Occupied, }
    impl bevy::ecs::schedule::SystemSet for BoardSystems where Self: 'static +
        Send + Sync + Clone + Eq + ::std::fmt::Debug + ::std::hash::Hash {
        fn dyn_clone(&self)
            -> ::std::boxed::Box<dyn bevy::ecs::schedule::SystemSet> {
            ::std::boxed::Box::new(::std::clone::Clone::clone(self))
        }
        fn as_dyn_eq(&self) -> &dyn bevy::ecs::schedule::DynEq { self }
        fn dyn_hash(&self, mut state: &mut dyn ::std::hash::Hasher) {
            let ty_id = ::std::any::TypeId::of::<Self>();
            ::std::hash::Hash::hash(&ty_id, &mut state);
            ::std::hash::Hash::hash(self, &mut state);
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BoardSystems {
        #[inline]
        fn clone(&self) -> BoardSystems { *self }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for BoardSystems { }
    #[automatically_derived]
    impl ::core::fmt::Debug for BoardSystems {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f,
                match self {
                    BoardSystems::Build => "Build",
                    BoardSystems::Location => "Location",
                    BoardSystems::Footprint => "Footprint",
                    BoardSystems::Occupied => "Occupied",
                })
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BoardSystems { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BoardSystems {
        #[inline]
        fn eq(&self, other: &BoardSystems) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for BoardSystems {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for BoardSystems {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    pub(super) fn plugin(app: &mut App) {
        app.register_type::<Board>();
        app.register_type::<BoardSettings>();
        app.register_type::<BoardBounds>();
        app.register_type::<Cell>();
        app.register_type::<Location>();
        app.register_type::<BoardBuiltEvent>();
        app.register_type::<footprint::Footprint>();
        ;
        app.init_resource::<Board>().init_resource::<Occupied>().insert_resource(BoardSettings::builder().width(10).height(10).hex_size(1.0).build()).add_event::<BoardBuiltEvent>();
        app.configure_sets(FixedUpdate,
            (BoardSystems::Build.run_if(resource_exists::<BoardSettings>),
                            BoardSystems::Location, BoardSystems::Footprint,
                            BoardSystems::Occupied).chain().run_if(resource_exists::<Board>).run_if(in_state(AppState::InGame)));
        app.add_systems(FixedUpdate,
            (build.in_set(BoardSystems::Build),
                (location::location,
                        location::on_board_built.run_if(on_event::<BoardBuiltEvent>())).in_set(BoardSystems::Location),
                footprint::agents.in_set(BoardSystems::Footprint),
                footprint::obstacles.in_set(BoardSystems::Footprint),
                occupied::splat.chain().in_set(BoardSystems::Occupied)));
    }
    #[reflect(Resource, Default)]
    pub struct BoardSettings {
        #[inspector(min = 1.0, max = 10.0)]
        pub hex_size: f32,
        #[inspector(min = 1, max = 100)]
        pub width: i32,
        #[inspector(min = 1, max = 100)]
        pub height: i32,
        #[builder(default)]
        pub orientation: HexOrientation,
        /// Obstacle scale
        #[builder(default)]
        #[inspector(min = 0.0, max = 10.0)]
        pub obstacle_scale: f32,
        /// Upward shift to sample obstacle from the ground
        #[builder(default)]
        #[inspector(min = -5.0, max = 10.0)]
        pub upward_shift: f32,
    }
    #[automatically_derived]
    impl BoardSettings {
        #[doc =
        "Create an instance of [`BoardSettings`] using the builder syntax"]
        #[inline(always)]
        #[allow(clippy :: inline_always, clippy :: use_self, clippy ::
        missing_const_for_fn)]
        pub fn builder() -> BoardSettingsBuilder<> {
            BoardSettingsBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_members: (::bon::private::Unset(::bon::private::Required),
                    ::bon::private::Unset(::bon::private::Required),
                    ::bon::private::Unset(::bon::private::Required),
                    ::bon::private::Unset(::bon::private::Optional),
                    ::bon::private::Unset(::bon::private::Optional),
                    ::bon::private::Unset(::bon::private::Optional)),
            }
        }
    }
    #[doc(hidden)]
    pub type __BoardSettingsBuilderInitialState =
        (::bon::private::Unset<::bon::private::Required>,
        ::bon::private::Unset<::bon::private::Required>,
        ::bon::private::Unset<::bon::private::Required>,
        ::bon::private::Unset<::bon::private::Optional>,
        ::bon::private::Unset<::bon::private::Optional>,
        ::bon::private::Unset<::bon::private::Optional>);
    #[must_use =
    "the builder does nothing until you call `build()` on it to finish building"]
    #[doc =
    "Use builder syntax to set the required parameters and finish by calling the method [`Self::build()`]."]
    #[allow(unused_parens)]
    #[allow(clippy :: struct_field_names, clippy :: type_complexity)]
    pub struct BoardSettingsBuilder<___State =
        __BoardSettingsBuilderInitialState> {
        #[doc =
        "Please don't touch this field. It's an implementation detail that is exempt from the API stability guarantees. This field couldn't be hidden using Rust's privacy syntax. The details about this are described in [the blog post](https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).\n        "]
        __private_phantom: ::core::marker::PhantomData<(::core::marker::PhantomData<BoardSettings<>>,
        ::core::marker::PhantomData<f32>, ::core::marker::PhantomData<i32>,
        ::core::marker::PhantomData<i32>,
        ::core::marker::PhantomData<HexOrientation>,
        ::core::marker::PhantomData<f32>, ::core::marker::PhantomData<f32>,
        ::core::marker::PhantomData<___State>)>,
        #[doc =
        "Please don't touch this field. It's an implementation detail that is exempt from the API stability guarantees. This field couldn't be hidden using Rust's privacy syntax. The details about this are described in [the blog post](https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).\n        "]
        __private_members: ___State,
    }
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub struct BoardSettingsBuilder__hex_size;
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub struct BoardSettingsBuilder__width;
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub struct BoardSettingsBuilder__height;
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub struct BoardSettingsBuilder__orientation;
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub struct BoardSettingsBuilder__obstacle_scale;
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub struct BoardSettingsBuilder__upward_shift;
    #[allow(unused_parens)]
    #[automatically_derived]
    impl<__HexSize, __Width, __Height, __Orientation, __ObstacleScale,
        __UpwardShift>
        BoardSettingsBuilder<(__HexSize, __Width, __Height, __Orientation,
        __ObstacleScale, __UpwardShift)> {
        #[doc = "Finishes building and returns the requested object."]
        #[inline(always)]
        #[allow(clippy :: inline_always)]
        #[must_use = "building a struct without using it is likely a bug"]
        pub fn build(self) -> BoardSettings<> where
            __HexSize: ::bon::private::IntoSet<f32,
            BoardSettingsBuilder__hex_size>,
            __Width: ::bon::private::IntoSet<i32,
            BoardSettingsBuilder__width>,
            __Height: ::bon::private::IntoSet<i32,
            BoardSettingsBuilder__height>,
            __Orientation: ::bon::private::IntoSet<Option<HexOrientation>,
            BoardSettingsBuilder__orientation>,
            __ObstacleScale: ::bon::private::IntoSet<Option<f32>,
            BoardSettingsBuilder__obstacle_scale>,
            __UpwardShift: ::bon::private::IntoSet<Option<f32>,
            BoardSettingsBuilder__upward_shift> {
            let hex_size: f32 =
                ::bon::private::IntoSet::<f32,
                        BoardSettingsBuilder__hex_size>::into_set(self.__private_members.0);
            let width: i32 =
                ::bon::private::IntoSet::<i32,
                        BoardSettingsBuilder__width>::into_set(self.__private_members.1);
            let height: i32 =
                ::bon::private::IntoSet::<i32,
                        BoardSettingsBuilder__height>::into_set(self.__private_members.2);
            let orientation: HexOrientation =
                ::bon::private::IntoSet::<Option<HexOrientation>,
                            BoardSettingsBuilder__orientation>::into_set(self.__private_members.3).unwrap_or_default();
            let obstacle_scale: f32 =
                ::bon::private::IntoSet::<Option<f32>,
                            BoardSettingsBuilder__obstacle_scale>::into_set(self.__private_members.4).unwrap_or_default();
            let upward_shift: f32 =
                ::bon::private::IntoSet::<Option<f32>,
                            BoardSettingsBuilder__upward_shift>::into_set(self.__private_members.5).unwrap_or_default();
            BoardSettings {
                hex_size,
                width,
                height,
                orientation,
                obstacle_scale,
                upward_shift,
            }
        }
        #[doc =
        "Sets the value of `hex_size`. See [`BoardSettings::builder()`] for more info."]
        #[allow(clippy :: inline_always, clippy :: impl_trait_in_params)]
        #[inline(always)]
        pub fn hex_size(self, value: f32)
            ->
                BoardSettingsBuilder<(::bon::private::Set<f32>, __Width,
                __Height, __Orientation, __ObstacleScale, __UpwardShift)>
            where __HexSize: ::bon::private::IsUnset {
            BoardSettingsBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_members: (::bon::private::Set(value),
                    self.__private_members.1, self.__private_members.2,
                    self.__private_members.3, self.__private_members.4,
                    self.__private_members.5),
            }
        }
        #[doc =
        "Sets the value of `width`. See [`BoardSettings::builder()`] for more info."]
        #[allow(clippy :: inline_always, clippy :: impl_trait_in_params)]
        #[inline(always)]
        pub fn width(self, value: i32)
            ->
                BoardSettingsBuilder<(__HexSize, ::bon::private::Set<i32>,
                __Height, __Orientation, __ObstacleScale, __UpwardShift)>
            where __Width: ::bon::private::IsUnset {
            BoardSettingsBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_members: (self.__private_members.0,
                    ::bon::private::Set(value), self.__private_members.2,
                    self.__private_members.3, self.__private_members.4,
                    self.__private_members.5),
            }
        }
        #[doc =
        "Sets the value of `height`. See [`BoardSettings::builder()`] for more info."]
        #[allow(clippy :: inline_always, clippy :: impl_trait_in_params)]
        #[inline(always)]
        pub fn height(self, value: i32)
            ->
                BoardSettingsBuilder<(__HexSize, __Width,
                ::bon::private::Set<i32>, __Orientation, __ObstacleScale,
                __UpwardShift)> where __Height: ::bon::private::IsUnset {
            BoardSettingsBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_members: (self.__private_members.0,
                    self.__private_members.1, ::bon::private::Set(value),
                    self.__private_members.3, self.__private_members.4,
                    self.__private_members.5),
            }
        }
        #[doc =
        "Same as [`Self::orientation`], but accepts an `Option` as input. See that method's documentation for more details."]
        #[allow(clippy :: inline_always, clippy :: impl_trait_in_params)]
        #[inline(always)]
        pub fn maybe_orientation(self, value: Option<HexOrientation>)
            ->
                BoardSettingsBuilder<(__HexSize, __Width, __Height,
                ::bon::private::Set<Option<HexOrientation>>, __ObstacleScale,
                __UpwardShift)> where __Orientation: ::bon::private::IsUnset {
            BoardSettingsBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_members: (self.__private_members.0,
                    self.__private_members.1, self.__private_members.2,
                    ::bon::private::Set(value), self.__private_members.4,
                    self.__private_members.5),
            }
        }
        #[doc =
        "Sets the value of `orientation`. See [`BoardSettings::builder()`] for more info."]
        #[allow(clippy :: inline_always, clippy :: impl_trait_in_params)]
        #[inline(always)]
        pub fn orientation(self, value: HexOrientation)
            ->
                BoardSettingsBuilder<(__HexSize, __Width, __Height,
                ::bon::private::Set<Option<HexOrientation>>, __ObstacleScale,
                __UpwardShift)> where __Orientation: ::bon::private::IsUnset {
            self.maybe_orientation(Some(value))
        }
        #[doc =
        "Same as [`Self::obstacle_scale`], but accepts an `Option` as input. See that method's documentation for more details."]
        #[allow(clippy :: inline_always, clippy :: impl_trait_in_params)]
        #[inline(always)]
        pub fn maybe_obstacle_scale(self, value: Option<f32>)
            ->
                BoardSettingsBuilder<(__HexSize, __Width, __Height,
                __Orientation, ::bon::private::Set<Option<f32>>,
                __UpwardShift)> where
            __ObstacleScale: ::bon::private::IsUnset {
            BoardSettingsBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_members: (self.__private_members.0,
                    self.__private_members.1, self.__private_members.2,
                    self.__private_members.3, ::bon::private::Set(value),
                    self.__private_members.5),
            }
        }
        #[doc = " Obstacle scale"]
        #[allow(clippy :: inline_always, clippy :: impl_trait_in_params)]
        #[inline(always)]
        pub fn obstacle_scale(self, value: f32)
            ->
                BoardSettingsBuilder<(__HexSize, __Width, __Height,
                __Orientation, ::bon::private::Set<Option<f32>>,
                __UpwardShift)> where
            __ObstacleScale: ::bon::private::IsUnset {
            self.maybe_obstacle_scale(Some(value))
        }
        #[doc =
        "Same as [`Self::upward_shift`], but accepts an `Option` as input. See that method's documentation for more details."]
        #[allow(clippy :: inline_always, clippy :: impl_trait_in_params)]
        #[inline(always)]
        pub fn maybe_upward_shift(self, value: Option<f32>)
            ->
                BoardSettingsBuilder<(__HexSize, __Width, __Height,
                __Orientation, __ObstacleScale,
                ::bon::private::Set<Option<f32>>)> where
            __UpwardShift: ::bon::private::IsUnset {
            BoardSettingsBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_members: (self.__private_members.0,
                    self.__private_members.1, self.__private_members.2,
                    self.__private_members.3, self.__private_members.4,
                    ::bon::private::Set(value)),
            }
        }
        #[doc = " Upward shift to sample obstacle from the ground"]
        #[allow(clippy :: inline_always, clippy :: impl_trait_in_params)]
        #[inline(always)]
        pub fn upward_shift(self, value: f32)
            ->
                BoardSettingsBuilder<(__HexSize, __Width, __Height,
                __Orientation, __ObstacleScale,
                ::bon::private::Set<Option<f32>>)> where
            __UpwardShift: ::bon::private::IsUnset {
            self.maybe_upward_shift(Some(value))
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BoardSettings {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ =
                &["hex_size", "width", "height", "orientation",
                            "obstacle_scale", "upward_shift"];
            let values: &[&dyn ::core::fmt::Debug] =
                &[&self.hex_size, &self.width, &self.height,
                            &self.orientation, &self.obstacle_scale,
                            &&self.upward_shift];
            ::core::fmt::Formatter::debug_struct_fields_finish(f,
                "BoardSettings", names, values)
        }
    }
    impl bevy::ecs::system::Resource for BoardSettings where Self: Send +
        Sync + 'static {}
    #[automatically_derived]
    impl ::core::default::Default for BoardSettings {
        #[inline]
        fn default() -> BoardSettings {
            BoardSettings {
                hex_size: ::core::default::Default::default(),
                width: ::core::default::Default::default(),
                height: ::core::default::Default::default(),
                orientation: ::core::default::Default::default(),
                obstacle_scale: ::core::default::Default::default(),
                upward_shift: ::core::default::Default::default(),
            }
        }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for BoardSettings where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexOrientation: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<ReflectResource>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<ReflectDefault>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {
                    <f32 as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <i32 as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <i32 as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <HexOrientation as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <f32 as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <f32 as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                }
            }
            impl bevy::reflect::Typed for BoardSettings where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexOrientation: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<f32>("hex_size").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<i32>("width").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<i32>("height").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<HexOrientation>("orientation").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<f32>("obstacle_scale").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<f32>("upward_shift").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for BoardSettings where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str {
                    "warband_lib::board::BoardSettings"
                }
                fn short_type_path() -> &'static str { "BoardSettings" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("BoardSettings")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board")
                }
            }
            impl bevy::reflect::Struct for BoardSettings where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexOrientation: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn field(&self, name: &str)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match name {
                        "hex_size" => ::core::option::Option::Some(&self.hex_size),
                        "width" => ::core::option::Option::Some(&self.width),
                        "height" => ::core::option::Option::Some(&self.height),
                        "orientation" =>
                            ::core::option::Option::Some(&self.orientation),
                        "obstacle_scale" =>
                            ::core::option::Option::Some(&self.obstacle_scale),
                        "upward_shift" =>
                            ::core::option::Option::Some(&self.upward_shift),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_mut(&mut self, name: &str)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match name {
                        "hex_size" =>
                            ::core::option::Option::Some(&mut self.hex_size),
                        "width" => ::core::option::Option::Some(&mut self.width),
                        "height" => ::core::option::Option::Some(&mut self.height),
                        "orientation" =>
                            ::core::option::Option::Some(&mut self.orientation),
                        "obstacle_scale" =>
                            ::core::option::Option::Some(&mut self.obstacle_scale),
                        "upward_shift" =>
                            ::core::option::Option::Some(&mut self.upward_shift),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&self.hex_size),
                        1usize => ::core::option::Option::Some(&self.width),
                        2usize => ::core::option::Option::Some(&self.height),
                        3usize => ::core::option::Option::Some(&self.orientation),
                        4usize =>
                            ::core::option::Option::Some(&self.obstacle_scale),
                        5usize => ::core::option::Option::Some(&self.upward_shift),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&mut self.hex_size),
                        1usize => ::core::option::Option::Some(&mut self.width),
                        2usize => ::core::option::Option::Some(&mut self.height),
                        3usize =>
                            ::core::option::Option::Some(&mut self.orientation),
                        4usize =>
                            ::core::option::Option::Some(&mut self.obstacle_scale),
                        5usize =>
                            ::core::option::Option::Some(&mut self.upward_shift),
                        _ => ::core::option::Option::None,
                    }
                }
                fn name_at(&self, index: usize)
                    -> ::core::option::Option<&str> {
                    match index {
                        0usize => ::core::option::Option::Some("hex_size"),
                        1usize => ::core::option::Option::Some("width"),
                        2usize => ::core::option::Option::Some("height"),
                        3usize => ::core::option::Option::Some("orientation"),
                        4usize => ::core::option::Option::Some("obstacle_scale"),
                        5usize => ::core::option::Option::Some("upward_shift"),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_len(&self) -> usize { 6usize }
                fn iter_fields(&self) -> bevy::reflect::FieldIter {
                    bevy::reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                    let mut dynamic: bevy::reflect::DynamicStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic.insert_boxed("hex_size",
                        bevy::reflect::Reflect::clone_value(&self.hex_size));
                    dynamic.insert_boxed("width",
                        bevy::reflect::Reflect::clone_value(&self.width));
                    dynamic.insert_boxed("height",
                        bevy::reflect::Reflect::clone_value(&self.height));
                    dynamic.insert_boxed("orientation",
                        bevy::reflect::Reflect::clone_value(&self.orientation));
                    dynamic.insert_boxed("obstacle_scale",
                        bevy::reflect::Reflect::clone_value(&self.obstacle_scale));
                    dynamic.insert_boxed("upward_shift",
                        bevy::reflect::Reflect::clone_value(&self.upward_shift));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for BoardSettings where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexOrientation: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                {
                                let name =
                                    bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::Struct::field_mut(self, name) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::Struct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::Struct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for BoardSettings where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexOrientation: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                bevy::reflect::Reflect::reflect_ref(reflect) {
                            let mut __this: Self = ::core::default::Default::default();
                            if let ::core::option::Option::Some(__field) =
                                        (||
                                                    <f32 as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "hex_size")?))() {
                                    __this.hex_size = __field;
                                }
                            if let ::core::option::Option::Some(__field) =
                                        (||
                                                    <i32 as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "width")?))() {
                                    __this.width = __field;
                                }
                            if let ::core::option::Option::Some(__field) =
                                        (||
                                                    <i32 as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "height")?))() {
                                    __this.height = __field;
                                }
                            if let ::core::option::Option::Some(__field) =
                                        (||
                                                    <HexOrientation as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "orientation")?))() {
                                    __this.orientation = __field;
                                }
                            if let ::core::option::Option::Some(__field) =
                                        (||
                                                    <f32 as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "obstacle_scale")?))() {
                                    __this.obstacle_scale = __field;
                                }
                            if let ::core::option::Option::Some(__field) =
                                        (||
                                                    <f32 as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "upward_shift")?))() {
                                    __this.upward_shift = __field;
                                }
                            ::core::option::Option::Some(__this)
                        } else { ::core::option::Option::None }
                }
            }
        };
    #[reflect(Resource)]
    pub struct Board {
        pub entities: HashMap<Hex, Entity>,
        pub layout: HexLayout,
        pub entity: Entity,
        pub bounds: BoardBounds,
        pub transform: Transform,
    }
    impl bevy::ecs::system::Resource for Board where Self: Send + Sync +
        'static {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Board {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(f, "Board",
                "entities", &self.entities, "layout", &self.layout, "entity",
                &self.entity, "bounds", &self.bounds, "transform",
                &&self.transform)
        }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for Board where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                HashMap<Hex, Entity>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexLayout: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Entity: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                BoardBounds: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Transform: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<ReflectResource>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {
                    <HashMap<Hex, Entity> as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <HexLayout as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <Entity as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <BoardBounds as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <Transform as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                }
            }
            impl bevy::reflect::Typed for Board where Self: ::core::any::Any +
                ::core::marker::Send + ::core::marker::Sync,
                HashMap<Hex, Entity>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexLayout: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Entity: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                BoardBounds: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Transform: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<HashMap<Hex,
                                                                    Entity>>("entities").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<HexLayout>("layout").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<Entity>("entity").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<BoardBounds>("bounds").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<Transform>("transform").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for Board where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str { "warband_lib::board::Board" }
                fn short_type_path() -> &'static str { "Board" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("Board")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board")
                }
            }
            impl bevy::reflect::Struct for Board where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                HashMap<Hex, Entity>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexLayout: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Entity: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                BoardBounds: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Transform: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn field(&self, name: &str)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match name {
                        "entities" => ::core::option::Option::Some(&self.entities),
                        "layout" => ::core::option::Option::Some(&self.layout),
                        "entity" => ::core::option::Option::Some(&self.entity),
                        "bounds" => ::core::option::Option::Some(&self.bounds),
                        "transform" =>
                            ::core::option::Option::Some(&self.transform),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_mut(&mut self, name: &str)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match name {
                        "entities" =>
                            ::core::option::Option::Some(&mut self.entities),
                        "layout" => ::core::option::Option::Some(&mut self.layout),
                        "entity" => ::core::option::Option::Some(&mut self.entity),
                        "bounds" => ::core::option::Option::Some(&mut self.bounds),
                        "transform" =>
                            ::core::option::Option::Some(&mut self.transform),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&self.entities),
                        1usize => ::core::option::Option::Some(&self.layout),
                        2usize => ::core::option::Option::Some(&self.entity),
                        3usize => ::core::option::Option::Some(&self.bounds),
                        4usize => ::core::option::Option::Some(&self.transform),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&mut self.entities),
                        1usize => ::core::option::Option::Some(&mut self.layout),
                        2usize => ::core::option::Option::Some(&mut self.entity),
                        3usize => ::core::option::Option::Some(&mut self.bounds),
                        4usize => ::core::option::Option::Some(&mut self.transform),
                        _ => ::core::option::Option::None,
                    }
                }
                fn name_at(&self, index: usize)
                    -> ::core::option::Option<&str> {
                    match index {
                        0usize => ::core::option::Option::Some("entities"),
                        1usize => ::core::option::Option::Some("layout"),
                        2usize => ::core::option::Option::Some("entity"),
                        3usize => ::core::option::Option::Some("bounds"),
                        4usize => ::core::option::Option::Some("transform"),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_len(&self) -> usize { 5usize }
                fn iter_fields(&self) -> bevy::reflect::FieldIter {
                    bevy::reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                    let mut dynamic: bevy::reflect::DynamicStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic.insert_boxed("entities",
                        bevy::reflect::Reflect::clone_value(&self.entities));
                    dynamic.insert_boxed("layout",
                        bevy::reflect::Reflect::clone_value(&self.layout));
                    dynamic.insert_boxed("entity",
                        bevy::reflect::Reflect::clone_value(&self.entity));
                    dynamic.insert_boxed("bounds",
                        bevy::reflect::Reflect::clone_value(&self.bounds));
                    dynamic.insert_boxed("transform",
                        bevy::reflect::Reflect::clone_value(&self.transform));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for Board where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                HashMap<Hex, Entity>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexLayout: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Entity: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                BoardBounds: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Transform: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                {
                                let name =
                                    bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::Struct::field_mut(self, name) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::Struct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::Struct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for Board where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync,
                HashMap<Hex, Entity>: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexLayout: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Entity: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                BoardBounds: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                Transform: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {
                                    entities: (||
                                                    <HashMap<Hex, Entity> as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "entities")?))()?,
                                    layout: (||
                                                    <HexLayout as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "layout")?))()?,
                                    entity: (||
                                                    <Entity as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "entity")?))()?,
                                    bounds: (||
                                                    <BoardBounds as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "bounds")?))()?,
                                    transform: (||
                                                    <Transform as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "transform")?))()?,
                                })
                        } else { ::core::option::Option::None }
                }
            }
        };
    impl Board {
        pub fn cells(&self) -> impl Iterator<Item = (Hex, Entity)> + '_ {
            self.entities.iter().map(|(hex, entity)| (*hex, *entity))
        }
    }
    impl FromWorld for Board {
        fn from_world(world: &mut World) -> Self {
            let entity =
                world.spawn((Name::unit("board"),
                            SpatialBundle { ..default() }, BoardHolder)).id();
            Self {
                entities: HashMap::new(),
                layout: HexLayout::default(),
                entity,
                bounds: BoardBounds::default(),
                transform: Transform::default(),
            }
        }
    }
    pub struct BoardBounds {
        half_width: i32,
        half_height: i32,
        orientation: HexOrientation,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BoardBounds {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(f,
                "BoardBounds", "half_width", &self.half_width, "half_height",
                &self.half_height, "orientation", &&self.orientation)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for BoardBounds {
        #[inline]
        fn default() -> BoardBounds {
            BoardBounds {
                half_width: ::core::default::Default::default(),
                half_height: ::core::default::Default::default(),
                orientation: ::core::default::Default::default(),
            }
        }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for BoardBounds where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, i32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexOrientation: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {
                    <i32 as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <i32 as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    <HexOrientation as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                }
            }
            impl bevy::reflect::Typed for BoardBounds where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, i32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexOrientation: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<i32>("half_width").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<i32>("half_height").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                        bevy::reflect::NamedField::new::<HexOrientation>("orientation").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for BoardBounds where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str {
                    "warband_lib::board::BoardBounds"
                }
                fn short_type_path() -> &'static str { "BoardBounds" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("BoardBounds")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board")
                }
            }
            impl bevy::reflect::Struct for BoardBounds where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, i32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexOrientation: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn field(&self, name: &str)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match name {
                        "half_width" =>
                            ::core::option::Option::Some(&self.half_width),
                        "half_height" =>
                            ::core::option::Option::Some(&self.half_height),
                        "orientation" =>
                            ::core::option::Option::Some(&self.orientation),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_mut(&mut self, name: &str)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match name {
                        "half_width" =>
                            ::core::option::Option::Some(&mut self.half_width),
                        "half_height" =>
                            ::core::option::Option::Some(&mut self.half_height),
                        "orientation" =>
                            ::core::option::Option::Some(&mut self.orientation),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&self.half_width),
                        1usize => ::core::option::Option::Some(&self.half_height),
                        2usize => ::core::option::Option::Some(&self.orientation),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index {
                        0usize =>
                            ::core::option::Option::Some(&mut self.half_width),
                        1usize =>
                            ::core::option::Option::Some(&mut self.half_height),
                        2usize =>
                            ::core::option::Option::Some(&mut self.orientation),
                        _ => ::core::option::Option::None,
                    }
                }
                fn name_at(&self, index: usize)
                    -> ::core::option::Option<&str> {
                    match index {
                        0usize => ::core::option::Option::Some("half_width"),
                        1usize => ::core::option::Option::Some("half_height"),
                        2usize => ::core::option::Option::Some("orientation"),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_len(&self) -> usize { 3usize }
                fn iter_fields(&self) -> bevy::reflect::FieldIter {
                    bevy::reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                    let mut dynamic: bevy::reflect::DynamicStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic.insert_boxed("half_width",
                        bevy::reflect::Reflect::clone_value(&self.half_width));
                    dynamic.insert_boxed("half_height",
                        bevy::reflect::Reflect::clone_value(&self.half_height));
                    dynamic.insert_boxed("orientation",
                        bevy::reflect::Reflect::clone_value(&self.orientation));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for BoardBounds where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, i32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexOrientation: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                {
                                let name =
                                    bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::Struct::field_mut(self, name) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::Struct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::Struct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for BoardBounds where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, i32: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                i32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection,
                HexOrientation: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {
                                    half_width: (||
                                                    <i32 as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "half_width")?))()?,
                                    half_height: (||
                                                    <i32 as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "half_height")?))()?,
                                    orientation: (||
                                                    <HexOrientation as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                "orientation")?))()?,
                                })
                        } else { ::core::option::Option::None }
                }
            }
        };
    impl BoardBounds {
        fn new(width: i32, height: i32, orientation: HexOrientation) -> Self {
            let half_width = width / 2;
            let half_height = height / 2;
            Self { half_width, half_height, orientation }
        }
        fn is_in_bounds(&self, hex: Hex) -> bool {
            let (x, y) = axial_to_xy(hex, self.orientation);
            x >= -self.half_width && x <= self.half_width &&
                    y >= -self.half_height && y <= self.half_height
        }
    }
    #[inline]
    fn axial_to_xy(hex: Hex, orientation: HexOrientation) -> (i32, i32) {
        let q = hex.x();
        let r = hex.y();
        match orientation {
            HexOrientation::Flat => (q, r + (q - (q & 1)) / 2),
            HexOrientation::Pointy => (q + (r - (r & 1)) / 2, r),
        }
    }
    pub struct BoardHolder;
    impl bevy::ecs::component::Component for BoardHolder where Self: Send +
        Sync + 'static {
        const STORAGE_TYPE: bevy::ecs::component::StorageType =
            bevy::ecs::component::StorageType::Table;
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BoardHolder {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "BoardHolder")
        }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for BoardHolder where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {}
            }
            impl bevy::reflect::Typed for BoardHolder where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for BoardHolder where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str {
                    "warband_lib::board::BoardHolder"
                }
                fn short_type_path() -> &'static str { "BoardHolder" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("BoardHolder")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board")
                }
            }
            impl bevy::reflect::Struct for BoardHolder where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn field(&self, name: &str)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match name { _ => ::core::option::Option::None, }
                }
                fn field_mut(&mut self, name: &str)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match name { _ => ::core::option::Option::None, }
                }
                fn field_at(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn field_at_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn name_at(&self, index: usize)
                    -> ::core::option::Option<&str> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn field_len(&self) -> usize { 0usize }
                fn iter_fields(&self) -> bevy::reflect::FieldIter {
                    bevy::reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                    let mut dynamic: bevy::reflect::DynamicStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for BoardHolder where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                {
                                let name =
                                    bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::Struct::field_mut(self, name) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::Struct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::Struct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for BoardHolder where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {})
                        } else { ::core::option::Option::None }
                }
            }
        };
    pub struct Cell;
    impl bevy::ecs::component::Component for Cell where Self: Send + Sync +
        'static {
        const STORAGE_TYPE: bevy::ecs::component::StorageType =
            bevy::ecs::component::StorageType::Table;
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Cell {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Cell")
        }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for Cell where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {}
            }
            impl bevy::reflect::Typed for Cell where Self: ::core::any::Any +
                ::core::marker::Send + ::core::marker::Sync {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for Cell where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str { "warband_lib::board::Cell" }
                fn short_type_path() -> &'static str { "Cell" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("Cell")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board")
                }
            }
            impl bevy::reflect::Struct for Cell where Self: ::core::any::Any +
                ::core::marker::Send + ::core::marker::Sync {
                fn field(&self, name: &str)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match name { _ => ::core::option::Option::None, }
                }
                fn field_mut(&mut self, name: &str)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match name { _ => ::core::option::Option::None, }
                }
                fn field_at(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn field_at_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn name_at(&self, index: usize)
                    -> ::core::option::Option<&str> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn field_len(&self) -> usize { 0usize }
                fn iter_fields(&self) -> bevy::reflect::FieldIter {
                    bevy::reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                    let mut dynamic: bevy::reflect::DynamicStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for Cell where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                {
                                let name =
                                    bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::Struct::field_mut(self, name) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::Struct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::Struct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for Cell where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {})
                        } else { ::core::option::Option::None }
                }
            }
        };
    #[reflect(Component)]
    pub struct Hovered;
    impl bevy::ecs::component::Component for Hovered where Self: Send + Sync +
        'static {
        const STORAGE_TYPE: bevy::ecs::component::StorageType =
            bevy::ecs::component::StorageType::Table;
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Hovered {
        #[inline]
        fn clone(&self) -> Hovered { *self }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Hovered { }
    #[automatically_derived]
    impl ::core::default::Default for Hovered {
        #[inline]
        fn default() -> Hovered { Hovered {} }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Hovered {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Hovered")
        }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for Hovered where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {}
            }
            impl bevy::reflect::Typed for Hovered where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for Hovered where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str {
                    "warband_lib::board::Hovered"
                }
                fn short_type_path() -> &'static str { "Hovered" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("Hovered")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board")
                }
            }
            impl bevy::reflect::Struct for Hovered where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn field(&self, name: &str)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match name { _ => ::core::option::Option::None, }
                }
                fn field_mut(&mut self, name: &str)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match name { _ => ::core::option::Option::None, }
                }
                fn field_at(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn field_at_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn name_at(&self, index: usize)
                    -> ::core::option::Option<&str> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn field_len(&self) -> usize { 0usize }
                fn iter_fields(&self) -> bevy::reflect::FieldIter {
                    bevy::reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                    let mut dynamic: bevy::reflect::DynamicStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for Hovered where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                {
                                let name =
                                    bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::Struct::field_mut(self, name) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::Struct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::Struct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for Hovered where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {})
                        } else { ::core::option::Option::None }
                }
            }
        };
    pub struct BoardBuiltEvent;
    impl bevy::ecs::event::Event for BoardBuiltEvent where Self: Send + Sync +
        'static {}
    impl bevy::ecs::component::Component for BoardBuiltEvent where
        Self: Send + Sync + 'static {
        const STORAGE_TYPE: bevy::ecs::component::StorageType =
            bevy::ecs::component::StorageType::SparseSet;
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BoardBuiltEvent {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "BoardBuiltEvent")
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for BoardBuiltEvent {
        #[inline]
        fn default() -> BoardBuiltEvent { BoardBuiltEvent {} }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for BoardBuiltEvent where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {}
            }
            impl bevy::reflect::Typed for BoardBuiltEvent where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for BoardBuiltEvent where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str {
                    "warband_lib::board::BoardBuiltEvent"
                }
                fn short_type_path() -> &'static str { "BoardBuiltEvent" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("BoardBuiltEvent")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::board")
                }
            }
            impl bevy::reflect::Struct for BoardBuiltEvent where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn field(&self, name: &str)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match name { _ => ::core::option::Option::None, }
                }
                fn field_mut(&mut self, name: &str)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match name { _ => ::core::option::Option::None, }
                }
                fn field_at(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn field_at_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn name_at(&self, index: usize)
                    -> ::core::option::Option<&str> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn field_len(&self) -> usize { 0usize }
                fn iter_fields(&self) -> bevy::reflect::FieldIter {
                    bevy::reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                    let mut dynamic: bevy::reflect::DynamicStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for BoardBuiltEvent where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                {
                                let name =
                                    bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::Struct::field_mut(self, name) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::Struct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::Struct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for BoardBuiltEvent where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {})
                        } else { ::core::option::Option::None }
                }
            }
        };
    fn build(mut commands: Commands, board_settings: Res<BoardSettings>,
        mut board: ResMut<Board>, mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut event_writer: EventWriter<BoardBuiltEvent>) {
        if !board_settings.is_changed() { return; }
        board.entities.iter_mut().for_each(|(_, entity)|
                { commands.entity(*entity).despawn_recursive(); });
        let layout =
            HexLayout {
                hex_size: Vec2::splat(board_settings.hex_size),
                orientation: board_settings.orientation,
                ..default()
            };
        let default_material = materials.add(Color::WHITE);
        let mesh = circle_column(&layout);
        let mesh_handle = meshes.add(mesh);
        let half_width = board_settings.width / 2;
        let half_height = board_settings.height / 2;
        let corners = [-half_width, half_width, -half_height, half_height];
        let entities: HashMap<_, _> =
            shapes::flat_rectangle(corners).map(|hex|
                        {
                            let pos = layout.hex_to_world_pos(hex);
                            let id =
                                commands.spawn((Name::new(::alloc::__export::must_use({
                                                        let res = ::alloc::fmt::format(format_args!("{0:?}", hex));
                                                        res
                                                    })),
                                            PbrBundle {
                                                transform: Transform::from_xyz(pos.x, 0.0, pos.y),
                                                mesh: mesh_handle.clone(),
                                                material: default_material.clone_weak(),
                                                ..default()
                                            }, Cell, Location::default(), PickableBundle::default(),
                                            On::<Pointer<Over>>::target_insert(Hovered),
                                            On::<Pointer<Out>>::target_remove::<Hovered>())).id();
                            (hex, id)
                        }).collect();
        commands.entity(board.entity).push_children(&entities.values().copied().collect::<Vec<_>>());
        board.entities = entities;
        board.layout = layout;
        board.bounds =
            BoardBounds::new(board_settings.width, board_settings.height,
                board_settings.orientation);
        board.transform.translation =
            Vec3::new(board.layout.origin.x, 0.0, board.layout.origin.y);
        board.transform.rotation = Quat::IDENTITY;
        event_writer.send(BoardBuiltEvent);
    }
    fn circle_column(hex_layout: &HexLayout) -> Mesh {
        Cylinder {
                radius: hex_layout.hex_size.x * 0.75,
                half_height: f32::EPSILON,
            }.into()
    }
}
mod core {
    use crate::{prelude::*, AppState};
    pub mod camera_driver {
        use bevy::{
            ecs::{intern::Interned, schedule::ScheduleLabel},
            transform::TransformSystem,
        };
        use crate::prelude::*;
        pub enum CameraDriverSystems { Reset, Drivers, Apply, }
        impl bevy::ecs::schedule::SystemSet for CameraDriverSystems where
            Self: 'static + Send + Sync + Clone + Eq + ::std::fmt::Debug +
            ::std::hash::Hash {
            fn dyn_clone(&self)
                -> ::std::boxed::Box<dyn bevy::ecs::schedule::SystemSet> {
                ::std::boxed::Box::new(::std::clone::Clone::clone(self))
            }
            fn as_dyn_eq(&self) -> &dyn bevy::ecs::schedule::DynEq { self }
            fn dyn_hash(&self, mut state: &mut dyn ::std::hash::Hasher) {
                let ty_id = ::std::any::TypeId::of::<Self>();
                ::std::hash::Hash::hash(&ty_id, &mut state);
                ::std::hash::Hash::hash(self, &mut state);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CameraDriverSystems {
            #[inline]
            fn clone(&self) -> CameraDriverSystems { *self }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for CameraDriverSystems { }
        #[automatically_derived]
        impl ::core::fmt::Debug for CameraDriverSystems {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f,
                    match self {
                        CameraDriverSystems::Reset => "Reset",
                        CameraDriverSystems::Drivers => "Drivers",
                        CameraDriverSystems::Apply => "Apply",
                    })
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CameraDriverSystems { }
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CameraDriverSystems {
            #[inline]
            fn eq(&self, other: &CameraDriverSystems) -> bool {
                let __self_discr =
                    ::core::intrinsics::discriminant_value(self);
                let __arg1_discr =
                    ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for CameraDriverSystems {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for CameraDriverSystems {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr =
                    ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        pub(super) fn plugin(schedule: impl ScheduleLabel)
            -> CameraDriverPlugin {
            CameraDriverPlugin::builder().schedule(schedule).build()
        }
        pub(super) struct CameraDriverPlugin {
            schedule: Interned<dyn ScheduleLabel>,
        }
        #[doc(hidden)]
        type __CameraDriverPluginBuilderInitialState =
            (::bon::private::Unset<::bon::private::Required>,);
        #[must_use =
        "the builder does nothing until you call `build()` on it to finish building"]
        #[doc =
        "Use builder syntax to set the required parameters and finish by calling the method [`Self::build()`]."]
        #[allow(unused_parens)]
        #[allow(clippy :: struct_field_names, clippy :: type_complexity)]
        struct CameraDriverPluginBuilder<__0, ___State =
            __CameraDriverPluginBuilderInitialState> where
            __0: ScheduleLabel {
            #[doc =
            "Please don't touch this field. It's an implementation detail that is exempt from the API stability guarantees. This field couldn't be hidden using Rust's privacy syntax. The details about this are described in [the blog post](https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).\n        "]
            __private_phantom: ::core::marker::PhantomData<(::core::marker::PhantomData<CameraDriverPlugin>,
            ::core::marker::PhantomData<__0>,
            ::core::marker::PhantomData<__0>,
            ::core::marker::PhantomData<___State>)>,
            #[doc =
            "Please don't touch this field. It's an implementation detail that is exempt from the API stability guarantees. This field couldn't be hidden using Rust's privacy syntax. The details about this are described in [the blog post](https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).\n        "]
            __private_members: ___State,
        }
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        struct CameraDriverPluginBuilder__schedule;
        #[allow(unused_parens)]
        #[automatically_derived]
        impl<__0, __Schedule> CameraDriverPluginBuilder<__0, (__Schedule,)>
            where __0: ScheduleLabel {
            #[doc = "Finishes building and performs the requested action."]
            #[inline(always)]
            #[allow(clippy :: inline_always)]
            fn build(self) -> CameraDriverPlugin where
                __Schedule: ::bon::private::IntoSet<__0,
                CameraDriverPluginBuilder__schedule> {
                let schedule: __0 =
                    ::bon::private::IntoSet::<__0,
                            CameraDriverPluginBuilder__schedule>::into_set(self.__private_members.0);
                <CameraDriverPlugin>::__orig_new::<>(schedule)
            }
            #[doc =
            "Sets the value of `schedule`. See [`CameraDriverPlugin::builder()`] for more info."]
            #[allow(clippy :: inline_always, clippy :: impl_trait_in_params)]
            #[inline(always)]
            fn schedule(self, value: __0)
                -> CameraDriverPluginBuilder<__0, (::bon::private::Set<__0>,)>
                where __Schedule: ::bon::private::IsUnset {
                CameraDriverPluginBuilder {
                    __private_phantom: ::core::marker::PhantomData,
                    __private_members: (::bon::private::Set(value),),
                }
            }
        }
        impl CameraDriverPlugin {
            #[inline(always)]
            #[allow(clippy :: inline_always, clippy :: use_self, clippy ::
            missing_const_for_fn)]
            fn builder<__0>() -> CameraDriverPluginBuilder<__0> where
                __0: ScheduleLabel {
                CameraDriverPluginBuilder {
                    __private_phantom: ::core::marker::PhantomData,
                    __private_members: (::bon::private::Unset(::bon::private::Required),),
                }
            }
            #[doc =
            "Positional function equivalent of [`CameraDriverPlugin::new()`].\nSee its docs for details."]
            #[doc(hidden)]
            #[allow(clippy :: too_many_arguments, clippy ::
            fn_params_excessive_bools,)]
            fn __orig_new(schedule: impl ScheduleLabel) -> Self {
                Self { schedule: schedule.intern() }
            }
        }
        impl Default for CameraDriverPlugin {
            fn default() -> Self {
                Self::builder().schedule(PostUpdate).build()
            }
        }
        impl Plugin for CameraDriverPlugin {
            fn build(&self, app: &mut App) {
                app.register_type::<RigTransform>();
                app.register_type::<YawPitch>();
                app.register_type::<Offset>();
                app.register_type::<Follow>();
                app.register_type::<Zoom>();
                app.register_type::<Smoothing>();
                ;
                app.configure_sets(self.schedule,
                    (CameraDriverSystems::Reset, CameraDriverSystems::Drivers,
                                CameraDriverSystems::Apply).before(TransformSystem::TransformPropagate).chain());
                app.add_systems(self.schedule,
                    (reset_rig_transform.in_set(CameraDriverSystems::Reset),
                        (driver_yaw_pitch, driver_follow,
                                driver_offset.after(driver_follow),
                                driver_zoom).in_set(CameraDriverSystems::Drivers),
                        sync_rig_transform.in_set(CameraDriverSystems::Apply)));
            }
        }
        fn reset_rig_transform(mut rig: Query<&mut RigTransform>) {
            for mut rig_transform in &mut rig { rig_transform.reset(); }
        }
        #[reflect(Component)]
        pub struct RigTransform {
            pub translation: Vec3,
            pub rotation: Quat,
            /// Only used for orthographic cameras
            pub zoom: Option<f32>,
        }
        impl bevy::ecs::component::Component for RigTransform where
            Self: Send + Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for RigTransform where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Vec3: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Quat: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Option<f32>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <Vec3 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                        <Quat as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                        <Option<f32> as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for RigTransform where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Vec3: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Quat: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Option<f32>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<Vec3>("translation").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                            bevy::reflect::NamedField::new::<Quat>("rotation").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                            bevy::reflect::NamedField::new::<Option<f32>>("zoom").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for RigTransform where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::core::camera_driver::RigTransform"
                    }
                    fn short_type_path() -> &'static str { "RigTransform" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("RigTransform")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver")
                    }
                }
                impl bevy::reflect::Struct for RigTransform where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Vec3: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Quat: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Option<f32>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, name: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match name {
                            "translation" =>
                                ::core::option::Option::Some(&self.translation),
                            "rotation" => ::core::option::Option::Some(&self.rotation),
                            "zoom" => ::core::option::Option::Some(&self.zoom),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, name: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match name {
                            "translation" =>
                                ::core::option::Option::Some(&mut self.translation),
                            "rotation" =>
                                ::core::option::Option::Some(&mut self.rotation),
                            "zoom" => ::core::option::Option::Some(&mut self.zoom),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_at(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.translation),
                            1usize => ::core::option::Option::Some(&self.rotation),
                            2usize => ::core::option::Option::Some(&self.zoom),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_at_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize =>
                                ::core::option::Option::Some(&mut self.translation),
                            1usize => ::core::option::Option::Some(&mut self.rotation),
                            2usize => ::core::option::Option::Some(&mut self.zoom),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn name_at(&self, index: usize)
                        -> ::core::option::Option<&str> {
                        match index {
                            0usize => ::core::option::Option::Some("translation"),
                            1usize => ::core::option::Option::Some("rotation"),
                            2usize => ::core::option::Option::Some("zoom"),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_len(&self) -> usize { 3usize }
                    fn iter_fields(&self) -> bevy::reflect::FieldIter {
                        bevy::reflect::FieldIter::new(self)
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                        let mut dynamic: bevy::reflect::DynamicStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed("translation",
                            bevy::reflect::Reflect::clone_value(&self.translation));
                        dynamic.insert_boxed("rotation",
                            bevy::reflect::Reflect::clone_value(&self.rotation));
                        dynamic.insert_boxed("zoom",
                            bevy::reflect::Reflect::clone_value(&self.zoom));
                        dynamic
                    }
                }
                impl bevy::reflect::Reflect for RigTransform where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Vec3: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Quat: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Option<f32>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                    bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                    {
                                    let name =
                                        bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::Struct::field_mut(self, name) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::Struct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Struct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Struct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Struct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Struct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::struct_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for RigTransform where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Vec3: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Quat: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Option<f32>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                    bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        translation: (||
                                                        <Vec3 as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "translation")?))()?,
                                        rotation: (||
                                                        <Quat as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "rotation")?))()?,
                                        zoom: (||
                                                        <Option<f32> as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "zoom")?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[automatically_derived]
        impl ::core::default::Default for RigTransform {
            #[inline]
            fn default() -> RigTransform {
                RigTransform {
                    translation: ::core::default::Default::default(),
                    rotation: ::core::default::Default::default(),
                    zoom: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for RigTransform { }
        #[automatically_derived]
        impl ::core::clone::Clone for RigTransform {
            #[inline]
            fn clone(&self) -> RigTransform {
                let _: ::core::clone::AssertParamIsClone<Vec3>;
                let _: ::core::clone::AssertParamIsClone<Quat>;
                let _: ::core::clone::AssertParamIsClone<Option<f32>>;
                *self
            }
        }
        #[reflect(Component)]
        pub struct YawPitch {
            pub yaw: f32,
            pub pitch: f32,
        }
        impl bevy::ecs::component::Component for YawPitch where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for YawPitch where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <f32 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                        <f32 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for YawPitch where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<f32>("yaw").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                            bevy::reflect::NamedField::new::<f32>("pitch").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for YawPitch where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::core::camera_driver::YawPitch"
                    }
                    fn short_type_path() -> &'static str { "YawPitch" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("YawPitch")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver")
                    }
                }
                impl bevy::reflect::Struct for YawPitch where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, name: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match name {
                            "yaw" => ::core::option::Option::Some(&self.yaw),
                            "pitch" => ::core::option::Option::Some(&self.pitch),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, name: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match name {
                            "yaw" => ::core::option::Option::Some(&mut self.yaw),
                            "pitch" => ::core::option::Option::Some(&mut self.pitch),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_at(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.yaw),
                            1usize => ::core::option::Option::Some(&self.pitch),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_at_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.yaw),
                            1usize => ::core::option::Option::Some(&mut self.pitch),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn name_at(&self, index: usize)
                        -> ::core::option::Option<&str> {
                        match index {
                            0usize => ::core::option::Option::Some("yaw"),
                            1usize => ::core::option::Option::Some("pitch"),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_len(&self) -> usize { 2usize }
                    fn iter_fields(&self) -> bevy::reflect::FieldIter {
                        bevy::reflect::FieldIter::new(self)
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                        let mut dynamic: bevy::reflect::DynamicStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed("yaw",
                            bevy::reflect::Reflect::clone_value(&self.yaw));
                        dynamic.insert_boxed("pitch",
                            bevy::reflect::Reflect::clone_value(&self.pitch));
                        dynamic
                    }
                }
                impl bevy::reflect::Reflect for YawPitch where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                    bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                    {
                                    let name =
                                        bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::Struct::field_mut(self, name) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::Struct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Struct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Struct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Struct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Struct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::struct_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for YawPitch where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                    bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        yaw: (||
                                                        <f32 as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "yaw")?))()?,
                                        pitch: (||
                                                        <f32 as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "pitch")?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[automatically_derived]
        impl ::core::marker::Copy for YawPitch { }
        #[automatically_derived]
        impl ::core::clone::Clone for YawPitch {
            #[inline]
            fn clone(&self) -> YawPitch {
                let _: ::core::clone::AssertParamIsClone<f32>;
                *self
            }
        }
        impl Default for YawPitch {
            fn default() -> Self { Self { yaw: 0.0, pitch: 0.0 } }
        }
        #[allow(unused)]
        impl YawPitch {
            pub fn with_yaw_pitch(yaw: f32, pitch: f32) -> Self {
                Self { yaw, pitch }
            }
            pub fn yaw(mut self, yaw: f32) { self.yaw = yaw; }
            pub fn pitch(mut self, pitch: f32) { self.pitch = pitch; }
            pub fn rotate_yaw(&mut self, yaw: f32) {
                self.yaw = (self.yaw + yaw) % 720_f32;
            }
            pub fn rotate_pitch(&mut self, pitch: f32) {
                self.pitch = (self.pitch + pitch).clamp(-90.0, 90.0);
            }
            pub fn rotate(&mut self, yaw: f32, pitch: f32) {
                self.rotate_yaw(yaw);
                self.rotate_pitch(pitch);
            }
        }
        fn driver_yaw_pitch(mut rig: Query<(&mut RigTransform, &YawPitch)>) {
            for (mut rig_transform, yaw_pitch) in &mut rig {
                let rotation =
                    Quat::from_euler(EulerRot::YXZ, yaw_pitch.yaw.to_radians(),
                        yaw_pitch.pitch.to_radians(), 0.0);
                rig_transform.rotation = rotation;
            }
        }
        #[reflect(Component)]
        pub struct Offset(pub Vec3);
        impl bevy::ecs::component::Component for Offset where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Offset where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Vec3: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <Vec3 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Offset where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Vec3: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[bevy::reflect::UnnamedField::new::<Vec3>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Offset where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::core::camera_driver::Offset"
                    }
                    fn short_type_path() -> &'static str { "Offset" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Offset")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver")
                    }
                }
                impl bevy::reflect::TupleStruct for Offset where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Vec3: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    #[inline]
                    fn field_len(&self) -> usize { 1usize }
                    #[inline]
                    fn iter_fields(&self)
                        -> bevy::reflect::TupleStructFieldIter {
                        bevy::reflect::TupleStructFieldIter::new(self)
                    }
                    fn clone_dynamic(&self)
                        -> bevy::reflect::DynamicTupleStruct {
                        let mut dynamic: bevy::reflect::DynamicTupleStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed(bevy::reflect::Reflect::clone_value(&self.0));
                        dynamic
                    }
                }
                impl bevy::reflect::Reflect for Offset where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Vec3: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                    = bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                    {
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::TupleStruct::field_mut(self, i) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::TupleStruct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::TupleStruct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::tuple_struct_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Offset where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Vec3: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                    = bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        0: (||
                                                        <Vec3 as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::TupleStruct::field(__ref_struct,
                                                                    0)?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[automatically_derived]
        impl ::core::marker::Copy for Offset { }
        #[automatically_derived]
        impl ::core::clone::Clone for Offset {
            #[inline]
            fn clone(&self) -> Offset {
                let _: ::core::clone::AssertParamIsClone<Vec3>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Offset {
            #[inline]
            fn default() -> Offset {
                Offset(::core::default::Default::default())
            }
        }
        fn driver_offset(mut rig: Query<(&mut RigTransform, &Offset)>) {
            for (mut rig_transform, offset) in &mut rig {
                rig_transform.translation += offset.0;
            }
        }
        #[reflect(Component)]
        pub enum Follow {
            Entity(Entity),
            Position(Vec3),

            #[default]
            None,
        }
        impl bevy::ecs::component::Component for Follow where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Follow where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Vec3: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <Entity as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                        <Vec3 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Follow where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Vec3: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Enum(bevy::reflect::EnumInfo::new::<Self>(&[bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("Entity",
                                                                        &[bevy::reflect::UnnamedField::new::<Entity>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                            bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("Position",
                                                                        &[bevy::reflect::UnnamedField::new::<Vec3>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                            bevy::reflect::VariantInfo::Unit(bevy::reflect::UnitVariantInfo::new("None").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Follow where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::core::camera_driver::Follow"
                    }
                    fn short_type_path() -> &'static str { "Follow" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Follow")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver")
                    }
                }
                impl bevy::reflect::Enum for Follow where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Vec3: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, __name_param: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at(&self, __index_param: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self {
                            Follow::Entity { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            Follow::Position { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, __name_param: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at_mut(&mut self, __index_param: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self {
                            Follow::Entity { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            Follow::Position { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn index_of(&self, __name_param: &str)
                        -> ::core::option::Option<usize> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn name_at(&self, __index_param: usize)
                        -> ::core::option::Option<&str> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn iter_fields(&self) -> bevy::reflect::VariantFieldIter {
                        bevy::reflect::VariantFieldIter::new(self)
                    }
                    #[inline]
                    fn field_len(&self) -> usize {
                        match self {
                            Follow::Entity { .. } => 1usize,
                            Follow::Position { .. } => 1usize,
                            Follow::None { .. } => 0usize,
                            _ => 0,
                        }
                    }
                    #[inline]
                    fn variant_name(&self) -> &str {
                        match self {
                            Follow::Entity { .. } => "Entity",
                            Follow::Position { .. } => "Position",
                            Follow::None { .. } => "None",
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_index(&self) -> usize {
                        match self {
                            Follow::Entity { .. } => 0usize,
                            Follow::Position { .. } => 1usize,
                            Follow::None { .. } => 2usize,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_type(&self) -> bevy::reflect::VariantType {
                        match self {
                            Follow::Entity { .. } => bevy::reflect::VariantType::Tuple,
                            Follow::Position { .. } =>
                                bevy::reflect::VariantType::Tuple,
                            Follow::None { .. } => bevy::reflect::VariantType::Unit,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicEnum {
                        bevy::reflect::DynamicEnum::from_ref::<Self>(self)
                    }
                }
                impl bevy::reflect::Reflect for Follow where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Vec3: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Enum::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        __value_param:
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(__value_param)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self,
                        __value_param: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Enum(__value_param) =
                                    bevy::reflect::Reflect::reflect_ref(__value_param) {
                                if bevy::reflect::Enum::variant_name(self) ==
                                            bevy::reflect::Enum::variant_name(__value_param) {
                                        match bevy::reflect::Enum::variant_type(__value_param) {
                                            bevy::reflect::VariantType::Struct => {
                                                for field in bevy::reflect::Enum::iter_fields(__value_param)
                                                    {
                                                    let name = field.name().unwrap();
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_mut(self, name) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            bevy::reflect::VariantType::Tuple => {
                                                for (index, field) in
                                                    ::core::iter::Iterator::enumerate(bevy::reflect::Enum::iter_fields(__value_param))
                                                    {
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_at_mut(self, index) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            _ => {}
                                        }
                                    } else {
                                       match bevy::reflect::Enum::variant_name(__value_param) {
                                           "Entity" => {
                                               *self =
                                                   Follow::Entity {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("Entity"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <Entity as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<Entity as
                                                                                   bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           "Position" => {
                                               *self =
                                                   Follow::Position {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("Position"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <Vec3 as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<Vec3 as
                                                                                   bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           "None" => { *self = Follow::None {} }
                                           name => {
                                               return ::core::result::Result::Err(bevy::reflect::ApplyError::UnknownVariant {
                                                           enum_name: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(self)),
                                                           variant_name: ::core::convert::Into::into(name),
                                                       });
                                           }
                                       }
                                   }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(__value_param),
                                           to_kind: bevy::reflect::ReflectKind::Enum,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Enum
                    }
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Enum(self)
                    }
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Enum(self)
                    }
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Enum(self)
                    }
                    fn reflect_hash(&self) -> ::core::option::Option<u64> {
                        bevy::reflect::enum_hash(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::enum_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Follow where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Vec3: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(__param0: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Enum(__param0) =
                                    bevy::reflect::Reflect::reflect_ref(__param0) {
                                match bevy::reflect::Enum::variant_name(__param0) {
                                    "Entity" =>
                                        ::core::option::Option::Some(Follow::Entity {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <Entity as bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    "Position" =>
                                        ::core::option::Option::Some(Follow::Position {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <Vec3 as bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    "None" => ::core::option::Option::Some(Follow::None {}),
                                    name => {
                                        ::core::panicking::panic_fmt(format_args!("variant with name `{0}` does not exist on enum `{1}`",
                                                name, <Self as bevy::reflect::TypePath>::type_path()));
                                    }
                                }
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[automatically_derived]
        impl ::core::marker::Copy for Follow { }
        #[automatically_derived]
        impl ::core::clone::Clone for Follow {
            #[inline]
            fn clone(&self) -> Follow {
                let _: ::core::clone::AssertParamIsClone<Entity>;
                let _: ::core::clone::AssertParamIsClone<Vec3>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Follow {
            #[inline]
            fn default() -> Follow { Self::None }
        }
        fn driver_follow(mut rig: Query<(&mut RigTransform, &Follow)>,
            transforms: Query<&Transform, Without<Follow>>) {
            for (mut rig_transform, follow) in &mut rig {
                match follow {
                    Follow::Entity(entity) => {
                        if let Ok(transform) = transforms.get(*entity) {
                                rig_transform.translation = transform.translation;
                            }
                    }
                    Follow::Position(position) => {
                        rig_transform.translation = *position;
                    }
                    _ => {}
                }
            }
        }
        #[reflect(Component)]
        pub struct Zoom(f32);
        impl bevy::ecs::component::Component for Zoom where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Zoom where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <f32 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Zoom where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[bevy::reflect::UnnamedField::new::<f32>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Zoom where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::core::camera_driver::Zoom"
                    }
                    fn short_type_path() -> &'static str { "Zoom" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Zoom")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver")
                    }
                }
                impl bevy::reflect::TupleStruct for Zoom where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    #[inline]
                    fn field_len(&self) -> usize { 1usize }
                    #[inline]
                    fn iter_fields(&self)
                        -> bevy::reflect::TupleStructFieldIter {
                        bevy::reflect::TupleStructFieldIter::new(self)
                    }
                    fn clone_dynamic(&self)
                        -> bevy::reflect::DynamicTupleStruct {
                        let mut dynamic: bevy::reflect::DynamicTupleStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed(bevy::reflect::Reflect::clone_value(&self.0));
                        dynamic
                    }
                }
                impl bevy::reflect::Reflect for Zoom where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                    = bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                    {
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::TupleStruct::field_mut(self, i) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::TupleStruct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::TupleStruct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::tuple_struct_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Zoom where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                    = bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        0: (||
                                                        <f32 as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::TupleStruct::field(__ref_struct,
                                                                    0)?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[automatically_derived]
        impl ::core::marker::Copy for Zoom { }
        #[automatically_derived]
        impl ::core::clone::Clone for Zoom {
            #[inline]
            fn clone(&self) -> Zoom {
                let _: ::core::clone::AssertParamIsClone<f32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Zoom {
            #[inline]
            fn default() -> Zoom { Zoom(::core::default::Default::default()) }
        }
        impl Zoom {
            pub fn with_zoom(zoom: f32) -> Self { Self(zoom) }
            pub fn zoom(&self) -> f32 { self.0 }
            #[allow(unused)]
            pub fn delta_zoom(&mut self, zoom: f32) {
                self.set_zoom(self.0 + zoom);
            }
            pub fn set_zoom(&mut self, zoom: f32) {
                self.0 = zoom;
                self.0 = self.0.max(0.0);
            }
        }
        fn driver_zoom(mut rig: Query<(&mut RigTransform, &Zoom)>) {
            for (mut rig_transform, zoom) in &mut rig {
                rig_transform.zoom = Some(zoom.0);
            }
        }
        #[reflect(Component)]
        pub struct Smoothing {
            pub position: f32,
            pub rotation: f32,
            pub zoom: f32,
        }
        impl bevy::ecs::component::Component for Smoothing where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Smoothing where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <f32 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                        <f32 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                        <f32 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Smoothing where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<f32>("position").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                            bevy::reflect::NamedField::new::<f32>("rotation").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                            bevy::reflect::NamedField::new::<f32>("zoom").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Smoothing where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::core::camera_driver::Smoothing"
                    }
                    fn short_type_path() -> &'static str { "Smoothing" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Smoothing")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::camera_driver")
                    }
                }
                impl bevy::reflect::Struct for Smoothing where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, name: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match name {
                            "position" => ::core::option::Option::Some(&self.position),
                            "rotation" => ::core::option::Option::Some(&self.rotation),
                            "zoom" => ::core::option::Option::Some(&self.zoom),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, name: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match name {
                            "position" =>
                                ::core::option::Option::Some(&mut self.position),
                            "rotation" =>
                                ::core::option::Option::Some(&mut self.rotation),
                            "zoom" => ::core::option::Option::Some(&mut self.zoom),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_at(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.position),
                            1usize => ::core::option::Option::Some(&self.rotation),
                            2usize => ::core::option::Option::Some(&self.zoom),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_at_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.position),
                            1usize => ::core::option::Option::Some(&mut self.rotation),
                            2usize => ::core::option::Option::Some(&mut self.zoom),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn name_at(&self, index: usize)
                        -> ::core::option::Option<&str> {
                        match index {
                            0usize => ::core::option::Option::Some("position"),
                            1usize => ::core::option::Option::Some("rotation"),
                            2usize => ::core::option::Option::Some("zoom"),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_len(&self) -> usize { 3usize }
                    fn iter_fields(&self) -> bevy::reflect::FieldIter {
                        bevy::reflect::FieldIter::new(self)
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                        let mut dynamic: bevy::reflect::DynamicStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed("position",
                            bevy::reflect::Reflect::clone_value(&self.position));
                        dynamic.insert_boxed("rotation",
                            bevy::reflect::Reflect::clone_value(&self.rotation));
                        dynamic.insert_boxed("zoom",
                            bevy::reflect::Reflect::clone_value(&self.zoom));
                        dynamic
                    }
                }
                impl bevy::reflect::Reflect for Smoothing where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                    bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                    {
                                    let name =
                                        bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::Struct::field_mut(self, name) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::Struct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Struct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Struct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Struct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Struct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::struct_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Smoothing where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                    bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        position: (||
                                                        <f32 as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "position")?))()?,
                                        rotation: (||
                                                        <f32 as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "rotation")?))()?,
                                        zoom: (||
                                                        <f32 as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "zoom")?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[automatically_derived]
        impl ::core::marker::Copy for Smoothing { }
        #[automatically_derived]
        impl ::core::clone::Clone for Smoothing {
            #[inline]
            fn clone(&self) -> Smoothing {
                let _: ::core::clone::AssertParamIsClone<f32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Smoothing {
            #[inline]
            fn default() -> Smoothing {
                Smoothing {
                    position: ::core::default::Default::default(),
                    rotation: ::core::default::Default::default(),
                    zoom: ::core::default::Default::default(),
                }
            }
        }
        impl Smoothing {
            pub fn with_position(mut self, position: f32) -> Self {
                self.position = position;
                self
            }
            pub fn with_rotation(mut self, rotation: f32) -> Self {
                self.rotation = rotation;
                self
            }
            pub fn with_zoom(mut self, zoom: f32) -> Self {
                self.zoom = zoom;
                self
            }
        }
        fn sync_rig_transform(mut camera:
                Query<(&mut Transform, &RigTransform, Option<&Smoothing>,
                Option<&mut Projection>)>, time: Res<Time>) {
            for (mut camera_transform, rig_transform, smoothing, projection)
                in &mut camera {
                let mut translation = rig_transform.translation;
                let mut rotation = rig_transform.rotation;
                if let Some(smoothing) = smoothing {
                        const SMOOTHNESS_MULT: f32 = 8.0;
                        let smoothstep =
                            |value: f32|
                                {
                                    1.0 -
                                        (-SMOOTHNESS_MULT * time.delta_seconds() /
                                                    value.max(1e-5)).exp()
                                };
                        translation =
                            camera_transform.translation.lerp_radius(translation,
                                smoothstep(smoothing.position), f32::EPSILON);
                        rotation =
                            Quat::slerp(camera_transform.rotation.normalize(),
                                    rotation.normalize(),
                                    smoothstep(smoothing.rotation)).normalize();
                        if rig_transform.zoom.is_some() &&
                                        let Some(mut projection) = projection &&
                                    let Projection::Orthographic(orthographic_projection) =
                                        projection.as_mut() {
                                orthographic_projection.scale =
                                    orthographic_projection.scale.lerp_radius(rig_transform.zoom.unwrap(),
                                        smoothstep(smoothing.zoom), f32::EPSILON);
                            }
                    }
                camera_transform.translation = translation;
                camera_transform.rotation = rotation;
            }
        }
    }
    pub mod cleanup {
        use bevy::ecs::intern::Interned;
        use crate::prelude::*;
        #[component(storage = "SparseSet")]
        pub struct Cleanup<T: ScheduleLabel>(pub T);
        impl<T: ScheduleLabel> bevy::ecs::component::Component for Cleanup<T>
            where Self: Send + Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::SparseSet;
        }
        pub(super) fn plugin<T: ScheduleLabel>(schedule: T)
            -> CleanupPlugin<T> {
            CleanupPlugin::<T> {
                schedule: schedule.intern(),
                _marker: default(),
            }
        }
        pub(super) struct CleanupPlugin<T: ScheduleLabel> {
            schedule: Interned<dyn ScheduleLabel>,
            _marker: std::marker::PhantomData<T>,
        }
        impl<T: ScheduleLabel> Plugin for CleanupPlugin<T> {
            fn build(&self, app: &mut App) {
                app.add_systems(self.schedule, cleanup::<T>);
            }
        }
        fn cleanup<T: ScheduleLabel>(commands: ParallelCommands,
            entities: Query<Entity, With<Cleanup<T>>>) {
            entities.par_iter().for_each(|e|
                    {
                        commands.command_scope(|mut c|
                                { c.entity(e).despawn_recursive(); })
                    });
        }
    }
    pub mod despawn {
        use crate::prelude::*;
        pub(crate) enum DespawnSystems { Timer, Despawn, }
        impl bevy::ecs::schedule::SystemSet for DespawnSystems where
            Self: 'static + Send + Sync + Clone + Eq + ::std::fmt::Debug +
            ::std::hash::Hash {
            fn dyn_clone(&self)
                -> ::std::boxed::Box<dyn bevy::ecs::schedule::SystemSet> {
                ::std::boxed::Box::new(::std::clone::Clone::clone(self))
            }
            fn as_dyn_eq(&self) -> &dyn bevy::ecs::schedule::DynEq { self }
            fn dyn_hash(&self, mut state: &mut dyn ::std::hash::Hasher) {
                let ty_id = ::std::any::TypeId::of::<Self>();
                ::std::hash::Hash::hash(&ty_id, &mut state);
                ::std::hash::Hash::hash(self, &mut state);
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DespawnSystems {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f,
                    match self {
                        DespawnSystems::Timer => "Timer",
                        DespawnSystems::Despawn => "Despawn",
                    })
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for DespawnSystems {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr =
                    ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DespawnSystems { }
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DespawnSystems {
            #[inline]
            fn eq(&self, other: &DespawnSystems) -> bool {
                let __self_discr =
                    ::core::intrinsics::discriminant_value(self);
                let __arg1_discr =
                    ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for DespawnSystems {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DespawnSystems {
            #[inline]
            fn clone(&self) -> DespawnSystems {
                match self {
                    DespawnSystems::Timer => DespawnSystems::Timer,
                    DespawnSystems::Despawn => DespawnSystems::Despawn,
                }
            }
        }
        pub(super) fn plugin(app: &mut App) {
            app.register_type::<Despawn>();
            app.register_type::<Despawning>();
            ;
            app.configure_sets(Last,
                (DespawnSystems::Timer, DespawnSystems::Despawn).chain());
            app.add_systems(Last,
                (despawn_timer.chain().in_set(DespawnSystems::Timer),
                        despawning.in_set(DespawnSystems::Despawn)).chain());
        }
        #[reflect(Component)]
        #[component(storage = "SparseSet")]
        pub enum Despawn {

            #[default]
            Immediate,
            Delay(f32),
            WaitFrames(u32),
        }
        impl bevy::ecs::component::Component for Despawn where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::SparseSet;
        }
        #[automatically_derived]
        impl ::core::default::Default for Despawn {
            #[inline]
            fn default() -> Despawn { Self::Immediate }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Despawn where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    u32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <f32 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                        <u32 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Despawn where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    u32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Enum(bevy::reflect::EnumInfo::new::<Self>(&[bevy::reflect::VariantInfo::Unit(bevy::reflect::UnitVariantInfo::new("Immediate").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                            bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("Delay",
                                                                        &[bevy::reflect::UnnamedField::new::<f32>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                            bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("WaitFrames",
                                                                        &[bevy::reflect::UnnamedField::new::<u32>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Despawn where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::core::despawn::Despawn"
                    }
                    fn short_type_path() -> &'static str { "Despawn" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Despawn")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::despawn".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::despawn")
                    }
                }
                impl bevy::reflect::Enum for Despawn where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    u32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, __name_param: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at(&self, __index_param: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self {
                            Despawn::Delay { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            Despawn::WaitFrames { 0: value, .. } if
                                __index_param == 0usize =>
                                ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, __name_param: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at_mut(&mut self, __index_param: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self {
                            Despawn::Delay { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            Despawn::WaitFrames { 0: value, .. } if
                                __index_param == 0usize =>
                                ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn index_of(&self, __name_param: &str)
                        -> ::core::option::Option<usize> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn name_at(&self, __index_param: usize)
                        -> ::core::option::Option<&str> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn iter_fields(&self) -> bevy::reflect::VariantFieldIter {
                        bevy::reflect::VariantFieldIter::new(self)
                    }
                    #[inline]
                    fn field_len(&self) -> usize {
                        match self {
                            Despawn::Immediate { .. } => 0usize,
                            Despawn::Delay { .. } => 1usize,
                            Despawn::WaitFrames { .. } => 1usize,
                            _ => 0,
                        }
                    }
                    #[inline]
                    fn variant_name(&self) -> &str {
                        match self {
                            Despawn::Immediate { .. } => "Immediate",
                            Despawn::Delay { .. } => "Delay",
                            Despawn::WaitFrames { .. } => "WaitFrames",
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_index(&self) -> usize {
                        match self {
                            Despawn::Immediate { .. } => 0usize,
                            Despawn::Delay { .. } => 1usize,
                            Despawn::WaitFrames { .. } => 2usize,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_type(&self) -> bevy::reflect::VariantType {
                        match self {
                            Despawn::Immediate { .. } =>
                                bevy::reflect::VariantType::Unit,
                            Despawn::Delay { .. } => bevy::reflect::VariantType::Tuple,
                            Despawn::WaitFrames { .. } =>
                                bevy::reflect::VariantType::Tuple,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicEnum {
                        bevy::reflect::DynamicEnum::from_ref::<Self>(self)
                    }
                }
                impl bevy::reflect::Reflect for Despawn where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    u32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Enum::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        __value_param:
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(__value_param)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self,
                        __value_param: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Enum(__value_param) =
                                    bevy::reflect::Reflect::reflect_ref(__value_param) {
                                if bevy::reflect::Enum::variant_name(self) ==
                                            bevy::reflect::Enum::variant_name(__value_param) {
                                        match bevy::reflect::Enum::variant_type(__value_param) {
                                            bevy::reflect::VariantType::Struct => {
                                                for field in bevy::reflect::Enum::iter_fields(__value_param)
                                                    {
                                                    let name = field.name().unwrap();
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_mut(self, name) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            bevy::reflect::VariantType::Tuple => {
                                                for (index, field) in
                                                    ::core::iter::Iterator::enumerate(bevy::reflect::Enum::iter_fields(__value_param))
                                                    {
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_at_mut(self, index) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            _ => {}
                                        }
                                    } else {
                                       match bevy::reflect::Enum::variant_name(__value_param) {
                                           "Immediate" => { *self = Despawn::Immediate {} }
                                           "Delay" => {
                                               *self =
                                                   Despawn::Delay {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("Delay"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <f32 as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<f32 as
                                                                                   bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           "WaitFrames" => {
                                               *self =
                                                   Despawn::WaitFrames {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("WaitFrames"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <u32 as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<u32 as
                                                                                   bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           name => {
                                               return ::core::result::Result::Err(bevy::reflect::ApplyError::UnknownVariant {
                                                           enum_name: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(self)),
                                                           variant_name: ::core::convert::Into::into(name),
                                                       });
                                           }
                                       }
                                   }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(__value_param),
                                           to_kind: bevy::reflect::ReflectKind::Enum,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Enum
                    }
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Enum(self)
                    }
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Enum(self)
                    }
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Enum(self)
                    }
                    fn reflect_hash(&self) -> ::core::option::Option<u64> {
                        bevy::reflect::enum_hash(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::enum_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Despawn where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, f32: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    u32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(__param0: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Enum(__param0) =
                                    bevy::reflect::Reflect::reflect_ref(__param0) {
                                match bevy::reflect::Enum::variant_name(__param0) {
                                    "Immediate" =>
                                        ::core::option::Option::Some(Despawn::Immediate {}),
                                    "Delay" =>
                                        ::core::option::Option::Some(Despawn::Delay {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <f32 as bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    "WaitFrames" =>
                                        ::core::option::Option::Some(Despawn::WaitFrames {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <u32 as bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    name => {
                                        ::core::panicking::panic_fmt(format_args!("variant with name `{0}` does not exist on enum `{1}`",
                                                name, <Self as bevy::reflect::TypePath>::type_path()));
                                    }
                                }
                            } else { ::core::option::Option::None }
                    }
                }
            };
        pub struct Despawning;
        impl bevy::ecs::component::Component for Despawning where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl ::core::default::Default for Despawning {
            #[inline]
            fn default() -> Despawning { Despawning {} }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Despawning where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {}
                }
                impl bevy::reflect::Typed for Despawning where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Despawning where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::core::despawn::Despawning"
                    }
                    fn short_type_path() -> &'static str { "Despawning" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Despawning")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::despawn".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::despawn")
                    }
                }
                impl bevy::reflect::Struct for Despawning where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn field(&self, name: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match name { _ => ::core::option::Option::None, }
                    }
                    fn field_mut(&mut self, name: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match name { _ => ::core::option::Option::None, }
                    }
                    fn field_at(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index { _ => ::core::option::Option::None, }
                    }
                    fn field_at_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index { _ => ::core::option::Option::None, }
                    }
                    fn name_at(&self, index: usize)
                        -> ::core::option::Option<&str> {
                        match index { _ => ::core::option::Option::None, }
                    }
                    fn field_len(&self) -> usize { 0usize }
                    fn iter_fields(&self) -> bevy::reflect::FieldIter {
                        bevy::reflect::FieldIter::new(self)
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                        let mut dynamic: bevy::reflect::DynamicStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic
                    }
                }
                impl bevy::reflect::Reflect for Despawning where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                    bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                    {
                                    let name =
                                        bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::Struct::field_mut(self, name) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::Struct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Struct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Struct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Struct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Struct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::struct_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Despawning where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                    bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {})
                            } else { ::core::option::Option::None }
                    }
                }
            };
        fn despawn_timer(mut commands: Commands,
            mut despawns: Query<(Entity, &mut Despawn)>, time: Res<Time>) {
            for (entity, mut despawn) in &mut despawns {
                let despawn =
                    match *despawn {
                        Despawn::Immediate => true,
                        Despawn::Delay(ref mut dur) => {
                            *dur -= time.delta_seconds();
                            *dur <= 0.0
                        }
                        Despawn::WaitFrames(ref mut frame) => {
                            if *frame == 0 { true } else { *frame -= 1; *frame == 0 }
                        }
                    };
                if despawn {
                        commands.entity(entity).remove::<Despawn>().insert(Despawning);
                    }
            }
        }
        fn despawning(mut commands: Commands,
            despawning: Query<Entity, Added<Despawning>>) {
            for entity in &despawning {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
    pub mod name {
        use crate::prelude::*;
        use std::borrow::Cow;
        macro_rules! implement_name_tags {
            ($($tag:tt),*) =>
            {
                pub trait NameTags
                {
                    $(fn $tag(name: impl Into<Cow<'static, str>>) -> Name
                    {
                        Name::new(format!(":{} {}", stringify!($tag), name.into()))
                    })*
                } impl NameTags for Name {}
            };
        }
        pub trait NameTags {
            fn ui(name: impl Into<Cow<'static, str>>) -> Name {
                Name::new(::alloc::__export::must_use({
                            let res =
                                ::alloc::fmt::format(format_args!(":{0} {1}", "ui",
                                        name.into()));
                            res
                        }))
            }
            fn unit(name: impl Into<Cow<'static, str>>) -> Name {
                Name::new(::alloc::__export::must_use({
                            let res =
                                ::alloc::fmt::format(format_args!(":{0} {1}", "unit",
                                        name.into()));
                            res
                        }))
            }
            fn camera(name: impl Into<Cow<'static, str>>) -> Name {
                Name::new(::alloc::__export::must_use({
                            let res =
                                ::alloc::fmt::format(format_args!(":{0} {1}", "camera",
                                        name.into()));
                            res
                        }))
            }
            fn light(name: impl Into<Cow<'static, str>>) -> Name {
                Name::new(::alloc::__export::must_use({
                            let res =
                                ::alloc::fmt::format(format_args!(":{0} {1}", "light",
                                        name.into()));
                            res
                        }))
            }
        }
        impl NameTags for Name {}
    }
    pub mod previous {
        use crate::prelude::*;
        pub(crate) fn plugin<T: Component + Clone + GetTypeRegistration +
            TypePath + FromReflect>(app: &mut App) {
            app.register_type::<Previous<T>>();
            ;
            app.add_systems(Last, propagate_previous_changed::<T>);
        }
        pub struct Previous<T: Component + Clone>(T);
        impl<T: Component + Clone> bevy::ecs::component::Component for
            Previous<T> where Self: Send + Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl<T: ::core::default::Default + Component + Clone>
            ::core::default::Default for Previous<T> {
            #[inline]
            fn default() -> Previous<T> {
                Previous(::core::default::Default::default())
            }
        }
        impl<T: Component + Clone> ::std::ops::Deref for Previous<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl<T: Component + Clone> ::std::ops::DerefMut for Previous<T> {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl<T: Component + Clone> bevy::reflect::GetTypeRegistration
                    for Previous<T> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    T: bevy::reflect::TypePath, T: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <T as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl<T: Component + Clone> bevy::reflect::Typed for
                    Previous<T> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    T: bevy::reflect::TypePath, T: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::GenericTypeInfoCell =
                            bevy::reflect::utility::GenericTypeInfoCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[bevy::reflect::UnnamedField::new::<T>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl<T: Component + Clone> bevy::reflect::TypePath for
                    Previous<T> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    T: bevy::reflect::TypePath {
                    fn type_path() -> &'static str {
                        static CELL: bevy::reflect::utility::GenericTypePathCell =
                            bevy::reflect::utility::GenericTypePathCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    ::std::string::ToString::to_string("warband_lib::core::previous::Previous<")
                                            + <T as bevy::reflect::TypePath>::type_path() + ">"
                                })
                    }
                    fn short_type_path() -> &'static str {
                        static CELL: bevy::reflect::utility::GenericTypePathCell =
                            bevy::reflect::utility::GenericTypePathCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    ::std::string::ToString::to_string("Previous<") +
                                            <T as bevy::reflect::TypePath>::short_type_path() + ">"
                                })
                    }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Previous")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::previous".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::core::previous")
                    }
                }
                impl<T: Component + Clone> bevy::reflect::TupleStruct for
                    Previous<T> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    T: bevy::reflect::TypePath, T: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    #[inline]
                    fn field_len(&self) -> usize { 1usize }
                    #[inline]
                    fn iter_fields(&self)
                        -> bevy::reflect::TupleStructFieldIter {
                        bevy::reflect::TupleStructFieldIter::new(self)
                    }
                    fn clone_dynamic(&self)
                        -> bevy::reflect::DynamicTupleStruct {
                        let mut dynamic: bevy::reflect::DynamicTupleStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed(bevy::reflect::Reflect::clone_value(&self.0));
                        dynamic
                    }
                }
                impl<T: Component + Clone> bevy::reflect::Reflect for
                    Previous<T> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    T: bevy::reflect::TypePath, T: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                    = bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                    {
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::TupleStruct::field_mut(self, i) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::TupleStruct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::TupleStruct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::tuple_struct_partial_eq(self, value)
                    }
                }
                impl<T: Component + Clone> bevy::reflect::FromReflect for
                    Previous<T> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    T: bevy::reflect::TypePath, T: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                    = bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        0: (||
                                                        <T as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::TupleStruct::field(__ref_struct,
                                                                    0)?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[automatically_derived]
        impl<T: Component + Clone> ::core::convert::From<(T)> for Previous<T>
            {
            #[inline]
            fn from(original: (T)) -> Previous<T> { Previous(original) }
        }
        impl<T: Component + Clone> Previous<T> {
            #[allow(unused)]
            pub fn get(&self) -> &T { &self.0 }
        }
        pub(crate) fn propagate_previous_changed<T: Component +
            Clone>(mut commands: Commands,
            mut values:
                Query<(Entity, Option<&mut Previous<T>>, &T), Changed<T>>) {
            for (entity, mut previous_value, current_value) in
                values.iter_mut() {
                if let Some(previous_value) = &mut previous_value {
                        previous_value.0 = current_value.clone();
                    } else {
                       commands.entity(entity).insert(Previous(current_value.clone()));
                   }
            }
        }
    }
    pub mod required_component {
        use crate::prelude::*;
        /// A system that make's sure component [`U`] is present on all entities that have component [`T`].
        /// If [`T`] is removed from an entity, [`U`] is also removed.
        pub(crate) fn required_component<T: Component, U: Component +
            Default>(commands: ParallelCommands,
            without: Query<Entity, (With<T>, Without<U>)>,
            mut removed: RemovedComponents<T>) {
            without.par_iter().for_each(|entity|
                    {
                        commands.command_scope(|mut c|
                                { c.entity(entity).insert(U::default()); })
                    });
            for entity in &mut removed.read() {
                commands.command_scope(|mut c|
                        {
                            if let Some(mut commands) = c.get_entity(entity) {
                                    commands.remove::<U>();
                                }
                        });
            }
        }
    }
    pub(super) fn plugin(app: &mut App) {
        app.register_type::<Owner>();
        ;
        app.add_plugins(bevy_mod_picking::DefaultPickingPlugins);
        app.add_plugins((despawn::plugin, camera_driver::plugin(Last)));
        app.add_plugins(cleanup::plugin(OnEnter(AppState::InGame)));
        app.add_plugins(cleanup::plugin(OnExit(AppState::InGame)));
    }
    /// Component to mark own
    pub struct Owner(pub Entity);
    impl bevy::ecs::component::Component for Owner where Self: Send + Sync +
        'static {
        const STORAGE_TYPE: bevy::ecs::component::StorageType =
            bevy::ecs::component::StorageType::Table;
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl bevy::reflect::GetTypeRegistration for Owner where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {
                    <Entity as
                            bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                }
            }
            impl bevy::reflect::Typed for Owner where Self: ::core::any::Any +
                ::core::marker::Send + ::core::marker::Sync,
                Entity: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                        =
                        bevy::reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(||
                            {
                                bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[bevy::reflect::UnnamedField::new::<Entity>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl bevy::reflect::TypePath for Owner where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync {
                fn type_path() -> &'static str { "warband_lib::core::Owner" }
                fn short_type_path() -> &'static str { "Owner" }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("Owner")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::core".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::core")
                }
            }
            impl bevy::reflect::TupleStruct for Owner where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn field(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&self.0),
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index {
                        0usize => ::core::option::Option::Some(&mut self.0),
                        _ => ::core::option::Option::None,
                    }
                }
                #[inline]
                fn field_len(&self) -> usize { 1usize }
                #[inline]
                fn iter_fields(&self) -> bevy::reflect::TupleStructFieldIter {
                    bevy::reflect::TupleStructFieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicTupleStruct {
                    let mut dynamic: bevy::reflect::DynamicTupleStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic.insert_boxed(bevy::reflect::Reflect::clone_value(&self.0));
                    dynamic
                }
            }
            impl bevy::reflect::Reflect for Owner where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                = bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                {
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::TupleStruct::field_mut(self, i) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::TupleStruct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::TupleStruct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::TupleStruct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::TupleStruct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::tuple_struct_partial_eq(self, value)
                }
            }
            impl bevy::reflect::FromReflect for Owner where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                bevy::reflect::TypePath +
                bevy::reflect::__macro_exports::RegisterForReflection {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                = bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {
                                    0: (||
                                                    <Entity as
                                                            bevy::reflect::FromReflect>::from_reflect(bevy::reflect::TupleStruct::field(__ref_struct,
                                                                0)?))()?,
                                })
                        } else { ::core::option::Option::None }
                }
            }
        };
    #[automatically_derived]
    impl ::core::fmt::Debug for Owner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Owner",
                &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Owner {
        #[inline]
        fn clone(&self) -> Owner {
            let _: ::core::clone::AssertParamIsClone<Entity>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Owner { }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Owner { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Owner {
        #[inline]
        fn eq(&self, other: &Owner) -> bool { self.0 == other.0 }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Owner {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Entity>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Owner {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    impl ::std::ops::Deref for Owner {
        type Target = Entity;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl ::std::ops::DerefMut for Owner {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    #[automatically_derived]
    impl ::core::convert::From<(Entity)> for Owner {
        #[inline]
        fn from(original: (Entity)) -> Owner { Owner(original) }
    }
    /// Generic component to mark component [`T`] as dirty.
    #[component(storage = "SparseSet")]
    pub struct Dirty<T: Component>(
        #[reflect(ignore)]
        pub PhantomData<T>);
    impl<T: Component> bevy::ecs::component::Component for Dirty<T> where
        Self: Send + Sync + 'static {
        const STORAGE_TYPE: bevy::ecs::component::StorageType =
            bevy::ecs::component::StorageType::SparseSet;
    }
    #[automatically_derived]
    impl<T: ::core::default::Default + Component> ::core::default::Default for
        Dirty<T> {
        #[inline]
        fn default() -> Dirty<T> {
            Dirty(::core::default::Default::default())
        }
    }
    impl<T: Component> ::std::ops::Deref for Dirty<T> {
        type Target = PhantomData<T>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<T: Component> ::std::ops::DerefMut for Dirty<T> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    #[automatically_derived]
    impl<T: Component> ::core::convert::From<(PhantomData<T>)> for Dirty<T> {
        #[inline]
        fn from(original: (PhantomData<T>)) -> Dirty<T> { Dirty(original) }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl<T: Component> bevy::reflect::GetTypeRegistration for Dirty<T>
                where Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {}
            }
            impl<T: Component> bevy::reflect::Typed for Dirty<T> where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::GenericTypeInfoCell =
                        bevy::reflect::utility::GenericTypeInfoCell::new();
                    CELL.get_or_insert::<Self,
                        _>(||
                            {
                                bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl<T: Component> bevy::reflect::TypePath for Dirty<T> where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                fn type_path() -> &'static str {
                    static CELL: bevy::reflect::utility::GenericTypePathCell =
                        bevy::reflect::utility::GenericTypePathCell::new();
                    CELL.get_or_insert::<Self,
                        _>(||
                            {
                                ::std::string::ToString::to_string("warband_lib::core::Dirty<")
                                        + <T as bevy::reflect::TypePath>::type_path() + ">"
                            })
                }
                fn short_type_path() -> &'static str {
                    static CELL: bevy::reflect::utility::GenericTypePathCell =
                        bevy::reflect::utility::GenericTypePathCell::new();
                    CELL.get_or_insert::<Self,
                        _>(||
                            {
                                ::std::string::ToString::to_string("Dirty<") +
                                        <T as bevy::reflect::TypePath>::short_type_path() + ">"
                            })
                }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("Dirty")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::core".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::core")
                }
            }
            impl<T: Component> bevy::reflect::TupleStruct for Dirty<T> where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                fn field(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn field_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                #[inline]
                fn field_len(&self) -> usize { 0usize }
                #[inline]
                fn iter_fields(&self) -> bevy::reflect::TupleStructFieldIter {
                    bevy::reflect::TupleStructFieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicTupleStruct {
                    let mut dynamic: bevy::reflect::DynamicTupleStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic
                }
            }
            impl<T: Component> bevy::reflect::Reflect for Dirty<T> where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                = bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                {
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::TupleStruct::field_mut(self, i) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::TupleStruct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::TupleStruct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::TupleStruct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::TupleStruct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::tuple_struct_partial_eq(self, value)
                }
            }
            impl<T: Component> bevy::reflect::FromReflect for Dirty<T> where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                = bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {
                                    0: ::core::default::Default::default(),
                                })
                        } else { ::core::option::Option::None }
                }
            }
        };
    /// Generic component to mark component [`T`] as disabled.
    #[component(storage = "SparseSet")]
    pub struct Disabled<T: Component>(
        #[reflect(ignore)]
        pub PhantomData<T>);
    impl<T: Component> bevy::ecs::component::Component for Disabled<T> where
        Self: Send + Sync + 'static {
        const STORAGE_TYPE: bevy::ecs::component::StorageType =
            bevy::ecs::component::StorageType::SparseSet;
    }
    #[automatically_derived]
    impl<T: ::core::default::Default + Component> ::core::default::Default for
        Disabled<T> {
        #[inline]
        fn default() -> Disabled<T> {
            Disabled(::core::default::Default::default())
        }
    }
    impl<T: Component> ::std::ops::Deref for Disabled<T> {
        type Target = PhantomData<T>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<T: Component> ::std::ops::DerefMut for Disabled<T> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    #[automatically_derived]
    impl<T: Component> ::core::convert::From<(PhantomData<T>)> for Disabled<T>
        {
        #[inline]
        fn from(original: (PhantomData<T>)) -> Disabled<T> {
            Disabled(original)
        }
    }
    const _: () =
        {
            #[allow(unused_mut)]
            impl<T: Component> bevy::reflect::GetTypeRegistration for
                Disabled<T> where Self: ::core::any::Any +
                ::core::marker::Send + ::core::marker::Sync,
                T: bevy::reflect::TypePath {
                fn get_type_registration()
                    -> bevy::reflect::TypeRegistration {
                    let mut registration =
                        bevy::reflect::TypeRegistration::of::<Self>();
                    registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                    registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(registry:
                        &mut bevy::reflect::TypeRegistry) {}
            }
            impl<T: Component> bevy::reflect::Typed for Disabled<T> where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                fn type_info() -> &'static bevy::reflect::TypeInfo {
                    static CELL: bevy::reflect::utility::GenericTypeInfoCell =
                        bevy::reflect::utility::GenericTypeInfoCell::new();
                    CELL.get_or_insert::<Self,
                        _>(||
                            {
                                bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                            })
                }
            }
            impl<T: Component> bevy::reflect::TypePath for Disabled<T> where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                fn type_path() -> &'static str {
                    static CELL: bevy::reflect::utility::GenericTypePathCell =
                        bevy::reflect::utility::GenericTypePathCell::new();
                    CELL.get_or_insert::<Self,
                        _>(||
                            {
                                ::std::string::ToString::to_string("warband_lib::core::Disabled<")
                                        + <T as bevy::reflect::TypePath>::type_path() + ">"
                            })
                }
                fn short_type_path() -> &'static str {
                    static CELL: bevy::reflect::utility::GenericTypePathCell =
                        bevy::reflect::utility::GenericTypePathCell::new();
                    CELL.get_or_insert::<Self,
                        _>(||
                            {
                                ::std::string::ToString::to_string("Disabled<") +
                                        <T as bevy::reflect::TypePath>::short_type_path() + ">"
                            })
                }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("Disabled")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::core".split(':').next().unwrap())
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("warband_lib::core")
                }
            }
            impl<T: Component> bevy::reflect::TupleStruct for Disabled<T>
                where Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                fn field(&self, index: usize)
                    -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                fn field_mut(&mut self, index: usize)
                    -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                    match index { _ => ::core::option::Option::None, }
                }
                #[inline]
                fn field_len(&self) -> usize { 0usize }
                #[inline]
                fn iter_fields(&self) -> bevy::reflect::TupleStructFieldIter {
                    bevy::reflect::TupleStructFieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy::reflect::DynamicTupleStruct {
                    let mut dynamic: bevy::reflect::DynamicTupleStruct =
                        ::core::default::Default::default();
                    dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                    dynamic
                }
            }
            impl<T: Component> bevy::reflect::Reflect for Disabled<T> where
                Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                #[inline]
                fn get_represented_type_info(&self)
                    ->
                        ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                    ::core::option::Option::Some(<Self as
                                bevy::reflect::Typed>::type_info())
                }
                #[inline]
                fn into_any(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn ::core::any::Any> {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any { self }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
                #[inline]
                fn into_reflect(self: ::std::boxed::Box<Self>)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                #[inline]
                fn as_reflect_mut(&mut self)
                    -> &mut dyn bevy::reflect::Reflect {
                    self
                }
                #[inline]
                fn clone_value(&self)
                    -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                    ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                }
                #[inline]
                fn set(&mut self,
                    value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                    ->
                        ::core::result::Result<(),
                        ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                    *self = <dyn bevy::reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                    -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                    if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                = bevy::reflect::Reflect::reflect_ref(value) {
                            for (i, value) in
                                ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                {
                                if let ::core::option::Option::Some(v) =
                                            bevy::reflect::TupleStruct::field_mut(self, i) {
                                        bevy::reflect::Reflect::try_apply(v, value)?;
                                    }
                            }
                        } else {
                           return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                       from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                       to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                   });
                       }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                    bevy::reflect::ReflectKind::TupleStruct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                    bevy::reflect::ReflectRef::TupleStruct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                    bevy::reflect::ReflectMut::TupleStruct(self)
                }
                #[inline]
                fn reflect_owned(self: ::std::boxed::Box<Self>)
                    -> bevy::reflect::ReflectOwned {
                    bevy::reflect::ReflectOwned::TupleStruct(self)
                }
                fn reflect_partial_eq(&self,
                    value: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<bool> {
                    bevy::reflect::tuple_struct_partial_eq(self, value)
                }
            }
            impl<T: Component> bevy::reflect::FromReflect for Disabled<T>
                where Self: ::core::any::Any + ::core::marker::Send +
                ::core::marker::Sync, T: bevy::reflect::TypePath {
                fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                    -> ::core::option::Option<Self> {
                    if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                = bevy::reflect::Reflect::reflect_ref(reflect) {
                            ::core::option::Option::Some(Self {
                                    0: ::core::default::Default::default(),
                                })
                        } else { ::core::option::Option::None }
                }
            }
        };
}
mod navigation {
    use crate::{prelude::*, AppState};
    pub mod agent {
        use crate::prelude::*;
        #[reflect(Component)]
        pub struct Agent {
            size: u8,
        }
        impl bevy::ecs::component::Component for Agent where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Agent where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, u8: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <u8 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Agent where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, u8: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<u8>("size").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Agent where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::navigation::agent::Agent"
                    }
                    fn short_type_path() -> &'static str { "Agent" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Agent")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::navigation::agent".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::navigation::agent")
                    }
                }
                impl bevy::reflect::Struct for Agent where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, u8: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, name: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match name {
                            "size" => ::core::option::Option::Some(&self.size),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, name: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match name {
                            "size" => ::core::option::Option::Some(&mut self.size),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_at(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.size),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_at_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.size),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn name_at(&self, index: usize)
                        -> ::core::option::Option<&str> {
                        match index {
                            0usize => ::core::option::Option::Some("size"),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_len(&self) -> usize { 1usize }
                    fn iter_fields(&self) -> bevy::reflect::FieldIter {
                        bevy::reflect::FieldIter::new(self)
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                        let mut dynamic: bevy::reflect::DynamicStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed("size",
                            bevy::reflect::Reflect::clone_value(&self.size));
                        dynamic
                    }
                }
                impl bevy::reflect::Reflect for Agent where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, u8: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                    bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                    {
                                    let name =
                                        bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::Struct::field_mut(self, name) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::Struct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Struct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Struct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Struct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Struct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::struct_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Agent where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, u8: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                    bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        size: (||
                                                        <u8 as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "size")?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        impl Default for Agent {
            fn default() -> Self { Self { size: 1 } }
        }
        impl Agent {
            pub fn with_size(size: u8) -> Self {
                if !(size > 0) {
                        ::core::panicking::panic("assertion failed: size > 0")
                    };
                Self { size }
            }
            pub fn size(&self) -> u8 { self.size }
        }
    }
    pub mod path {
        use bevy::tasks::AsyncComputeTaskPool;
        use hexx::{algorithms::a_star, Hex};
        use crate::{
            board::{self, Footprint, Location},
            prelude::*,
        };
        use std::sync::{Arc, RwLock};
        #[reflect(Component)]
        pub enum Target {

            #[default]
            None,
            Entity(Entity),
            Cell(Hex),
        }
        impl bevy::ecs::component::Component for Target where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Target {
            #[inline]
            fn clone(&self) -> Target {
                let _: ::core::clone::AssertParamIsClone<Entity>;
                let _: ::core::clone::AssertParamIsClone<Hex>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Target { }
        #[automatically_derived]
        impl ::core::default::Default for Target {
            #[inline]
            fn default() -> Target { Self::None }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Target { }
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Target {
            #[inline]
            fn eq(&self, other: &Target) -> bool {
                let __self_discr =
                    ::core::intrinsics::discriminant_value(self);
                let __arg1_discr =
                    ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr &&
                    match (self, other) {
                        (Target::Entity(__self_0), Target::Entity(__arg1_0)) =>
                            __self_0 == __arg1_0,
                        (Target::Cell(__self_0), Target::Cell(__arg1_0)) =>
                            __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Target {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Entity>;
                let _: ::core::cmp::AssertParamIsEq<Hex>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Target {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr =
                    ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    Target::Entity(__self_0) =>
                        ::core::hash::Hash::hash(__self_0, state),
                    Target::Cell(__self_0) =>
                        ::core::hash::Hash::hash(__self_0, state),
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Target {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                match self {
                    Target::None =>
                        ::core::fmt::Formatter::write_str(f, "None"),
                    Target::Entity(__self_0) =>
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f,
                            "Entity", &__self_0),
                    Target::Cell(__self_0) =>
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Cell",
                            &__self_0),
                }
            }
        }
        #[automatically_derived]
        impl ::core::convert::From<(Hex)> for Target {
            #[inline]
            fn from(original: (Hex)) -> Target { Target::Cell(original) }
        }
        #[automatically_derived]
        impl ::core::convert::From<()> for Target {
            #[inline]
            fn from(original: ()) -> Target { Target::None {} }
        }
        #[automatically_derived]
        impl ::core::convert::From<(Entity)> for Target {
            #[inline]
            fn from(original: (Entity)) -> Target { Target::Entity(original) }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Target where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Hex: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <Entity as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                        <Hex as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Target where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Hex: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Enum(bevy::reflect::EnumInfo::new::<Self>(&[bevy::reflect::VariantInfo::Unit(bevy::reflect::UnitVariantInfo::new("None").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                            bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("Entity",
                                                                        &[bevy::reflect::UnnamedField::new::<Entity>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                            bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("Cell",
                                                                        &[bevy::reflect::UnnamedField::new::<Hex>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Target where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::navigation::path::Target"
                    }
                    fn short_type_path() -> &'static str { "Target" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Target")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::navigation::path".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::navigation::path")
                    }
                }
                impl bevy::reflect::Enum for Target where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Hex: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, __name_param: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at(&self, __index_param: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self {
                            Target::Entity { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            Target::Cell { 0: value, .. } if __index_param == 0usize =>
                                ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, __name_param: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at_mut(&mut self, __index_param: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self {
                            Target::Entity { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            Target::Cell { 0: value, .. } if __index_param == 0usize =>
                                ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn index_of(&self, __name_param: &str)
                        -> ::core::option::Option<usize> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn name_at(&self, __index_param: usize)
                        -> ::core::option::Option<&str> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn iter_fields(&self) -> bevy::reflect::VariantFieldIter {
                        bevy::reflect::VariantFieldIter::new(self)
                    }
                    #[inline]
                    fn field_len(&self) -> usize {
                        match self {
                            Target::None { .. } => 0usize,
                            Target::Entity { .. } => 1usize,
                            Target::Cell { .. } => 1usize,
                            _ => 0,
                        }
                    }
                    #[inline]
                    fn variant_name(&self) -> &str {
                        match self {
                            Target::None { .. } => "None",
                            Target::Entity { .. } => "Entity",
                            Target::Cell { .. } => "Cell",
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_index(&self) -> usize {
                        match self {
                            Target::None { .. } => 0usize,
                            Target::Entity { .. } => 1usize,
                            Target::Cell { .. } => 2usize,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_type(&self) -> bevy::reflect::VariantType {
                        match self {
                            Target::None { .. } => bevy::reflect::VariantType::Unit,
                            Target::Entity { .. } => bevy::reflect::VariantType::Tuple,
                            Target::Cell { .. } => bevy::reflect::VariantType::Tuple,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicEnum {
                        bevy::reflect::DynamicEnum::from_ref::<Self>(self)
                    }
                }
                impl bevy::reflect::Reflect for Target where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Hex: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Enum::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        __value_param:
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(__value_param)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self,
                        __value_param: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Enum(__value_param) =
                                    bevy::reflect::Reflect::reflect_ref(__value_param) {
                                if bevy::reflect::Enum::variant_name(self) ==
                                            bevy::reflect::Enum::variant_name(__value_param) {
                                        match bevy::reflect::Enum::variant_type(__value_param) {
                                            bevy::reflect::VariantType::Struct => {
                                                for field in bevy::reflect::Enum::iter_fields(__value_param)
                                                    {
                                                    let name = field.name().unwrap();
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_mut(self, name) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            bevy::reflect::VariantType::Tuple => {
                                                for (index, field) in
                                                    ::core::iter::Iterator::enumerate(bevy::reflect::Enum::iter_fields(__value_param))
                                                    {
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_at_mut(self, index) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            _ => {}
                                        }
                                    } else {
                                       match bevy::reflect::Enum::variant_name(__value_param) {
                                           "None" => { *self = Target::None {} }
                                           "Entity" => {
                                               *self =
                                                   Target::Entity {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("Entity"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <Entity as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<Entity as
                                                                                   bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           "Cell" => {
                                               *self =
                                                   Target::Cell {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("Cell"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <Hex as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<Hex as
                                                                                   bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           name => {
                                               return ::core::result::Result::Err(bevy::reflect::ApplyError::UnknownVariant {
                                                           enum_name: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(self)),
                                                           variant_name: ::core::convert::Into::into(name),
                                                       });
                                           }
                                       }
                                   }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(__value_param),
                                           to_kind: bevy::reflect::ReflectKind::Enum,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Enum
                    }
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Enum(self)
                    }
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Enum(self)
                    }
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Enum(self)
                    }
                    fn reflect_hash(&self) -> ::core::option::Option<u64> {
                        bevy::reflect::enum_hash(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::enum_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Target where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    Hex: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(__param0: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Enum(__param0) =
                                    bevy::reflect::Reflect::reflect_ref(__param0) {
                                match bevy::reflect::Enum::variant_name(__param0) {
                                    "None" => ::core::option::Option::Some(Target::None {}),
                                    "Entity" =>
                                        ::core::option::Option::Some(Target::Entity {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <Entity as bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    "Cell" =>
                                        ::core::option::Option::Some(Target::Cell {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <Hex as bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    name => {
                                        ::core::panicking::panic_fmt(format_args!("variant with name `{0}` does not exist on enum `{1}`",
                                                name, <Self as bevy::reflect::TypePath>::type_path()));
                                    }
                                }
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[reflect(Component)]
        pub enum TargetLocation {

            #[default]
            None,
            Value(Hex),
        }
        impl bevy::ecs::component::Component for TargetLocation where
            Self: Send + Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TargetLocation {
            #[inline]
            fn clone(&self) -> TargetLocation {
                let _: ::core::clone::AssertParamIsClone<Hex>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TargetLocation { }
        #[automatically_derived]
        impl ::core::default::Default for TargetLocation {
            #[inline]
            fn default() -> TargetLocation { Self::None }
        }
        #[automatically_derived]
        impl ::core::convert::From<(Hex)> for TargetLocation {
            #[inline]
            fn from(original: (Hex)) -> TargetLocation {
                TargetLocation::Value(original)
            }
        }
        #[automatically_derived]
        impl ::core::convert::From<()> for TargetLocation {
            #[inline]
            fn from(original: ()) -> TargetLocation {
                TargetLocation::None {}
            }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for TargetLocation
                    where Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Hex: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <Hex as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for TargetLocation where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Hex: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Enum(bevy::reflect::EnumInfo::new::<Self>(&[bevy::reflect::VariantInfo::Unit(bevy::reflect::UnitVariantInfo::new("None").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                            bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("Value",
                                                                        &[bevy::reflect::UnnamedField::new::<Hex>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for TargetLocation where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::navigation::path::TargetLocation"
                    }
                    fn short_type_path() -> &'static str { "TargetLocation" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("TargetLocation")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::navigation::path".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::navigation::path")
                    }
                }
                impl bevy::reflect::Enum for TargetLocation where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Hex: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, __name_param: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at(&self, __index_param: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self {
                            TargetLocation::Value { 0: value, .. } if
                                __index_param == 0usize =>
                                ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, __name_param: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at_mut(&mut self, __index_param: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self {
                            TargetLocation::Value { 0: value, .. } if
                                __index_param == 0usize =>
                                ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn index_of(&self, __name_param: &str)
                        -> ::core::option::Option<usize> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn name_at(&self, __index_param: usize)
                        -> ::core::option::Option<&str> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn iter_fields(&self) -> bevy::reflect::VariantFieldIter {
                        bevy::reflect::VariantFieldIter::new(self)
                    }
                    #[inline]
                    fn field_len(&self) -> usize {
                        match self {
                            TargetLocation::None { .. } => 0usize,
                            TargetLocation::Value { .. } => 1usize,
                            _ => 0,
                        }
                    }
                    #[inline]
                    fn variant_name(&self) -> &str {
                        match self {
                            TargetLocation::None { .. } => "None",
                            TargetLocation::Value { .. } => "Value",
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_index(&self) -> usize {
                        match self {
                            TargetLocation::None { .. } => 0usize,
                            TargetLocation::Value { .. } => 1usize,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_type(&self) -> bevy::reflect::VariantType {
                        match self {
                            TargetLocation::None { .. } =>
                                bevy::reflect::VariantType::Unit,
                            TargetLocation::Value { .. } =>
                                bevy::reflect::VariantType::Tuple,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicEnum {
                        bevy::reflect::DynamicEnum::from_ref::<Self>(self)
                    }
                }
                impl bevy::reflect::Reflect for TargetLocation where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Hex: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Enum::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        __value_param:
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(__value_param)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self,
                        __value_param: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Enum(__value_param) =
                                    bevy::reflect::Reflect::reflect_ref(__value_param) {
                                if bevy::reflect::Enum::variant_name(self) ==
                                            bevy::reflect::Enum::variant_name(__value_param) {
                                        match bevy::reflect::Enum::variant_type(__value_param) {
                                            bevy::reflect::VariantType::Struct => {
                                                for field in bevy::reflect::Enum::iter_fields(__value_param)
                                                    {
                                                    let name = field.name().unwrap();
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_mut(self, name) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            bevy::reflect::VariantType::Tuple => {
                                                for (index, field) in
                                                    ::core::iter::Iterator::enumerate(bevy::reflect::Enum::iter_fields(__value_param))
                                                    {
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_at_mut(self, index) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            _ => {}
                                        }
                                    } else {
                                       match bevy::reflect::Enum::variant_name(__value_param) {
                                           "None" => { *self = TargetLocation::None {} }
                                           "Value" => {
                                               *self =
                                                   TargetLocation::Value {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("Value"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <Hex as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<Hex as
                                                                                   bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           name => {
                                               return ::core::result::Result::Err(bevy::reflect::ApplyError::UnknownVariant {
                                                           enum_name: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(self)),
                                                           variant_name: ::core::convert::Into::into(name),
                                                       });
                                           }
                                       }
                                   }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(__value_param),
                                           to_kind: bevy::reflect::ReflectKind::Enum,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Enum
                    }
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Enum(self)
                    }
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Enum(self)
                    }
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Enum(self)
                    }
                    fn reflect_hash(&self) -> ::core::option::Option<u64> {
                        bevy::reflect::enum_hash(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::enum_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for TargetLocation where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Hex: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(__param0: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Enum(__param0) =
                                    bevy::reflect::Reflect::reflect_ref(__param0) {
                                match bevy::reflect::Enum::variant_name(__param0) {
                                    "None" =>
                                        ::core::option::Option::Some(TargetLocation::None {}),
                                    "Value" =>
                                        ::core::option::Option::Some(TargetLocation::Value {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <Hex as bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    name => {
                                        ::core::panicking::panic_fmt(format_args!("variant with name `{0}` does not exist on enum `{1}`",
                                                name, <Self as bevy::reflect::TypePath>::type_path()));
                                    }
                                }
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TargetLocation { }
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TargetLocation {
            #[inline]
            fn eq(&self, other: &TargetLocation) -> bool {
                let __self_discr =
                    ::core::intrinsics::discriminant_value(self);
                let __arg1_discr =
                    ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr &&
                    match (self, other) {
                        (TargetLocation::Value(__self_0),
                            TargetLocation::Value(__arg1_0)) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        pub(super) fn target_location(mut with_target_location:
                Query<(&Target, &mut TargetLocation)>,
            with_location: Query<&Location>) {
            with_target_location.par_iter_mut().for_each(|(target,
                        mut target_location)|
                    {
                        let location: TargetLocation =
                            match target {
                                Target::Cell(hex) => TargetLocation::Value(*hex),
                                Target::Entity(entity) => {
                                    if let Ok(location) = with_location.get(*entity) {
                                            if let Location::Valid(hex) = *location {
                                                    TargetLocation::Value(hex)
                                                } else { TargetLocation::None }
                                        } else { TargetLocation::None }
                                }
                                Target::None => TargetLocation::None,
                            };
                        if *target_location != location {
                                *target_location = location;
                            }
                    });
        }
        #[reflect(Component)]
        pub struct Path(Vec<Vec2>);
        impl bevy::ecs::component::Component for Path where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        impl ::std::ops::Deref for Path {
            type Target = Vec<Vec2>;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl ::std::ops::DerefMut for Path {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Path where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync,
                    Vec<Vec2>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <Vec<Vec2> as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Path where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync,
                    Vec<Vec2>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[bevy::reflect::UnnamedField::new::<Vec<Vec2>>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Path where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::navigation::path::Path"
                    }
                    fn short_type_path() -> &'static str { "Path" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Path")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::navigation::path".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::navigation::path")
                    }
                }
                impl bevy::reflect::TupleStruct for Path where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync,
                    Vec<Vec2>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    #[inline]
                    fn field_len(&self) -> usize { 1usize }
                    #[inline]
                    fn iter_fields(&self)
                        -> bevy::reflect::TupleStructFieldIter {
                        bevy::reflect::TupleStructFieldIter::new(self)
                    }
                    fn clone_dynamic(&self)
                        -> bevy::reflect::DynamicTupleStruct {
                        let mut dynamic: bevy::reflect::DynamicTupleStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed(bevy::reflect::Reflect::clone_value(&self.0));
                        dynamic
                    }
                }
                impl bevy::reflect::Reflect for Path where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync,
                    Vec<Vec2>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                    = bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                    {
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::TupleStruct::field_mut(self, i) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::TupleStruct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::TupleStruct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::tuple_struct_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Path where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync,
                    Vec<Vec2>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                    = bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        0: (||
                                                        <Vec<Vec2> as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::TupleStruct::field(__ref_struct,
                                                                    0)?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[automatically_derived]
        impl ::core::default::Default for Path {
            #[inline]
            fn default() -> Path { Path(::core::default::Default::default()) }
        }
        pub struct FindingPath(Arc<RwLock<(Option<Vec<Hex>>, bool)>>);
        impl bevy::ecs::component::Component for FindingPath where
            Self: Send + Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        pub(super) fn on_changed(mut commands: Commands,
            with_target: Query<Entity, With<Target>>,
            footprints: Query<Entity, Changed<Footprint>>,
            obstacles: Res<board::Occupied>) {
            if !obstacles.is_changed() || footprints.is_empty() { return; }
            for entity in &with_target {
                commands.entity(entity).insert(Dirty::<Path>::default());
            }
        }
        pub(super) fn on_target_changed(mut commands: Commands,
            with_target:
                Query<Entity,
                Or<(Changed<Target>, Changed<TargetLocation>)>>) {
            for entity in &with_target {
                commands.entity(entity).insert(Dirty::<Path>::default());
            }
        }
        pub(super) fn compute(mut commands: Commands,
            with_target:
                Query<(Entity, &TargetLocation, &Location),
                With<Dirty<Path>>>, occupied: Res<board::Occupied>) {
            for (entity, target, location) in &with_target {
                let Location::Valid(start) = *location else { return; };
                let TargetLocation::Value(end) =
                    *target else {
                        commands.entity(entity).remove::<Path>();
                        continue;
                    };
                if start == end {
                        commands.entity(entity).remove::<Path>();
                        continue;
                    }
                let finding =
                    FindingPath(Arc::new(RwLock::new((None, false))));
                let writer = finding.0.clone();
                let occupied = occupied.clone();
                AsyncComputeTaskPool::get().spawn(async move
                            {
                            let path =
                                a_star(start, end,
                                    |_, b|
                                        { if occupied.contains_key(&b) { 1 } else { 0 }.into() });
                            *writer.write().unwrap() = (path, true);
                        }).detach();
                commands.entity(entity).insert(finding).remove::<Dirty<Path>>();
            }
        }
        pub(super) fn poll(mut commands: Commands,
            computing: Query<(Entity, &FindingPath)>,
            board: Res<board::Board>) {
            for (entity, task) in &computing {
                let mut task = task.0.write().unwrap();
                if task.1 {
                        if let Some(path) = task.0.take() {
                                commands.entity(entity).insert(Path(path.iter().copied().skip(1).map(|h|
                                                        board.layout.hex_to_world_pos(h)).collect())).remove::<FindingPath>();
                            } else {
                               {
                                   use ::tracing::__macro_support::Callsite as _;
                                   static __CALLSITE: ::tracing::callsite::DefaultCallsite =
                                       {
                                           static META: ::tracing::Metadata<'static> =
                                               {
                                                   ::tracing_core::metadata::Metadata::new("event crates\\warband_lib\\src\\navigation\\path.rs:149",
                                                       "warband_lib::navigation::path", ::tracing::Level::INFO,
                                                       ::core::option::Option::Some("crates\\warband_lib\\src\\navigation\\path.rs"),
                                                       ::core::option::Option::Some(149u32),
                                                       ::core::option::Option::Some("warband_lib::navigation::path"),
                                                       ::tracing_core::field::FieldSet::new(&["message"],
                                                           ::tracing_core::callsite::Identifier(&__CALLSITE)),
                                                       ::tracing::metadata::Kind::EVENT)
                                               };
                                           ::tracing::callsite::DefaultCallsite::new(&META)
                                       };
                                   let enabled =
                                       ::tracing::Level::INFO <=
                                                   ::tracing::level_filters::STATIC_MAX_LEVEL &&
                                               ::tracing::Level::INFO <=
                                                   ::tracing::level_filters::LevelFilter::current() &&
                                           {
                                               let interest = __CALLSITE.interest();
                                               !interest.is_never() &&
                                                   ::tracing::__macro_support::__is_enabled(__CALLSITE.metadata(),
                                                       interest)
                                           };
                                   if enabled {
                                           (|value_set: ::tracing::field::ValueSet|
                                                       {
                                                           let meta = __CALLSITE.metadata();
                                                           ::tracing::Event::dispatch(meta, &value_set);
                                                           if match ::tracing::Level::INFO {
                                                                           ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                           ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                           ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                           ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                           _ => ::tracing::log::Level::Trace,
                                                                       } <= ::tracing::log::STATIC_MAX_LEVEL {
                                                                   if !::tracing::dispatcher::has_been_set() {
                                                                           {
                                                                               use ::tracing::log;
                                                                               let level =
                                                                                   match ::tracing::Level::INFO {
                                                                                       ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                                       ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                                       ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                                       ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                                       _ => ::tracing::log::Level::Trace,
                                                                                   };
                                                                               if level <= log::max_level() {
                                                                                       let meta = __CALLSITE.metadata();
                                                                                       let log_meta =
                                                                                           log::Metadata::builder().level(level).target(meta.target()).build();
                                                                                       let logger = log::logger();
                                                                                       if logger.enabled(&log_meta) {
                                                                                               ::tracing::__macro_support::__tracing_log(meta, logger,
                                                                                                   log_meta, &value_set)
                                                                                           }
                                                                                   }
                                                                           }
                                                                       } else { {} }
                                                               } else { {} };
                                                       })({
                                                   #[allow(unused_imports)]
                                                   use ::tracing::field::{debug, display, Value};
                                                   let mut iter = __CALLSITE.metadata().fields().iter();
                                                   __CALLSITE.metadata().fields().value_set(&[(&::core::iter::Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                                                                       ::core::option::Option::Some(&format_args!("no path found")
                                                                               as &dyn Value))])
                                               });
                                       } else {
                                          if match ::tracing::Level::INFO {
                                                          ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                          ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                          ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                          ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                          _ => ::tracing::log::Level::Trace,
                                                      } <= ::tracing::log::STATIC_MAX_LEVEL {
                                                  if !::tracing::dispatcher::has_been_set() {
                                                          {
                                                              use ::tracing::log;
                                                              let level =
                                                                  match ::tracing::Level::INFO {
                                                                      ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                      ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                      ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                      ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                      _ => ::tracing::log::Level::Trace,
                                                                  };
                                                              if level <= log::max_level() {
                                                                      let meta = __CALLSITE.metadata();
                                                                      let log_meta =
                                                                          log::Metadata::builder().level(level).target(meta.target()).build();
                                                                      let logger = log::logger();
                                                                      if logger.enabled(&log_meta) {
                                                                              ::tracing::__macro_support::__tracing_log(meta, logger,
                                                                                  log_meta,
                                                                                  &{
                                                                                          #[allow(unused_imports)]
                                                                                          use ::tracing::field::{debug, display, Value};
                                                                                          let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                          __CALLSITE.metadata().fields().value_set(&[(&::core::iter::Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                                                                                                              ::core::option::Option::Some(&format_args!("no path found")
                                                                                                                      as &dyn Value))])
                                                                                      })
                                                                          }
                                                                  }
                                                          }
                                                      } else { {} }
                                              } else { {} };
                                      }
                               };
                           }
                    }
            }
        }
    }
    pub enum NavigationSystems { Maintain, DetectChanges, Pathfinding, }
    impl bevy::ecs::schedule::SystemSet for NavigationSystems where
        Self: 'static + Send + Sync + Clone + Eq + ::std::fmt::Debug +
        ::std::hash::Hash {
        fn dyn_clone(&self)
            -> ::std::boxed::Box<dyn bevy::ecs::schedule::SystemSet> {
            ::std::boxed::Box::new(::std::clone::Clone::clone(self))
        }
        fn as_dyn_eq(&self) -> &dyn bevy::ecs::schedule::DynEq { self }
        fn dyn_hash(&self, mut state: &mut dyn ::std::hash::Hasher) {
            let ty_id = ::std::any::TypeId::of::<Self>();
            ::std::hash::Hash::hash(&ty_id, &mut state);
            ::std::hash::Hash::hash(self, &mut state);
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NavigationSystems {
        #[inline]
        fn clone(&self) -> NavigationSystems { *self }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for NavigationSystems { }
    #[automatically_derived]
    impl ::core::fmt::Debug for NavigationSystems {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f,
                match self {
                    NavigationSystems::Maintain => "Maintain",
                    NavigationSystems::DetectChanges => "DetectChanges",
                    NavigationSystems::Pathfinding => "Pathfinding",
                })
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for NavigationSystems { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for NavigationSystems {
        #[inline]
        fn eq(&self, other: &NavigationSystems) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for NavigationSystems {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for NavigationSystems {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    pub(super) fn plugin(app: &mut App) {
        app.register_type::<agent::Agent>();
        app.register_type::<path::Path>();
        app.register_type::<path::Target>();
        app.register_type::<path::TargetLocation>();
        ;
        app.configure_sets(FixedUpdate,
            (NavigationSystems::Maintain, NavigationSystems::DetectChanges,
                        NavigationSystems::Pathfinding).chain().run_if(in_state(AppState::InGame)));
        app.add_systems(FixedUpdate,
            ((required_component::<path::Target, path::TargetLocation>,
                        path::target_location).in_set(NavigationSystems::Maintain),
                (path::on_changed,
                        path::on_target_changed).in_set(NavigationSystems::DetectChanges),
                (path::compute,
                        path::poll).in_set(NavigationSystems::Pathfinding)));
    }
}
mod physics {
    use crate::prelude::*;
    mod motor {
        use crate::prelude::*;
        pub struct CharacterMotor;
        impl bevy::ecs::component::Component for CharacterMotor where
            Self: Send + Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[component(storage = "SparseSet")]
        pub struct Grounded;
        impl bevy::ecs::component::Component for Grounded where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::SparseSet;
        }
    }
    pub(super) fn plugin(app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
    }
    pub(crate) enum CollisionLayer { Player, Units, Terrain, Sensor, }
    impl PhysicsLayer for CollisionLayer {
        fn all_bits() -> u32 { 15u32 }
        fn to_bits(&self) -> u32 {
            match self {
                CollisionLayer::Player => 1u32,
                CollisionLayer::Units => 2u32,
                CollisionLayer::Terrain => 4u32,
                CollisionLayer::Sensor => 8u32,
            }
        }
    }
}
mod player {
    use crate::prelude::*;
    pub mod camera {
        use bevy::{
            core_pipeline::prepass::{DepthPrepass, NormalPrepass},
            input::mouse::MouseWheel,
        };
        use crate::{core::name::NameTags, prelude::*};
        pub(super) fn plugin(app: &mut App) {
            app.register_type::<MainCamera>();
            app.register_type::<UiCamera>();
            ;
            app.add_systems(Startup, setup);
            app.add_systems(Update, controls);
        }
        pub struct MainCamera;
        impl bevy::ecs::component::Component for MainCamera where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl ::core::default::Default for MainCamera {
            #[inline]
            fn default() -> MainCamera { MainCamera {} }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for MainCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {}
                }
                impl bevy::reflect::Typed for MainCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for MainCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::player::camera::MainCamera"
                    }
                    fn short_type_path() -> &'static str { "MainCamera" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("MainCamera")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::player::camera".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::player::camera")
                    }
                }
                impl bevy::reflect::Struct for MainCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn field(&self, name: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match name { _ => ::core::option::Option::None, }
                    }
                    fn field_mut(&mut self, name: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match name { _ => ::core::option::Option::None, }
                    }
                    fn field_at(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index { _ => ::core::option::Option::None, }
                    }
                    fn field_at_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index { _ => ::core::option::Option::None, }
                    }
                    fn name_at(&self, index: usize)
                        -> ::core::option::Option<&str> {
                        match index { _ => ::core::option::Option::None, }
                    }
                    fn field_len(&self) -> usize { 0usize }
                    fn iter_fields(&self) -> bevy::reflect::FieldIter {
                        bevy::reflect::FieldIter::new(self)
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                        let mut dynamic: bevy::reflect::DynamicStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic
                    }
                }
                impl bevy::reflect::Reflect for MainCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                    bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                    {
                                    let name =
                                        bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::Struct::field_mut(self, name) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::Struct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Struct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Struct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Struct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Struct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::struct_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for MainCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                    bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {})
                            } else { ::core::option::Option::None }
                    }
                }
            };
        pub struct UiCamera;
        impl bevy::ecs::component::Component for UiCamera where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl ::core::default::Default for UiCamera {
            #[inline]
            fn default() -> UiCamera { UiCamera {} }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for UiCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {}
                }
                impl bevy::reflect::Typed for UiCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for UiCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::player::camera::UiCamera"
                    }
                    fn short_type_path() -> &'static str { "UiCamera" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("UiCamera")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::player::camera".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::player::camera")
                    }
                }
                impl bevy::reflect::Struct for UiCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn field(&self, name: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match name { _ => ::core::option::Option::None, }
                    }
                    fn field_mut(&mut self, name: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match name { _ => ::core::option::Option::None, }
                    }
                    fn field_at(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index { _ => ::core::option::Option::None, }
                    }
                    fn field_at_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index { _ => ::core::option::Option::None, }
                    }
                    fn name_at(&self, index: usize)
                        -> ::core::option::Option<&str> {
                        match index { _ => ::core::option::Option::None, }
                    }
                    fn field_len(&self) -> usize { 0usize }
                    fn iter_fields(&self) -> bevy::reflect::FieldIter {
                        bevy::reflect::FieldIter::new(self)
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                        let mut dynamic: bevy::reflect::DynamicStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic
                    }
                }
                impl bevy::reflect::Reflect for UiCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                    bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                    {
                                    let name =
                                        bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::Struct::field_mut(self, name) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::Struct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Struct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Struct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Struct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Struct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::struct_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for UiCamera where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                    bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {})
                            } else { ::core::option::Option::None }
                    }
                }
            };
        fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
            commands.spawn((Name::camera("main"), MainCamera,
                    Camera3dBundle {
                        camera: Camera {
                            clear_color: ClearColorConfig::Custom(Color::BLACK),
                            ..default()
                        },
                        camera_3d: Camera3d::default(),
                        projection: orthographic_fixed_vertical(1.0, 30.0, -100.0,
                            200.0),
                        transform: Transform::from_xyz(-2.5, 4.5,
                                9.0).looking_at(Vec3::ZERO, Vec3::Y),
                        ..default()
                    }, IsDefaultUiCamera, DepthPrepass, NormalPrepass,
                    camera_driver::RigTransform::default(),
                    camera_driver::Zoom::with_zoom(30.0),
                    camera_driver::YawPitch::with_yaw_pitch(25.0, -45.0),
                    camera_driver::Smoothing::default().with_position(0.0).with_rotation(2.0).with_zoom(0.0)));
        }
        pub fn orthographic_fixed_vertical(height: f32, scale: f32, near: f32,
            far: f32) -> Projection {
            OrthographicProjection {
                    scale,
                    scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(height),
                    near,
                    far,
                    ..default()
                }.into()
        }
        fn controls(mut camera:
                Query<(&mut camera_driver::YawPitch,
                &mut camera_driver::Zoom), With<MainCamera>>,
            mut scroll: EventReader<MouseWheel>,
            input: Res<ButtonInput<KeyCode>>) {
            for (mut yaw_pitch, mut zoom) in &mut camera {
                let yaw_input =
                    if input.just_pressed(KeyCode::ArrowLeft) {
                                1.0
                            } else { 0.0 } -
                        if input.just_pressed(KeyCode::ArrowRight) {
                                1.0
                            } else { 0.0 };
                yaw_pitch.rotate_yaw(yaw_input * 90.0);
                let pitch_input =
                    if input.just_pressed(KeyCode::ArrowDown) {
                                1.0
                            } else { 0.0 } -
                        if input.just_pressed(KeyCode::ArrowUp) {
                                1.0
                            } else { 0.0 };
                yaw_pitch.rotate_pitch(pitch_input * 5.0);
                if input.just_pressed(KeyCode::KeyR) {
                        yaw_pitch.pitch = -35.0;
                        yaw_pitch.yaw = 180.0;
                    }
                const MAX_ZOOM: f32 = 100.0;
                const MIN_ZOOM: f32 = 1.0;
                for event in scroll.read() {
                    let zoom_scale = zoom.zoom();
                    zoom.set_zoom((zoom_scale -
                                    event.y).clamp(MIN_ZOOM, MAX_ZOOM));
                }
            }
        }
    }
    pub(super) fn plugin(app: &mut App) { app.add_plugins(camera::plugin); }
}
mod stats {
    use modifier::Modifies;
    use crate::prelude::*;
    pub mod modifier {
        use crate::prelude::*;
        use super::StatSystems;
        pub(super) fn plugin<M: Modifier<S> + Component, S: Stat +
            Component>(app: &mut App) {
            app.add_systems(PostUpdate,
                changed::<M, S>.in_set(StatSystems::Dirty));
            app.observe(removed::<M, S>);
            app.add_systems(PostUpdate,
                (add_accumulate::<M, S>,
                            accumulate::<M,
                                S>).chain().in_set(StatSystems::Accumulate));
        }
        pub enum Modifies { Single(Entity), Many(SmallVec<[Entity; 8]>), }
        impl bevy::ecs::component::Component for Modifies where Self: Send +
            Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Modifies {
            #[inline]
            fn clone(&self) -> Modifies {
                match self {
                    Modifies::Single(__self_0) =>
                        Modifies::Single(::core::clone::Clone::clone(__self_0)),
                    Modifies::Many(__self_0) =>
                        Modifies::Many(::core::clone::Clone::clone(__self_0)),
                }
            }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl bevy::reflect::GetTypeRegistration for Modifies where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    SmallVec<[Entity; 8]>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <Entity as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                        <SmallVec<[Entity; 8]> as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl bevy::reflect::Typed for Modifies where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    SmallVec<[Entity; 8]>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::NonGenericTypeInfoCell
                            =
                            bevy::reflect::utility::NonGenericTypeInfoCell::new();
                        CELL.get_or_set(||
                                {
                                    bevy::reflect::TypeInfo::Enum(bevy::reflect::EnumInfo::new::<Self>(&[bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("Single",
                                                                        &[bevy::reflect::UnnamedField::new::<Entity>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                            bevy::reflect::VariantInfo::Tuple(bevy::reflect::TupleVariantInfo::new("Many",
                                                                        &[bevy::reflect::UnnamedField::new::<SmallVec<[Entity; 8]>>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl bevy::reflect::TypePath for Modifies where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync {
                    fn type_path() -> &'static str {
                        "warband_lib::stats::modifier::Modifies"
                    }
                    fn short_type_path() -> &'static str { "Modifies" }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Modifies")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::stats::modifier".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::stats::modifier")
                    }
                }
                impl bevy::reflect::Enum for Modifies where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    SmallVec<[Entity; 8]>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, __name_param: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at(&self, __index_param: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match self {
                            Modifies::Single { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            Modifies::Many { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, __name_param: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn field_at_mut(&mut self, __index_param: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match self {
                            Modifies::Single { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            Modifies::Many { 0: value, .. } if __index_param == 0usize
                                => ::core::option::Option::Some(value),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn index_of(&self, __name_param: &str)
                        -> ::core::option::Option<usize> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn name_at(&self, __index_param: usize)
                        -> ::core::option::Option<&str> {
                        match self { _ => ::core::option::Option::None, }
                    }
                    fn iter_fields(&self) -> bevy::reflect::VariantFieldIter {
                        bevy::reflect::VariantFieldIter::new(self)
                    }
                    #[inline]
                    fn field_len(&self) -> usize {
                        match self {
                            Modifies::Single { .. } => 1usize,
                            Modifies::Many { .. } => 1usize,
                            _ => 0,
                        }
                    }
                    #[inline]
                    fn variant_name(&self) -> &str {
                        match self {
                            Modifies::Single { .. } => "Single",
                            Modifies::Many { .. } => "Many",
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_index(&self) -> usize {
                        match self {
                            Modifies::Single { .. } => 0usize,
                            Modifies::Many { .. } => 1usize,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    #[inline]
                    fn variant_type(&self) -> bevy::reflect::VariantType {
                        match self {
                            Modifies::Single { .. } =>
                                bevy::reflect::VariantType::Tuple,
                            Modifies::Many { .. } => bevy::reflect::VariantType::Tuple,
                            _ =>
                                ::core::panicking::panic("internal error: entered unreachable code"),
                        }
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicEnum {
                        bevy::reflect::DynamicEnum::from_ref::<Self>(self)
                    }
                }
                impl bevy::reflect::Reflect for Modifies where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    SmallVec<[Entity; 8]>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Enum::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        __value_param:
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(__value_param)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self,
                        __value_param: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Enum(__value_param) =
                                    bevy::reflect::Reflect::reflect_ref(__value_param) {
                                if bevy::reflect::Enum::variant_name(self) ==
                                            bevy::reflect::Enum::variant_name(__value_param) {
                                        match bevy::reflect::Enum::variant_type(__value_param) {
                                            bevy::reflect::VariantType::Struct => {
                                                for field in bevy::reflect::Enum::iter_fields(__value_param)
                                                    {
                                                    let name = field.name().unwrap();
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_mut(self, name) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            bevy::reflect::VariantType::Tuple => {
                                                for (index, field) in
                                                    ::core::iter::Iterator::enumerate(bevy::reflect::Enum::iter_fields(__value_param))
                                                    {
                                                    if let ::core::option::Option::Some(v) =
                                                                bevy::reflect::Enum::field_at_mut(self, index) {
                                                            bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                        }
                                                }
                                            }
                                            _ => {}
                                        }
                                    } else {
                                       match bevy::reflect::Enum::variant_name(__value_param) {
                                           "Single" => {
                                               *self =
                                                   Modifies::Single {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("Single"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <Entity as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<Entity as
                                                                                   bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           "Many" => {
                                               *self =
                                                   Modifies::Many {
                                                       0: {
                                                           let _0 = __value_param.field_at(0usize);
                                                           let _0 =
                                                               _0.ok_or(bevy::reflect::ApplyError::MissingEnumField {
                                                                           variant_name: ::core::convert::Into::into("Many"),
                                                                           field_name: ::core::convert::Into::into(".0"),
                                                                       })?;
                                                           <SmallVec<[Entity; 8]> as
                                                                           bevy::reflect::FromReflect>::from_reflect(_0).ok_or(bevy::reflect::ApplyError::MismatchedTypes {
                                                                       from_type: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(_0)),
                                                                       to_type: ::core::convert::Into::into(<SmallVec<[Entity; 8]>
                                                                                   as bevy::reflect::TypePath>::type_path()),
                                                                   })?
                                                       },
                                                   }
                                           }
                                           name => {
                                               return ::core::result::Result::Err(bevy::reflect::ApplyError::UnknownVariant {
                                                           enum_name: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(self)),
                                                           variant_name: ::core::convert::Into::into(name),
                                                       });
                                           }
                                       }
                                   }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(__value_param),
                                           to_kind: bevy::reflect::ReflectKind::Enum,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Enum
                    }
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Enum(self)
                    }
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Enum(self)
                    }
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Enum(self)
                    }
                    fn reflect_hash(&self) -> ::core::option::Option<u64> {
                        bevy::reflect::enum_hash(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::enum_partial_eq(self, value)
                    }
                }
                impl bevy::reflect::FromReflect for Modifies where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, Entity: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    SmallVec<[Entity; 8]>: bevy::reflect::FromReflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(__param0: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Enum(__param0) =
                                    bevy::reflect::Reflect::reflect_ref(__param0) {
                                match bevy::reflect::Enum::variant_name(__param0) {
                                    "Single" =>
                                        ::core::option::Option::Some(Modifies::Single {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <Entity as bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    "Many" =>
                                        ::core::option::Option::Some(Modifies::Many {
                                                0: {
                                                    let _0 = __param0.field_at(0usize);
                                                    let _0 = _0?;
                                                    <SmallVec<[Entity; 8]> as
                                                                bevy::reflect::FromReflect>::from_reflect(_0)?
                                                },
                                            }),
                                    name => {
                                        ::core::panicking::panic_fmt(format_args!("variant with name `{0}` does not exist on enum `{1}`",
                                                name, <Self as bevy::reflect::TypePath>::type_path()));
                                    }
                                }
                            } else { ::core::option::Option::None }
                    }
                }
            };
        #[automatically_derived]
        impl ::core::convert::From<(SmallVec<[Entity; 8]>)> for Modifies {
            #[inline]
            fn from(original: (SmallVec<[Entity; 8]>)) -> Modifies {
                Modifies::Many(original)
            }
        }
        #[automatically_derived]
        impl ::core::convert::From<(Entity)> for Modifies {
            #[inline]
            fn from(original: (Entity)) -> Modifies {
                Modifies::Single(original)
            }
        }
        pub trait Modifier<S: Stat>: Default + Send + Sync + 'static {
            fn apply(value: f32, accumulated: f32)
            -> f32;
            fn value(&self)
            -> f32;
            fn base() -> f32 { 0.0 }
        }
        #[reflect(from_reflect = false)]
        pub struct Flat<S: Stat>(pub S);
        impl<S: Stat> bevy::ecs::component::Component for Flat<S> where
            Self: Send + Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl<S: ::core::default::Default + Stat> ::core::default::Default for
            Flat<S> {
            #[inline]
            fn default() -> Flat<S> {
                Flat(::core::default::Default::default())
            }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl<S: Stat> bevy::reflect::GetTypeRegistration for Flat<S>
                    where Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <S as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl<S: Stat> bevy::reflect::Typed for Flat<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::GenericTypeInfoCell =
                            bevy::reflect::utility::GenericTypeInfoCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[bevy::reflect::UnnamedField::new::<S>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl<S: Stat> bevy::reflect::TypePath for Flat<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath {
                    fn type_path() -> &'static str {
                        static CELL: bevy::reflect::utility::GenericTypePathCell =
                            bevy::reflect::utility::GenericTypePathCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    ::std::string::ToString::to_string("warband_lib::stats::modifier::Flat<")
                                            + <S as bevy::reflect::TypePath>::type_path() + ">"
                                })
                    }
                    fn short_type_path() -> &'static str {
                        static CELL: bevy::reflect::utility::GenericTypePathCell =
                            bevy::reflect::utility::GenericTypePathCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    ::std::string::ToString::to_string("Flat<") +
                                            <S as bevy::reflect::TypePath>::short_type_path() + ">"
                                })
                    }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Flat")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::stats::modifier".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::stats::modifier")
                    }
                }
                impl<S: Stat> bevy::reflect::TupleStruct for Flat<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    #[inline]
                    fn field_len(&self) -> usize { 1usize }
                    #[inline]
                    fn iter_fields(&self)
                        -> bevy::reflect::TupleStructFieldIter {
                        bevy::reflect::TupleStructFieldIter::new(self)
                    }
                    fn clone_dynamic(&self)
                        -> bevy::reflect::DynamicTupleStruct {
                        let mut dynamic: bevy::reflect::DynamicTupleStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed(bevy::reflect::Reflect::clone_value(&self.0));
                        dynamic
                    }
                }
                impl<S: Stat> bevy::reflect::Reflect for Flat<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                    = bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                    {
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::TupleStruct::field_mut(self, i) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::TupleStruct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::TupleStruct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::tuple_struct_partial_eq(self, value)
                    }
                }
            };
        const _: () =
            {
                impl<S: Stat> bevy::reflect::FromReflect for Flat<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                    = bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        0: (||
                                                        <S as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::TupleStruct::field(__ref_struct,
                                                                    0)?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        impl<S: Stat> ::std::ops::Deref for Flat<S> {
            type Target = S;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl<S: Stat> ::std::ops::DerefMut for Flat<S> {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
        #[automatically_derived]
        impl<S: Stat> ::core::convert::From<(S)> for Flat<S> {
            #[inline]
            fn from(original: (S)) -> Flat<S> { Flat(original) }
        }
        impl<S: Stat, M: Stat> Modifier<S> for Flat<M> {
            fn apply(value: f32, accumulated: f32) -> f32 {
                value + accumulated
            }
            fn value(&self) -> f32 { self.0.value() }
        }
        #[reflect(from_reflect = false)]
        pub struct Additive<S: Stat>(pub S);
        impl<S: Stat> bevy::ecs::component::Component for Additive<S> where
            Self: Send + Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl<S: ::core::default::Default + Stat> ::core::default::Default for
            Additive<S> {
            #[inline]
            fn default() -> Additive<S> {
                Additive(::core::default::Default::default())
            }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl<S: Stat> bevy::reflect::GetTypeRegistration for
                    Additive<S> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    S: bevy::reflect::TypePath, S: bevy::reflect::Reflect +
                    bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <S as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl<S: Stat> bevy::reflect::Typed for Additive<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::GenericTypeInfoCell =
                            bevy::reflect::utility::GenericTypeInfoCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[bevy::reflect::UnnamedField::new::<S>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl<S: Stat> bevy::reflect::TypePath for Additive<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath {
                    fn type_path() -> &'static str {
                        static CELL: bevy::reflect::utility::GenericTypePathCell =
                            bevy::reflect::utility::GenericTypePathCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    ::std::string::ToString::to_string("warband_lib::stats::modifier::Additive<")
                                            + <S as bevy::reflect::TypePath>::type_path() + ">"
                                })
                    }
                    fn short_type_path() -> &'static str {
                        static CELL: bevy::reflect::utility::GenericTypePathCell =
                            bevy::reflect::utility::GenericTypePathCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    ::std::string::ToString::to_string("Additive<") +
                                            <S as bevy::reflect::TypePath>::short_type_path() + ">"
                                })
                    }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Additive")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::stats::modifier".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::stats::modifier")
                    }
                }
                impl<S: Stat> bevy::reflect::TupleStruct for Additive<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    #[inline]
                    fn field_len(&self) -> usize { 1usize }
                    #[inline]
                    fn iter_fields(&self)
                        -> bevy::reflect::TupleStructFieldIter {
                        bevy::reflect::TupleStructFieldIter::new(self)
                    }
                    fn clone_dynamic(&self)
                        -> bevy::reflect::DynamicTupleStruct {
                        let mut dynamic: bevy::reflect::DynamicTupleStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed(bevy::reflect::Reflect::clone_value(&self.0));
                        dynamic
                    }
                }
                impl<S: Stat> bevy::reflect::Reflect for Additive<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                    = bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                    {
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::TupleStruct::field_mut(self, i) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::TupleStruct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::TupleStruct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::tuple_struct_partial_eq(self, value)
                    }
                }
            };
        const _: () =
            {
                impl<S: Stat> bevy::reflect::FromReflect for Additive<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                    = bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        0: (||
                                                        <S as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::TupleStruct::field(__ref_struct,
                                                                    0)?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        impl<S: Stat> ::std::ops::Deref for Additive<S> {
            type Target = S;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl<S: Stat> ::std::ops::DerefMut for Additive<S> {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
        #[automatically_derived]
        impl<S: Stat> ::core::convert::From<(S)> for Additive<S> {
            #[inline]
            fn from(original: (S)) -> Additive<S> { Additive(original) }
        }
        impl<S: Stat, M: Stat> Modifier<S> for Additive<M> {
            fn apply(value: f32, accumulated: f32) -> f32 {
                value * accumulated
            }
            fn value(&self) -> f32 { self.0.value() }
            fn base() -> f32 { 1.0 }
        }
        #[reflect(from_reflect = false)]
        pub struct Mult<S: Stat>(pub S);
        impl<S: Stat> bevy::ecs::component::Component for Mult<S> where
            Self: Send + Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;
        }
        #[automatically_derived]
        impl<S: ::core::default::Default + Stat> ::core::default::Default for
            Mult<S> {
            #[inline]
            fn default() -> Mult<S> {
                Mult(::core::default::Default::default())
            }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl<S: Stat> bevy::reflect::GetTypeRegistration for Mult<S>
                    where Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <S as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl<S: Stat> bevy::reflect::Typed for Mult<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::GenericTypeInfoCell =
                            bevy::reflect::utility::GenericTypeInfoCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    bevy::reflect::TypeInfo::TupleStruct(bevy::reflect::TupleStructInfo::new::<Self>(&[bevy::reflect::UnnamedField::new::<S>(0usize).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl<S: Stat> bevy::reflect::TypePath for Mult<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath {
                    fn type_path() -> &'static str {
                        static CELL: bevy::reflect::utility::GenericTypePathCell =
                            bevy::reflect::utility::GenericTypePathCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    ::std::string::ToString::to_string("warband_lib::stats::modifier::Mult<")
                                            + <S as bevy::reflect::TypePath>::type_path() + ">"
                                })
                    }
                    fn short_type_path() -> &'static str {
                        static CELL: bevy::reflect::utility::GenericTypePathCell =
                            bevy::reflect::utility::GenericTypePathCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    ::std::string::ToString::to_string("Mult<") +
                                            <S as bevy::reflect::TypePath>::short_type_path() + ">"
                                })
                    }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Mult")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::stats::modifier".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::stats::modifier")
                    }
                }
                impl<S: Stat> bevy::reflect::TupleStruct for Mult<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.0),
                            _ => ::core::option::Option::None,
                        }
                    }
                    #[inline]
                    fn field_len(&self) -> usize { 1usize }
                    #[inline]
                    fn iter_fields(&self)
                        -> bevy::reflect::TupleStructFieldIter {
                        bevy::reflect::TupleStructFieldIter::new(self)
                    }
                    fn clone_dynamic(&self)
                        -> bevy::reflect::DynamicTupleStruct {
                        let mut dynamic: bevy::reflect::DynamicTupleStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed(bevy::reflect::Reflect::clone_value(&self.0));
                        dynamic
                    }
                }
                impl<S: Stat> bevy::reflect::Reflect for Mult<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::Reflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::TupleStruct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::TupleStruct(struct_value)
                                    = bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::TupleStruct::iter_fields(struct_value))
                                    {
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::TupleStruct::field_mut(self, i) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::TupleStruct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::TupleStruct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::TupleStruct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::TupleStruct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::tuple_struct_partial_eq(self, value)
                    }
                }
            };
        const _: () =
            {
                impl<S: Stat> bevy::reflect::FromReflect for Mult<S> where
                    Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, S: bevy::reflect::TypePath,
                    S: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::TupleStruct(__ref_struct)
                                    = bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        0: (||
                                                        <S as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::TupleStruct::field(__ref_struct,
                                                                    0)?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        impl<S: Stat> ::std::ops::Deref for Mult<S> {
            type Target = S;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl<S: Stat> ::std::ops::DerefMut for Mult<S> {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
        #[automatically_derived]
        impl<S: Stat> ::core::convert::From<(S)> for Mult<S> {
            #[inline]
            fn from(original: (S)) -> Mult<S> { Mult(original) }
        }
        impl<S: Stat, M: Stat> Modifier<S> for Mult<M> {
            fn apply(value: f32, accumulated: f32) -> f32 {
                value * accumulated
            }
            fn value(&self) -> f32 { self.0.value() }
            fn base() -> f32 { 1.0 }
        }
        #[reflect(Component)]
        #[component(storage = "SparseSet")]
        pub struct Accumulated<M: Modifier<S>, S: Stat> {
            pub value: f32,
            pub _marker: std::marker::PhantomData<(M, S)>,
        }
        impl<M: Modifier<S>, S: Stat> bevy::ecs::component::Component for
            Accumulated<M, S> where Self: Send + Sync + 'static {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::SparseSet;
        }
        #[automatically_derived]
        impl<M: ::core::default::Default + Modifier<S>,
            S: ::core::default::Default + Stat> ::core::default::Default for
            Accumulated<M, S> {
            #[inline]
            fn default() -> Accumulated<M, S> {
                Accumulated {
                    value: ::core::default::Default::default(),
                    _marker: ::core::default::Default::default(),
                }
            }
        }
        const _: () =
            {
                #[allow(unused_mut)]
                impl<M: Modifier<S>, S: Stat>
                    bevy::reflect::GetTypeRegistration for Accumulated<M, S>
                    where Self: ::core::any::Any + ::core::marker::Send +
                    ::core::marker::Sync, M: bevy::reflect::TypePath,
                    S: bevy::reflect::TypePath,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    std::marker::PhantomData<(M,
                    S)>: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn get_type_registration()
                        -> bevy::reflect::TypeRegistration {
                        let mut registration =
                            bevy::reflect::TypeRegistration::of::<Self>();
                        registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                        registration.insert::<ReflectComponent>(bevy::reflect::FromType::<Self>::from_type());
                        registration
                    }
                    #[inline(never)]
                    fn register_type_dependencies(registry:
                            &mut bevy::reflect::TypeRegistry) {
                        <f32 as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                        <std::marker::PhantomData<(M, S)> as
                                bevy::reflect::__macro_exports::RegisterForReflection>::__register(registry);
                    }
                }
                impl<M: Modifier<S>, S: Stat> bevy::reflect::Typed for
                    Accumulated<M, S> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    M: bevy::reflect::TypePath, S: bevy::reflect::TypePath,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    std::marker::PhantomData<(M,
                    S)>: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn type_info() -> &'static bevy::reflect::TypeInfo {
                        static CELL: bevy::reflect::utility::GenericTypeInfoCell =
                            bevy::reflect::utility::GenericTypeInfoCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    bevy::reflect::TypeInfo::Struct(bevy::reflect::StructInfo::new::<Self>(&[bevy::reflect::NamedField::new::<f32>("value").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()),
                                                            bevy::reflect::NamedField::new::<std::marker::PhantomData<(M,
                                                                        S)>>("_marker").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                                })
                    }
                }
                impl<M: Modifier<S>, S: Stat> bevy::reflect::TypePath for
                    Accumulated<M, S> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    M: bevy::reflect::TypePath, S: bevy::reflect::TypePath {
                    fn type_path() -> &'static str {
                        static CELL: bevy::reflect::utility::GenericTypePathCell =
                            bevy::reflect::utility::GenericTypePathCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    ::std::string::ToString::to_string("warband_lib::stats::modifier::Accumulated<")
                                                    +
                                                    &::std::string::ToString::to_string(<M as
                                                                    bevy::reflect::TypePath>::type_path()) + ", " +
                                            <S as bevy::reflect::TypePath>::type_path() + ">"
                                })
                    }
                    fn short_type_path() -> &'static str {
                        static CELL: bevy::reflect::utility::GenericTypePathCell =
                            bevy::reflect::utility::GenericTypePathCell::new();
                        CELL.get_or_insert::<Self,
                            _>(||
                                {
                                    ::std::string::ToString::to_string("Accumulated<") +
                                                    &::std::string::ToString::to_string(<M as
                                                                    bevy::reflect::TypePath>::short_type_path()) + ", " +
                                            <S as bevy::reflect::TypePath>::short_type_path() + ">"
                                })
                    }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Accumulated")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::stats::modifier".split(':').next().unwrap())
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some("warband_lib::stats::modifier")
                    }
                }
                impl<M: Modifier<S>, S: Stat> bevy::reflect::Struct for
                    Accumulated<M, S> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    M: bevy::reflect::TypePath, S: bevy::reflect::TypePath,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    std::marker::PhantomData<(M,
                    S)>: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn field(&self, name: &str)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match name {
                            "value" => ::core::option::Option::Some(&self.value),
                            "_marker" => ::core::option::Option::Some(&self._marker),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_mut(&mut self, name: &str)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match name {
                            "value" => ::core::option::Option::Some(&mut self.value),
                            "_marker" =>
                                ::core::option::Option::Some(&mut self._marker),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_at(&self, index: usize)
                        -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&self.value),
                            1usize => ::core::option::Option::Some(&self._marker),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_at_mut(&mut self, index: usize)
                        -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                        match index {
                            0usize => ::core::option::Option::Some(&mut self.value),
                            1usize => ::core::option::Option::Some(&mut self._marker),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn name_at(&self, index: usize)
                        -> ::core::option::Option<&str> {
                        match index {
                            0usize => ::core::option::Option::Some("value"),
                            1usize => ::core::option::Option::Some("_marker"),
                            _ => ::core::option::Option::None,
                        }
                    }
                    fn field_len(&self) -> usize { 2usize }
                    fn iter_fields(&self) -> bevy::reflect::FieldIter {
                        bevy::reflect::FieldIter::new(self)
                    }
                    fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
                        let mut dynamic: bevy::reflect::DynamicStruct =
                            ::core::default::Default::default();
                        dynamic.set_represented_type(bevy::reflect::Reflect::get_represented_type_info(self));
                        dynamic.insert_boxed("value",
                            bevy::reflect::Reflect::clone_value(&self.value));
                        dynamic.insert_boxed("_marker",
                            bevy::reflect::Reflect::clone_value(&self._marker));
                        dynamic
                    }
                }
                impl<M: Modifier<S>, S: Stat> bevy::reflect::Reflect for
                    Accumulated<M, S> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    M: bevy::reflect::TypePath, S: bevy::reflect::TypePath,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    std::marker::PhantomData<(M,
                    S)>: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    #[inline]
                    fn get_represented_type_info(&self)
                        ->
                            ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                        ::core::option::Option::Some(<Self as
                                    bevy::reflect::Typed>::type_info())
                    }
                    #[inline]
                    fn into_any(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn ::core::any::Any> {
                        self
                    }
                    #[inline]
                    fn as_any(&self) -> &dyn ::core::any::Any { self }
                    #[inline]
                    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                        self
                    }
                    #[inline]
                    fn into_reflect(self: ::std::boxed::Box<Self>)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        self
                    }
                    #[inline]
                    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
                    #[inline]
                    fn as_reflect_mut(&mut self)
                        -> &mut dyn bevy::reflect::Reflect {
                        self
                    }
                    #[inline]
                    fn clone_value(&self)
                        -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                        ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
                    }
                    #[inline]
                    fn set(&mut self,
                        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                        ->
                            ::core::result::Result<(),
                            ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                        *self = <dyn bevy::reflect::Reflect>::take(value)?;
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn try_apply(&mut self, value: &dyn bevy::reflect::Reflect)
                        -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                        if let bevy::reflect::ReflectRef::Struct(struct_value) =
                                    bevy::reflect::Reflect::reflect_ref(value) {
                                for (i, value) in
                                    ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
                                    {
                                    let name =
                                        bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                                    if let ::core::option::Option::Some(v) =
                                                bevy::reflect::Struct::field_mut(self, name) {
                                            bevy::reflect::Reflect::try_apply(v, value)?;
                                        }
                                }
                            } else {
                               return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                           from_kind: bevy::reflect::Reflect::reflect_kind(value),
                                           to_kind: bevy::reflect::ReflectKind::Struct,
                                       });
                           }
                        ::core::result::Result::Ok(())
                    }
                    #[inline]
                    fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                        bevy::reflect::ReflectKind::Struct
                    }
                    #[inline]
                    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                        bevy::reflect::ReflectRef::Struct(self)
                    }
                    #[inline]
                    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                        bevy::reflect::ReflectMut::Struct(self)
                    }
                    #[inline]
                    fn reflect_owned(self: ::std::boxed::Box<Self>)
                        -> bevy::reflect::ReflectOwned {
                        bevy::reflect::ReflectOwned::Struct(self)
                    }
                    fn reflect_partial_eq(&self,
                        value: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<bool> {
                        bevy::reflect::struct_partial_eq(self, value)
                    }
                }
                impl<M: Modifier<S>, S: Stat> bevy::reflect::FromReflect for
                    Accumulated<M, S> where Self: ::core::any::Any +
                    ::core::marker::Send + ::core::marker::Sync,
                    M: bevy::reflect::TypePath, S: bevy::reflect::TypePath,
                    f32: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection,
                    std::marker::PhantomData<(M,
                    S)>: bevy::reflect::FromReflect + bevy::reflect::TypePath +
                    bevy::reflect::__macro_exports::RegisterForReflection {
                    fn from_reflect(reflect: &dyn bevy::reflect::Reflect)
                        -> ::core::option::Option<Self> {
                        if let bevy::reflect::ReflectRef::Struct(__ref_struct) =
                                    bevy::reflect::Reflect::reflect_ref(reflect) {
                                ::core::option::Option::Some(Self {
                                        value: (||
                                                        <f32 as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "value")?))()?,
                                        _marker: (||
                                                        <std::marker::PhantomData<(M, S)> as
                                                                bevy::reflect::FromReflect>::from_reflect(bevy::reflect::Struct::field(__ref_struct,
                                                                    "_marker")?))()?,
                                    })
                            } else { ::core::option::Option::None }
                    }
                }
            };
        impl<M: Modifier<S>, S: Stat> Accumulated<M, S> {
            fn new(value: f32) -> Self {
                Self { value, _marker: std::marker::PhantomData }
            }
            fn add(&mut self, value: f32) { self.value += value; }
            fn compute(&self, value: f32) -> f32 {
                M::apply(value, self.value)
            }
        }
        type NonDirtyStatFilter<S> = (With<S>, Without<Dirty<S>>);
        /// note: registered per [Stat] in [super::stat::plugin]
        pub(super) fn modifies_changed<S: Stat>(mut commands: Commands,
            mut stats: Query<Entity, NonDirtyStatFilter<S>>,
            modifiers:
                Query<(&Modifies, &Previous<Modifies>), Changed<Modifies>>)
            where S: Component {
            let mut add_dirty_stat =
                |entity: &Entity|
                    {
                        if stats.get_mut(*entity).is_ok() {
                                commands.entity(*entity).insert(Dirty::<S>::default());
                            }
                    };
            for (modifier, previous) in modifiers.iter() {
                for target in [modifier, previous.get()] {
                    match target {
                        Modifies::Single(entity) => add_dirty_stat(entity),
                        Modifies::Many(entities) => {
                            for entity in entities.iter() { add_dirty_stat(entity) }
                        }
                    }
                }
            }
        }
        fn changed<M: Modifier<S>,
            S: Stat>(mut commands: Commands,
            mut stats: Query<Entity, NonDirtyStatFilter<S>>,
            modifiers:
                Query<(Entity, Option<&Parent>, Option<&Modifies>),
                Changed<M>>, modifier_parents: Query<(Entity, &Modifies)>)
            where M: Component + Modifier<S>, S: Component {
            let mut add_dirty_stat =
                |entity: Entity|
                    {
                        if stats.get_mut(entity).is_ok() {
                                commands.entity(entity).insert(Dirty::<S>::default());
                            }
                    };
            for (entity, maybe_parent, maybe_target) in &modifiers {
                let modifier_target =
                    maybe_target.or(maybe_parent.and_then(|p|
                                modifier_parents.get(p.get()).ok().map(|(_, t)| t)));
                match modifier_target {
                    Some(Modifies::Single(entity)) => add_dirty_stat(*entity),
                    Some(Modifies::Many(entities)) => {
                        for entity in entities.iter() { add_dirty_stat(*entity); }
                    }
                    None => {
                        if let Some(parent) = maybe_parent {
                                add_dirty_stat(parent.get())
                            }
                        add_dirty_stat(entity);
                    }
                }
            }
        }
        fn removed<M: Modifier<S>,
            S: Stat>(trigger: Trigger<OnRemove, M>,
            non_dirty: Query<Entity, NonDirtyStatFilter<S>>,
            modifiers:
                Query<(Entity, Option<&Parent>, Option<&Modifies>), With<M>>,
            modifier_parents: Query<(Entity, &Modifies)>,
            mut commands: Commands) where M: Component + Modifier<S>,
            S: Component {
            let entity: Entity = trigger.entity();
            let Ok((entity, maybe_parent, maybe_target)) =
                modifiers.get(entity) else { return; };
            let mut add_dirty_stat =
                |entity: &Entity|
                    {
                        if let Ok(stat) = non_dirty.get(*entity) {
                                commands.entity(stat).insert(Dirty::<S>::default());
                            }
                    };
            let modifier_target =
                maybe_target.or(maybe_parent.and_then(|p|
                            modifier_parents.get(p.get()).ok().map(|(_, t)| t)));
            match modifier_target {
                Some(Modifies::Single(entity)) => add_dirty_stat(entity),
                Some(Modifies::Many(entities)) => {
                    for entity in entities.iter() { add_dirty_stat(entity); }
                }
                None => {
                    if let Some(parent) = maybe_parent {
                            add_dirty_stat(&parent.get());
                        }
                    add_dirty_stat(&entity);
                }
            }
        }
        fn add_accumulate<M: Modifier<S>, S: Stat +
            Component>(with_dirty: Query<Entity, With<Dirty<S>>>,
            mut commands: Commands) {
            for entity in &with_dirty {
                let accumulated = Accumulated::<M, S>::new(M::base());
                commands.entity(entity).insert(accumulated);
            }
        }
        fn accumulate<M: Modifier<S>,
            S: Stat>(mut stats: Query<&mut Accumulated<M, S>, With<Dirty<S>>>,
            modifiers:
                Query<(Entity, &M, Option<&Parent>, Option<&Modifies>)>,
            modifier_parents: Query<(Entity, &Modifies)>) where M: Component +
            Modifier<S>, S: Component {
            for (entity, modifier, maybe_parent, maybe_target) in
                modifiers.iter() {
                let mut apply_modifier =
                    |entity: &Entity|
                        {
                            if let Ok(mut accumulated) = stats.get_mut(*entity) {
                                    accumulated.add(modifier.value());
                                }
                        };
                let modifier_target =
                    maybe_target.or(maybe_parent.and_then(|p|
                                modifier_parents.get(p.get()).ok().map(|(_, t)| t)));
                match modifier_target {
                    Some(Modifies::Single(entity)) => apply_modifier(entity),
                    Some(Modifies::Many(entities)) => {
                        for entity in entities.iter() { apply_modifier(entity); }
                    }
                    None => {
                        if let Some(parent) = maybe_parent {
                                apply_modifier(&parent.get());
                            }
                        apply_modifier(&entity);
                    }
                }
            }
        }
        /// note: registered per [Stat] in [super::stat::plugin]
        pub(super) fn compute<S: Stat +
            Component>(mut stat:
                Query<(Entity, &mut S, &Accumulated<Flat<S>, S>,
                &Accumulated<Additive<S>, S>, &Accumulated<Mult<S>, S>),
                With<Dirty<S>>>, mut commands: Commands) {
            for (entity, mut stat, flat, additive, multiplicative) in
                &mut stat {
                const BASE: f32 = 0.0;
                let computed =
                    multiplicative.compute(additive.compute(flat.compute(BASE)));
                let computed = S::clamp(S::round(computed));
                if stat.value() != computed { *stat = S::new(computed); }
                commands.entity(entity).remove::<Accumulated<Flat<S>,
                            S>>().remove::<Accumulated<Additive<S>,
                        S>>().remove::<Accumulated<Mult<S>, S>>();
            }
        }
    }
    pub mod stat {
        use super::{modifier, StatSystems};
        use crate::prelude::*;
        pub(crate) fn plugin<S: Stat>(app: &mut App) where S: Component +
            GetTypeRegistration {
            app.register_type::<S>();
            app.register_type::<Dirty<S>>();
            app.register_type::<modifier::Flat<S>>();
            app.register_type::<modifier::Additive<S>>();
            app.register_type::<modifier::Mult<S>>();
            ;
            app.add_plugins((modifier::plugin::<modifier::Flat<S>, S>,
                    modifier::plugin::<modifier::Additive<S>, S>,
                    modifier::plugin::<modifier::Mult<S>, S>));
            app.observe(on_insert::<S>);
            app.observe(on_remove::<S>);
            app.add_systems(PostUpdate,
                modifier::modifies_changed::<S>.in_set(StatSystems::Dirty));
            app.add_systems(PostUpdate,
                modifier::compute::<S>.in_set(StatSystems::Compute));
            app.add_systems(PostUpdate,
                dirty_remove::<S>.in_set(StatSystems::Cleanup));
        }
        pub trait Stat: Reflect + TypePath + Default + Sync + Send + Sized +
            Copy + 'static {
            /// Creates a new [Stat] with the given value.
            fn new(value: f32)
            -> Self;
            /// Returns the value of the [Stat].
            fn value(&self)
            -> f32;
            /// Returns a [modifier::Flat] modifier with the given value of [Stat]
            fn flat(self) -> modifier::Flat<Self> {
                modifier::Flat::from(self)
            }
            /// Returns a [modifier::Additive] modifier with the given value of [Stat]
            fn additive(self) -> modifier::Additive<Self> {
                modifier::Additive::from(self)
            }
            /// Returns a [modifier::Mult] modifier with the given value of [Stat]
            fn mult(self) -> modifier::Mult<Self> {
                modifier::Mult::from(self)
            }
            /// Clamps the value of the [Stat].
            fn clamp(value: f32) -> f32 { value }
            /// Rounds the value of the [Stat].
            fn round(value: f32) -> f32 { value }
        }
        fn on_insert<S: Stat +
            Component>(trigger: Trigger<OnAdd, S>, stat: Query<&S>,
            mut commands: Commands) {
            let entity = trigger.entity();
            let stat = stat.get(entity).unwrap();
            commands.entity(entity).insert(modifier::Flat::<S>::from(*stat)).insert(Dirty::<S>::default());
        }
        fn on_remove<S: Stat +
            Component>(trigger: Trigger<OnRemove, S>,
            without_modifies: Query<Entity, Without<modifier::Modifies>>,
            mut commands: Commands) {
            let entity = trigger.entity();
            if without_modifies.get(entity).is_ok() {
                    commands.entity(entity).remove::<modifier::Flat<S>>().remove::<Dirty<S>>();
                }
        }
        fn dirty_remove<S: Stat +
            Component>(with_dirty: Query<Entity, With<Dirty<S>>>,
            mut commands: Commands) {
            for entity in &with_dirty {
                commands.entity(entity).remove::<Dirty<S>>();
            }
        }
    }
    pub enum StatSystems { Dirty, Accumulate, Compute, Cleanup, }
    impl bevy::ecs::schedule::SystemSet for StatSystems where Self: 'static +
        Send + Sync + Clone + Eq + ::std::fmt::Debug + ::std::hash::Hash {
        fn dyn_clone(&self)
            -> ::std::boxed::Box<dyn bevy::ecs::schedule::SystemSet> {
            ::std::boxed::Box::new(::std::clone::Clone::clone(self))
        }
        fn as_dyn_eq(&self) -> &dyn bevy::ecs::schedule::DynEq { self }
        fn dyn_hash(&self, mut state: &mut dyn ::std::hash::Hasher) {
            let ty_id = ::std::any::TypeId::of::<Self>();
            ::std::hash::Hash::hash(&ty_id, &mut state);
            ::std::hash::Hash::hash(self, &mut state);
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StatSystems {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f,
                match self {
                    StatSystems::Dirty => "Dirty",
                    StatSystems::Accumulate => "Accumulate",
                    StatSystems::Compute => "Compute",
                    StatSystems::Cleanup => "Cleanup",
                })
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for StatSystems {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StatSystems { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StatSystems {
        #[inline]
        fn eq(&self, other: &StatSystems) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StatSystems {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StatSystems {
        #[inline]
        fn clone(&self) -> StatSystems {
            match self {
                StatSystems::Dirty => StatSystems::Dirty,
                StatSystems::Accumulate => StatSystems::Accumulate,
                StatSystems::Compute => StatSystems::Compute,
                StatSystems::Cleanup => StatSystems::Cleanup,
            }
        }
    }
    pub(super) fn plugin(app: &mut App) {
        app.register_type::<modifier::Modifies>();
        app.register_type::<Previous<modifier::Modifies>>();
        ;
        app.configure_sets(PostUpdate,
            (StatSystems::Dirty, StatSystems::Accumulate,
                    StatSystems::Compute, StatSystems::Cleanup).chain());
        app.add_systems(PostUpdate,
            propagate_previous_changed::<Modifies>.in_set(StatSystems::Dirty));
    }
}
pub enum AppState {

    #[default]
    Loading,
    InGame,
}
impl bevy::state::state::States for AppState {}
impl bevy::state::state::FreelyMutableState for AppState {}
#[automatically_derived]
impl ::core::default::Default for AppState {
    #[inline]
    fn default() -> AppState { Self::Loading }
}
#[automatically_derived]
impl ::core::clone::Clone for AppState {
    #[inline]
    fn clone(&self) -> AppState {
        match self {
            AppState::Loading => AppState::Loading,
            AppState::InGame => AppState::InGame,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for AppState {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for AppState { }
#[automatically_derived]
impl ::core::cmp::PartialEq for AppState {
    #[inline]
    fn eq(&self, other: &AppState) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for AppState {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f,
            match self {
                AppState::Loading => "Loading",
                AppState::InGame => "InGame",
            })
    }
}
#[automatically_derived]
impl ::core::hash::Hash for AppState {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_discr, state)
    }
}
const _: () =
    {
        #[allow(unused_mut)]
        impl bevy::reflect::GetTypeRegistration for AppState where
            Self: ::core::any::Any + ::core::marker::Send +
            ::core::marker::Sync {
            fn get_type_registration() -> bevy::reflect::TypeRegistration {
                let mut registration =
                    bevy::reflect::TypeRegistration::of::<Self>();
                registration.insert::<bevy::reflect::ReflectFromPtr>(bevy::reflect::FromType::<Self>::from_type());
                registration.insert::<bevy::reflect::ReflectFromReflect>(bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry:
                    &mut bevy::reflect::TypeRegistry) {}
        }
        impl bevy::reflect::Typed for AppState where Self: ::core::any::Any +
            ::core::marker::Send + ::core::marker::Sync {
            fn type_info() -> &'static bevy::reflect::TypeInfo {
                static CELL: bevy::reflect::utility::NonGenericTypeInfoCell =
                    bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(||
                        {
                            bevy::reflect::TypeInfo::Enum(bevy::reflect::EnumInfo::new::<Self>(&[bevy::reflect::VariantInfo::Unit(bevy::reflect::UnitVariantInfo::new("Loading").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default())),
                                                    bevy::reflect::VariantInfo::Unit(bevy::reflect::UnitVariantInfo::new("InGame").with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))]).with_custom_attributes(bevy::reflect::attributes::CustomAttributes::default()))
                        })
            }
        }
        impl bevy::reflect::TypePath for AppState where
            Self: ::core::any::Any + ::core::marker::Send +
            ::core::marker::Sync {
            fn type_path() -> &'static str { "warband_lib::AppState" }
            fn short_type_path() -> &'static str { "AppState" }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("AppState")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some("warband_lib".split(':').next().unwrap())
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("warband_lib")
            }
        }
        impl bevy::reflect::Enum for AppState where Self: ::core::any::Any +
            ::core::marker::Send + ::core::marker::Sync {
            fn field(&self, __name_param: &str)
                -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                match self { _ => ::core::option::Option::None, }
            }
            fn field_at(&self, __index_param: usize)
                -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                match self { _ => ::core::option::Option::None, }
            }
            fn field_mut(&mut self, __name_param: &str)
                -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                match self { _ => ::core::option::Option::None, }
            }
            fn field_at_mut(&mut self, __index_param: usize)
                -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                match self { _ => ::core::option::Option::None, }
            }
            fn index_of(&self, __name_param: &str)
                -> ::core::option::Option<usize> {
                match self { _ => ::core::option::Option::None, }
            }
            fn name_at(&self, __index_param: usize)
                -> ::core::option::Option<&str> {
                match self { _ => ::core::option::Option::None, }
            }
            fn iter_fields(&self) -> bevy::reflect::VariantFieldIter {
                bevy::reflect::VariantFieldIter::new(self)
            }
            #[inline]
            fn field_len(&self) -> usize {
                match self {
                    AppState::Loading { .. } => 0usize,
                    AppState::InGame { .. } => 0usize,
                    _ => 0,
                }
            }
            #[inline]
            fn variant_name(&self) -> &str {
                match self {
                    AppState::Loading { .. } => "Loading",
                    AppState::InGame { .. } => "InGame",
                    _ =>
                        ::core::panicking::panic("internal error: entered unreachable code"),
                }
            }
            #[inline]
            fn variant_index(&self) -> usize {
                match self {
                    AppState::Loading { .. } => 0usize,
                    AppState::InGame { .. } => 1usize,
                    _ =>
                        ::core::panicking::panic("internal error: entered unreachable code"),
                }
            }
            #[inline]
            fn variant_type(&self) -> bevy::reflect::VariantType {
                match self {
                    AppState::Loading { .. } =>
                        bevy::reflect::VariantType::Unit,
                    AppState::InGame { .. } => bevy::reflect::VariantType::Unit,
                    _ =>
                        ::core::panicking::panic("internal error: entered unreachable code"),
                }
            }
            fn clone_dynamic(&self) -> bevy::reflect::DynamicEnum {
                bevy::reflect::DynamicEnum::from_ref::<Self>(self)
            }
        }
        impl bevy::reflect::Reflect for AppState where
            Self: ::core::any::Any + ::core::marker::Send +
            ::core::marker::Sync {
            #[inline]
            fn get_represented_type_info(&self)
                -> ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(<Self as
                            bevy::reflect::Typed>::type_info())
            }
            #[inline]
            fn into_any(self: ::std::boxed::Box<Self>)
                -> ::std::boxed::Box<dyn ::core::any::Any> {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any { self }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any { self }
            #[inline]
            fn into_reflect(self: ::std::boxed::Box<Self>)
                -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn bevy::reflect::Reflect { self }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn clone_value(&self)
                -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                ::std::boxed::Box::new(bevy::reflect::Enum::clone_dynamic(self))
            }
            #[inline]
            fn set(&mut self,
                __value_param: ::std::boxed::Box<dyn bevy::reflect::Reflect>)
                ->
                    ::core::result::Result<(),
                    ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
                *self = <dyn bevy::reflect::Reflect>::take(__value_param)?;
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn try_apply(&mut self,
                __value_param: &dyn bevy::reflect::Reflect)
                -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                if let bevy::reflect::ReflectRef::Enum(__value_param) =
                            bevy::reflect::Reflect::reflect_ref(__value_param) {
                        if bevy::reflect::Enum::variant_name(self) ==
                                    bevy::reflect::Enum::variant_name(__value_param) {
                                match bevy::reflect::Enum::variant_type(__value_param) {
                                    bevy::reflect::VariantType::Struct => {
                                        for field in bevy::reflect::Enum::iter_fields(__value_param)
                                            {
                                            let name = field.name().unwrap();
                                            if let ::core::option::Option::Some(v) =
                                                        bevy::reflect::Enum::field_mut(self, name) {
                                                    bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                }
                                        }
                                    }
                                    bevy::reflect::VariantType::Tuple => {
                                        for (index, field) in
                                            ::core::iter::Iterator::enumerate(bevy::reflect::Enum::iter_fields(__value_param))
                                            {
                                            if let ::core::option::Option::Some(v) =
                                                        bevy::reflect::Enum::field_at_mut(self, index) {
                                                    bevy::reflect::Reflect::try_apply(v, field.value())?;
                                                }
                                        }
                                    }
                                    _ => {}
                                }
                            } else {
                               match bevy::reflect::Enum::variant_name(__value_param) {
                                   "Loading" => { *self = AppState::Loading {} }
                                   "InGame" => { *self = AppState::InGame {} }
                                   name => {
                                       return ::core::result::Result::Err(bevy::reflect::ApplyError::UnknownVariant {
                                                   enum_name: ::core::convert::Into::into(bevy::reflect::DynamicTypePath::reflect_type_path(self)),
                                                   variant_name: ::core::convert::Into::into(name),
                                               });
                                   }
                               }
                           }
                    } else {
                       return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                                   from_kind: bevy::reflect::Reflect::reflect_kind(__value_param),
                                   to_kind: bevy::reflect::ReflectKind::Enum,
                               });
                   }
                ::core::result::Result::Ok(())
            }
            fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                bevy::reflect::ReflectKind::Enum
            }
            fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                bevy::reflect::ReflectRef::Enum(self)
            }
            fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                bevy::reflect::ReflectMut::Enum(self)
            }
            fn reflect_owned(self: ::std::boxed::Box<Self>)
                -> bevy::reflect::ReflectOwned {
                bevy::reflect::ReflectOwned::Enum(self)
            }
            fn reflect_hash(&self) -> ::core::option::Option<u64> {
                bevy::reflect::enum_hash(self)
            }
            fn reflect_partial_eq(&self, value: &dyn bevy::reflect::Reflect)
                -> ::core::option::Option<bool> {
                bevy::reflect::enum_partial_eq(self, value)
            }
        }
        impl bevy::reflect::FromReflect for AppState where
            Self: ::core::any::Any + ::core::marker::Send +
            ::core::marker::Sync {
            fn from_reflect(__param0: &dyn bevy::reflect::Reflect)
                -> ::core::option::Option<Self> {
                if let bevy::reflect::ReflectRef::Enum(__param0) =
                            bevy::reflect::Reflect::reflect_ref(__param0) {
                        match bevy::reflect::Enum::variant_name(__param0) {
                            "Loading" =>
                                ::core::option::Option::Some(AppState::Loading {}),
                            "InGame" =>
                                ::core::option::Option::Some(AppState::InGame {}),
                            name => {
                                ::core::panicking::panic_fmt(format_args!("variant with name `{0}` does not exist on enum `{1}`",
                                        name, <Self as bevy::reflect::TypePath>::type_path()));
                            }
                        }
                    } else { ::core::option::Option::None }
            }
        }
    };
pub fn plugin(app: &mut App) {
    app.register_type::<AppState>();
    ;
    app.init_state::<AppState>().add_plugins((core::plugin, assets::plugin,
            in_game::plugin, board::plugin, player::plugin,
            navigation::plugin, physics::plugin, stats::plugin));
}
