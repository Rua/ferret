use crate::{
	common::spawn::SpawnMergerHandlerSet,
	doom::{
		assets::template::{EntityTemplateRef, EntityTemplateRefDef},
		draw::{sprite::SpriteRender, wsprite::WeaponSpriteRender},
		game::{
			map::{LinedefRef, LinedefRefDef, MapDynamic, SectorRef, SectorRefDef, SpawnPoint},
			RandomTransformDef, Transform, TransformDef,
		},
		ui::{
			hud::{AmmoStat, ArmsStat, HealthStat},
			UiImage, UiText, UiTransform,
		},
	},
};
use legion::{systems::ResourceSet, Registry, Resources, Write};

pub fn register_components(resources: &mut Resources) {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	handler_set.register_clone::<AmmoStat>();

	handler_set.register_clone::<ArmsStat>();

	registry.register::<EntityTemplateRef>("EntityTemplateRef".into());
	handler_set.register_spawn::<EntityTemplateRefDef, EntityTemplateRef>();

	handler_set.register_clone::<HealthStat>();

	registry.register::<LinedefRef>("LinedefRef".into());
	handler_set.register_clone::<LinedefRef>();
	handler_set.register_spawn::<LinedefRefDef, LinedefRef>();

	registry.register::<MapDynamic>("MapDynamic".into());
	handler_set.register_clone::<MapDynamic>();

	registry.register::<SectorRef>("SectorRef".into());
	handler_set.register_clone::<SectorRef>();
	handler_set.register_spawn::<SectorRefDef, SectorRef>();

	registry.register::<SpawnPoint>("SpawnPoint".into());
	handler_set.register_clone::<SpawnPoint>();

	registry.register::<SpriteRender>("SpriteRender".into());
	handler_set.register_clone::<SpriteRender>();

	registry.register::<Transform>("Transform".into());
	handler_set.register_spawn::<TransformDef, Transform>();
	handler_set.register_spawn::<RandomTransformDef, Transform>();

	handler_set.register_clone::<UiImage>();

	handler_set.register_clone::<UiText>();

	handler_set.register_clone::<UiTransform>();

	registry.register::<WeaponSpriteRender>("WeaponSpriteRender".into());
	handler_set.register_clone::<WeaponSpriteRender>();
}
