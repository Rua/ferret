use crate::{
	common::{assets::AssetStorage, video::RenderContext},
	doom::{client::Client, image::process_images, state::weapon::WeaponState, wad::IWADInfo},
};
use legion::{systems::ResourceSet, IntoQuery, Read, Resources, World, Write};

pub fn give_all(world: &mut World, resources: &mut Resources, add_keys: bool) {
	let (client, iwadinfo, render_context, mut asset_storage) = <(
		Read<Client>,
		Read<IWADInfo>,
		Read<RenderContext>,
		Write<AssetStorage>,
	)>::fetch_mut(resources);
	let mut query = <&mut WeaponState>::query();

	if let Some(weapon_state) = client
		.entity
		.and_then(|entity| query.get_mut(world, entity).ok())
	{
		weapon_state
			.inventory
			.extend(iwadinfo.weapons.iter().map(|name| asset_storage.load(name)));
		process_images(&render_context, &mut asset_storage);

		if add_keys {
			log::info!("Very Happy Ammo Added!");
		} else {
			log::info!("Ammo (no keys) Added");
		}
	}
}
