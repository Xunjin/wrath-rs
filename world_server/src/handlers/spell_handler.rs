use crate::prelude::*;
use crate::{client_manager::ClientManager, world::World};
use wow_world_messages::wrath::CMSG_CAST_SPELL;
use wow_spells;

//HIGH-LEVEL: This is a handler for CMSG_CAST_SPELL from https://github.com/azerothcore/azerothcore-wotlk/blob/a5cd23fa5b4b1db11a3848d5ac4a87e3c53b738e/src/server/game/Handlers/SpellHandler.cpp#L341-L478
pub async fn handle_cmsg_cast_spell(client_manager: &ClientManager, client_id: u64, world: &World, packet: &CMSG_CAST_SPELL) -> Result<()> {
    let client = client_manager.get_authenticated_client(client_id).await?;
    let character_lock = client.get_active_character().await?;
    let character = character_lock.read().await;

    //HIGH-LEVEL: Do we have Unit data, like this one? https://github.com/azerothcore/azerothcore-wotlk/blob/a5cd23fa5b4b1db11a3848d5ac4a87e3c53b738e/src/server/game/Entities/Unit/Unit.cpp#L205-L229

    //HIGH-LEVEL: Get spell info from spell_id
    if let Some(spell) = wow_spells::wrath::lookup_spell(packet.spell) {
        println!("Spell is named '{}'.", spell.spell_name());
    } else {
        println!("Spell not found.");
    }

    Ok(())
}

// let spell_id = packet.spell_id;
// let spell = wow_spells::wrath::lookup_spell(spell_id);
// if let Some(spell) = spell {
//     let spell_cast = wow_world_messages::wrath::spell_to_spell_cast(&character, spell);
//     let mut world = world.write().await;
//     world.add_spell_cast(spell_cast);
// } else {
//     warn!("Failed to find spell with id {}", spell_id);
// }

fn get_spell_cast_targets() {
    unimplemented!();
}

fn handle_client_cast_flags() {
    unimplemented!();
}