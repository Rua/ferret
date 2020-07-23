#![allow(unused_variables)]
use crate::{
	assets::AssetStorage,
	component::EntityComponents,
	doom::{
		camera::Camera,
		client::User,
		components::{SpawnOnCeiling, SpawnPoint, Velocity},
		data::FRAME_TIME,
		entitytemplate::{EntityTemplate, EntityTypeId},
		physics::{BoxCollider, SolidMask},
		render::sprite::SpriteRender,
		wad::WadLoader,
	},
	geometry::Angle,
};
use legion::prelude::{ResourceSet, Resources, Write};
use nalgebra::Vector3;
use std::default::Default;

#[rustfmt::skip]
pub fn load(resources: &mut Resources) {
	let (mut asset_storage, mut loader) = <(
		Write<AssetStorage>,
		Write<WadLoader>,
	)>::fetch_mut(resources);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(1)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 1 }),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(2)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 2 }),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(3)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 3 }),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(4)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 4 }),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(11)),
		components: EntityComponents::new(),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: Some("PLAYER"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(Camera {
				base: Vector3::new(0.0, 0.0, 41.0),
				offset: Vector3::zeros(),
				bob_angle: Angle::default(),
				bob_max: 16.0,
				bob_period: 20 * FRAME_TIME,
				deviation_position: 0.0,
				deviation_velocity: 0.0,
				impact_sound: asset_storage.load("DSOOF", &mut *loader),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLAY", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(User {
				error_sound: asset_storage.load("DSNOWAY", &mut *loader),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("PLAYER", template);

	let template = EntityTemplate {
		name: Some("POSSESSED"),
		type_id: Some(EntityTypeId::Thing(3004)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POSS", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("POSSESSED", template);

	let template = EntityTemplate {
		name: Some("SHOTGUY"),
		type_id: Some(EntityTypeId::Thing(9)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SPOS", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SHOTGUY", template);

	let template = EntityTemplate {
		name: Some("VILE"),
		type_id: Some(EntityTypeId::Thing(64)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("VILE", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("VILE", template);

	let template = EntityTemplate {
		name: Some("FIRE"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("FIRE", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("FIRE", template);

	let template = EntityTemplate {
		name: Some("UNDEAD"),
		type_id: Some(EntityTypeId::Thing(66)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SKEL", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("UNDEAD", template);

	let template = EntityTemplate {
		name: Some("TRACER"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("FATB", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("TRACER", template);

	let template = EntityTemplate {
		name: Some("SMOKE"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("PUFF", &mut *loader),
				frame: 1,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SMOKE", template);

	let template = EntityTemplate {
		name: Some("FATSO"),
		type_id: Some(EntityTypeId::Thing(67)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 48.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("FATT", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("FATSO", template);

	let template = EntityTemplate {
		name: Some("FATSHOT"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("MANF", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("FATSHOT", template);

	let template = EntityTemplate {
		name: Some("CHAINGUY"),
		type_id: Some(EntityTypeId::Thing(65)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CPOS", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("CHAINGUY", template);

	let template = EntityTemplate {
		name: Some("TROOP"),
		type_id: Some(EntityTypeId::Thing(3001)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TROO", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("TROOP", template);

	let template = EntityTemplate {
		name: Some("SERGEANT"),
		type_id: Some(EntityTypeId::Thing(3002)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 30.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SARG", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SERGEANT", template);

	let template = EntityTemplate {
		name: Some("SHADOWS"),
		type_id: Some(EntityTypeId::Thing(58)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 30.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SARG", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SHADOWS", template);

	let template = EntityTemplate {
		name: Some("HEAD"),
		type_id: Some(EntityTypeId::Thing(3005)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 31.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HEAD", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("HEAD", template);

	let template = EntityTemplate {
		name: Some("BRUISER"),
		type_id: Some(EntityTypeId::Thing(3003)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 24.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BOSS", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BRUISER", template);

	let template = EntityTemplate {
		name: Some("BRUISERSHOT"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BAL7", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BRUISERSHOT", template);

	let template = EntityTemplate {
		name: Some("KNIGHT"),
		type_id: Some(EntityTypeId::Thing(69)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 24.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BOS2", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("KNIGHT", template);

	let template = EntityTemplate {
		name: Some("SKULL"),
		type_id: Some(EntityTypeId::Thing(3006)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SKUL", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SKULL", template);

	let template = EntityTemplate {
		name: Some("SPIDER"),
		type_id: Some(EntityTypeId::Thing(7)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 100.0,
				radius: 128.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SPID", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SPIDER", template);

	let template = EntityTemplate {
		name: Some("BABY"),
		type_id: Some(EntityTypeId::Thing(68)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 64.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BSPI", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BABY", template);

	let template = EntityTemplate {
		name: Some("CYBORG"),
		type_id: Some(EntityTypeId::Thing(16)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 110.0,
				radius: 40.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CYBR", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("CYBORG", template);

	let template = EntityTemplate {
		name: Some("PAIN"),
		type_id: Some(EntityTypeId::Thing(71)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 31.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PAIN", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("PAIN", template);

	let template = EntityTemplate {
		name: Some("WOLFSS"),
		type_id: Some(EntityTypeId::Thing(84)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SSWV", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("WOLFSS", template);

	let template = EntityTemplate {
		name: Some("KEEN"),
		type_id: Some(EntityTypeId::Thing(72)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 72.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 72.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("KEEN", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("KEEN", template);

	let template = EntityTemplate {
		name: Some("BOSSBRAIN"),
		type_id: Some(EntityTypeId::Thing(88)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BBRN", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BOSSBRAIN", template);

	let template = EntityTemplate {
		name: Some("BOSSSPIT"),
		type_id: Some(EntityTypeId::Thing(89)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("SSWV", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BOSSSPIT", template);

	let template = EntityTemplate {
		name: Some("BOSSTARGET"),
		type_id: Some(EntityTypeId::Thing(87)),
		components: EntityComponents::new(),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BOSSTARGET", template);

	let template = EntityTemplate {
		name: Some("SPAWNSHOT"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BOSF", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SPAWNSHOT", template);

	let template = EntityTemplate {
		name: Some("SPAWNFIRE"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("FIRE", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SPAWNFIRE", template);

	let template = EntityTemplate {
		name: Some("BARREL"),
		type_id: Some(EntityTypeId::Thing(2035)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 42.0,
				radius: 10.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BAR1", &mut *loader),
				frame: 0,
				full_bright: false,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BARREL", template);

	let template = EntityTemplate {
		name: Some("TROOPSHOT"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BAL1", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("TROOPSHOT", template);

	let template = EntityTemplate {
		name: Some("HEADSHOT"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BAL2", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("HEADSHOT", template);

	let template = EntityTemplate {
		name: Some("ROCKET"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("MISL", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("ROCKET", template);

	let template = EntityTemplate {
		name: Some("PLASMA"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLSS", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("PLASMA", template);

	let template = EntityTemplate {
		name: Some("BFG"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BFS1", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BFG", template);

	let template = EntityTemplate {
		name: Some("ARACHPLAZ"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("APLS", &mut *loader),
				frame: 0,
				full_bright: true,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("ARACHPLAZ", template);

	let template = EntityTemplate {
		name: Some("PUFF"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("PUFF", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("PUFF", template);

	let template = EntityTemplate {
		name: Some("BLOOD"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BLUD", &mut *loader),
				frame: 2,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BLOOD", template);

	let template = EntityTemplate {
		name: Some("TFOG"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("TFOG", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("TFOG", template);

	let template = EntityTemplate {
		name: Some("IFOG"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("IFOG", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("IFOG", template);

	let template = EntityTemplate {
		name: Some("TELEPORTMAN"),
		type_id: Some(EntityTypeId::Thing(14)),
		components: EntityComponents::new(),
	};
	asset_storage.insert_with_name::<EntityTemplate>("TELEPORTMAN", template);

	let template = EntityTemplate {
		name: Some("EXTRABFG"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BFE2", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("EXTRABFG", template);

	let template = EntityTemplate {
		name: Some("MISC0"),
		type_id: Some(EntityTypeId::Thing(2018)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ARM1", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC0", template);

	let template = EntityTemplate {
		name: Some("MISC1"),
		type_id: Some(EntityTypeId::Thing(2019)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ARM2", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC1", template);

	let template = EntityTemplate {
		name: Some("MISC2"),
		type_id: Some(EntityTypeId::Thing(2014)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BON1", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC2", template);

	let template = EntityTemplate {
		name: Some("MISC3"),
		type_id: Some(EntityTypeId::Thing(2015)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BON2", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC3", template);

	let template = EntityTemplate {
		name: Some("MISC4"),
		type_id: Some(EntityTypeId::Thing(5)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BKEY", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC4", template);

	let template = EntityTemplate {
		name: Some("MISC5"),
		type_id: Some(EntityTypeId::Thing(13)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("RKEY", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC5", template);

	let template = EntityTemplate {
		name: Some("MISC6"),
		type_id: Some(EntityTypeId::Thing(6)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("YKEY", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC6", template);

	let template = EntityTemplate {
		name: Some("MISC7"),
		type_id: Some(EntityTypeId::Thing(39)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("YSKU", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC7", template);

	let template = EntityTemplate {
		name: Some("MISC8"),
		type_id: Some(EntityTypeId::Thing(38)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("RSKU", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC8", template);

	let template = EntityTemplate {
		name: Some("MISC9"),
		type_id: Some(EntityTypeId::Thing(40)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BSKU", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC9", template);

	let template = EntityTemplate {
		name: Some("MISC10"),
		type_id: Some(EntityTypeId::Thing(2011)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("STIM", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC10", template);

	let template = EntityTemplate {
		name: Some("MISC11"),
		type_id: Some(EntityTypeId::Thing(2012)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("MEDI", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC11", template);

	let template = EntityTemplate {
		name: Some("MISC12"),
		type_id: Some(EntityTypeId::Thing(2013)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SOUL", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC12", template);

	let template = EntityTemplate {
		name: Some("INV"),
		type_id: Some(EntityTypeId::Thing(2022)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PINV", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("INV", template);

	let template = EntityTemplate {
		name: Some("MISC13"),
		type_id: Some(EntityTypeId::Thing(2023)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PSTR", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC13", template);

	let template = EntityTemplate {
		name: Some("INS"),
		type_id: Some(EntityTypeId::Thing(2024)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PINS", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("INS", template);

	let template = EntityTemplate {
		name: Some("MISC14"),
		type_id: Some(EntityTypeId::Thing(2025)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SUIT", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC14", template);

	let template = EntityTemplate {
		name: Some("MISC15"),
		type_id: Some(EntityTypeId::Thing(2026)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PMAP", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC15", template);

	let template = EntityTemplate {
		name: Some("MISC16"),
		type_id: Some(EntityTypeId::Thing(2045)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PVIS", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC16", template);

	let template = EntityTemplate {
		name: Some("MEGA"),
		type_id: Some(EntityTypeId::Thing(83)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("MEGA", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MEGA", template);

	let template = EntityTemplate {
		name: Some("CLIP"),
		type_id: Some(EntityTypeId::Thing(2007)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CLIP", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("CLIP", template);

	let template = EntityTemplate {
		name: Some("MISC17"),
		type_id: Some(EntityTypeId::Thing(2048)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("AMMO", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC17", template);

	let template = EntityTemplate {
		name: Some("MISC18"),
		type_id: Some(EntityTypeId::Thing(2010)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ROCK", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC18", template);

	let template = EntityTemplate {
		name: Some("MISC19"),
		type_id: Some(EntityTypeId::Thing(2046)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BROK", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC19", template);

	let template = EntityTemplate {
		name: Some("MISC20"),
		type_id: Some(EntityTypeId::Thing(2047)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CELL", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC20", template);

	let template = EntityTemplate {
		name: Some("MISC21"),
		type_id: Some(EntityTypeId::Thing(17)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CELP", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC21", template);

	let template = EntityTemplate {
		name: Some("MISC22"),
		type_id: Some(EntityTypeId::Thing(2008)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SHEL", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC22", template);

	let template = EntityTemplate {
		name: Some("MISC23"),
		type_id: Some(EntityTypeId::Thing(2049)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SBOX", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC23", template);

	let template = EntityTemplate {
		name: Some("MISC24"),
		type_id: Some(EntityTypeId::Thing(8)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BPAK", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC24", template);

	let template = EntityTemplate {
		name: Some("MISC25"),
		type_id: Some(EntityTypeId::Thing(2006)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BFUG", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC25", template);

	let template = EntityTemplate {
		name: Some("CHAINGUN"),
		type_id: Some(EntityTypeId::Thing(2002)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("MGUN", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("CHAINGUN", template);

	let template = EntityTemplate {
		name: Some("MISC26"),
		type_id: Some(EntityTypeId::Thing(2005)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CSAW", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC26", template);

	let template = EntityTemplate {
		name: Some("MISC27"),
		type_id: Some(EntityTypeId::Thing(2003)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("LAUN", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC27", template);

	let template = EntityTemplate {
		name: Some("MISC28"),
		type_id: Some(EntityTypeId::Thing(2004)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLAS", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC28", template);

	let template = EntityTemplate {
		name: Some("SHOTGUN"),
		type_id: Some(EntityTypeId::Thing(2001)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SHOT", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SHOTGUN", template);

	let template = EntityTemplate {
		name: Some("SUPERSHOTGUN"),
		type_id: Some(EntityTypeId::Thing(82)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SGN2", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SUPERSHOTGUN", template);

	let template = EntityTemplate {
		name: Some("MISC29"),
		type_id: Some(EntityTypeId::Thing(85)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TLMP", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC29", template);

	let template = EntityTemplate {
		name: Some("MISC30"),
		type_id: Some(EntityTypeId::Thing(86)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TLP2", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC30", template);

	let template = EntityTemplate {
		name: Some("MISC31"),
		type_id: Some(EntityTypeId::Thing(2028)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COLU", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC31", template);

	let template = EntityTemplate {
		name: Some("MISC32"),
		type_id: Some(EntityTypeId::Thing(30)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL1", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC32", template);

	let template = EntityTemplate {
		name: Some("MISC33"),
		type_id: Some(EntityTypeId::Thing(31)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL2", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC33", template);

	let template = EntityTemplate {
		name: Some("MISC34"),
		type_id: Some(EntityTypeId::Thing(32)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL3", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC34", template);

	let template = EntityTemplate {
		name: Some("MISC35"),
		type_id: Some(EntityTypeId::Thing(33)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL4", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC35", template);

	let template = EntityTemplate {
		name: Some("MISC36"),
		type_id: Some(EntityTypeId::Thing(37)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL6", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC36", template);

	let template = EntityTemplate {
		name: Some("MISC37"),
		type_id: Some(EntityTypeId::Thing(36)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL5", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC37", template);

	let template = EntityTemplate {
		name: Some("MISC38"),
		type_id: Some(EntityTypeId::Thing(41)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CEYE", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC38", template);

	let template = EntityTemplate {
		name: Some("MISC39"),
		type_id: Some(EntityTypeId::Thing(42)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("FSKU", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC39", template);

	let template = EntityTemplate {
		name: Some("MISC40"),
		type_id: Some(EntityTypeId::Thing(43)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TRE1", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC40", template);

	let template = EntityTemplate {
		name: Some("MISC41"),
		type_id: Some(EntityTypeId::Thing(44)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TBLU", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC41", template);

	let template = EntityTemplate {
		name: Some("MISC42"),
		type_id: Some(EntityTypeId::Thing(45)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TGRN", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC42", template);

	let template = EntityTemplate {
		name: Some("MISC43"),
		type_id: Some(EntityTypeId::Thing(46)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TRED", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC43", template);

	let template = EntityTemplate {
		name: Some("MISC44"),
		type_id: Some(EntityTypeId::Thing(55)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SMBT", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC44", template);

	let template = EntityTemplate {
		name: Some("MISC45"),
		type_id: Some(EntityTypeId::Thing(56)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SMGT", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC45", template);

	let template = EntityTemplate {
		name: Some("MISC46"),
		type_id: Some(EntityTypeId::Thing(57)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SMRT", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC46", template);

	let template = EntityTemplate {
		name: Some("MISC47"),
		type_id: Some(EntityTypeId::Thing(47)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SMIT", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC47", template);

	let template = EntityTemplate {
		name: Some("MISC48"),
		type_id: Some(EntityTypeId::Thing(48)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ELEC", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC48", template);

	let template = EntityTemplate {
		name: Some("MISC49"),
		type_id: Some(EntityTypeId::Thing(34)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CAND", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC49", template);

	let template = EntityTemplate {
		name: Some("MISC50"),
		type_id: Some(EntityTypeId::Thing(35)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CBRA", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC50", template);

	let template = EntityTemplate {
		name: Some("MISC51"),
		type_id: Some(EntityTypeId::Thing(49)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 68.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 68.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR1", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC51", template);

	let template = EntityTemplate {
		name: Some("MISC52"),
		type_id: Some(EntityTypeId::Thing(50)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 84.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 84.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR2", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC52", template);

	let template = EntityTemplate {
		name: Some("MISC53"),
		type_id: Some(EntityTypeId::Thing(51)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 84.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 84.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR3", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC53", template);

	let template = EntityTemplate {
		name: Some("MISC54"),
		type_id: Some(EntityTypeId::Thing(52)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 68.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 68.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR4", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC54", template);

	let template = EntityTemplate {
		name: Some("MISC55"),
		type_id: Some(EntityTypeId::Thing(53)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 52.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 52.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR5", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC55", template);

	let template = EntityTemplate {
		name: Some("MISC56"),
		type_id: Some(EntityTypeId::Thing(59)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 84.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpawnOnCeiling {
				offset: 84.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR2", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC56", template);

	let template = EntityTemplate {
		name: Some("MISC57"),
		type_id: Some(EntityTypeId::Thing(60)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 68.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpawnOnCeiling {
				offset: 68.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR4", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC57", template);

	let template = EntityTemplate {
		name: Some("MISC58"),
		type_id: Some(EntityTypeId::Thing(61)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 52.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpawnOnCeiling {
				offset: 52.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR3", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC58", template);

	let template = EntityTemplate {
		name: Some("MISC59"),
		type_id: Some(EntityTypeId::Thing(62)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 52.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpawnOnCeiling {
				offset: 52.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR5", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC59", template);

	let template = EntityTemplate {
		name: Some("MISC60"),
		type_id: Some(EntityTypeId::Thing(63)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 68.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpawnOnCeiling {
				offset: 68.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR1", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC60", template);

	let template = EntityTemplate {
		name: Some("MISC61"),
		type_id: Some(EntityTypeId::Thing(22)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HEAD", &mut *loader),
				frame: 11,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC61", template);

	let template = EntityTemplate {
		name: Some("MISC62"),
		type_id: Some(EntityTypeId::Thing(15)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLAY", &mut *loader),
				frame: 13,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC62", template);

	let template = EntityTemplate {
		name: Some("MISC63"),
		type_id: Some(EntityTypeId::Thing(18)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POSS", &mut *loader),
				frame: 11,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC63", template);

	let template = EntityTemplate {
		name: Some("MISC64"),
		type_id: Some(EntityTypeId::Thing(21)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SARG", &mut *loader),
				frame: 13,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC64", template);

	let template = EntityTemplate {
		name: Some("MISC65"),
		type_id: Some(EntityTypeId::Thing(23)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SKUL", &mut *loader),
				frame: 10,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC65", template);

	let template = EntityTemplate {
		name: Some("MISC66"),
		type_id: Some(EntityTypeId::Thing(20)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TROO", &mut *loader),
				frame: 12,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC66", template);

	let template = EntityTemplate {
		name: Some("MISC67"),
		type_id: Some(EntityTypeId::Thing(19)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SPOS", &mut *loader),
				frame: 11,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC67", template);

	let template = EntityTemplate {
		name: Some("MISC68"),
		type_id: Some(EntityTypeId::Thing(10)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLAY", &mut *loader),
				frame: 22,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC68", template);

	let template = EntityTemplate {
		name: Some("MISC69"),
		type_id: Some(EntityTypeId::Thing(12)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLAY", &mut *loader),
				frame: 22,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC69", template);

	let template = EntityTemplate {
		name: Some("MISC70"),
		type_id: Some(EntityTypeId::Thing(28)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL2", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC70", template);

	let template = EntityTemplate {
		name: Some("MISC71"),
		type_id: Some(EntityTypeId::Thing(24)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL5", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC71", template);

	let template = EntityTemplate {
		name: Some("MISC72"),
		type_id: Some(EntityTypeId::Thing(27)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL4", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC72", template);

	let template = EntityTemplate {
		name: Some("MISC73"),
		type_id: Some(EntityTypeId::Thing(29)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL3", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC73", template);

	let template = EntityTemplate {
		name: Some("MISC74"),
		type_id: Some(EntityTypeId::Thing(25)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL1", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC74", template);

	let template = EntityTemplate {
		name: Some("MISC75"),
		type_id: Some(EntityTypeId::Thing(26)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL6", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC75", template);

	let template = EntityTemplate {
		name: Some("MISC76"),
		type_id: Some(EntityTypeId::Thing(54)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 32.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TRE2", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC76", template);

	let template = EntityTemplate {
		name: Some("MISC77"),
		type_id: Some(EntityTypeId::Thing(70)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("FCAN", &mut *loader),
				frame: 0,
				full_bright: true,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC77", template);

	let template = EntityTemplate {
		name: Some("MISC78"),
		type_id: Some(EntityTypeId::Thing(73)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 88.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 88.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB1", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC78", template);

	let template = EntityTemplate {
		name: Some("MISC79"),
		type_id: Some(EntityTypeId::Thing(74)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 88.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 88.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB2", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC79", template);

	let template = EntityTemplate {
		name: Some("MISC80"),
		type_id: Some(EntityTypeId::Thing(75)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 64.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB3", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC80", template);

	let template = EntityTemplate {
		name: Some("MISC81"),
		type_id: Some(EntityTypeId::Thing(76)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 64.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB4", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC81", template);

	let template = EntityTemplate {
		name: Some("MISC82"),
		type_id: Some(EntityTypeId::Thing(77)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 64.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB5", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC82", template);

	let template = EntityTemplate {
		name: Some("MISC83"),
		type_id: Some(EntityTypeId::Thing(78)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 64.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB6", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC83", template);

	let template = EntityTemplate {
		name: Some("MISC84"),
		type_id: Some(EntityTypeId::Thing(79)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("POB1", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC84", template);

	let template = EntityTemplate {
		name: Some("MISC85"),
		type_id: Some(EntityTypeId::Thing(80)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("POB2", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC85", template);

	let template = EntityTemplate {
		name: Some("MISC86"),
		type_id: Some(EntityTypeId::Thing(81)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BRS1", &mut *loader),
				frame: 0,
				full_bright: false,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC86", template);
}
