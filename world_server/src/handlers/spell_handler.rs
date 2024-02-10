use crate::prelude::*;
use crate::world::prelude::GameObject;
use chrono::{Datelike, Duration, TimeZone, Utc};
use wow_dbc::Indexable;
use wow_world_messages::wrath::{
    Power, SMSG_SPELL_GO_GameobjectCastFlags, SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate, SMSG_SPELL_START_CastFlags, CMSG_CAST_SPELL,
    SMSG_SPELL_GO, SMSG_SPELL_START,
};

use crate::client_manager::ClientManager;
use crate::packet::ServerMessageExt;

pub async fn handle_cmsg_cast_spell(client_manager: &ClientManager, client_id: u64, packet: &CMSG_CAST_SPELL) -> Result<()> {
    const DEALING_WITH_NETWORK_LATENCY_MS: i32 = 50;
    let client = client_manager.get_authenticated_client(client_id).await?;
    let character_lock = client.get_active_character().await?;
    let character_read = character_lock.read().await;
    let character = character_read.as_character();

    if let Some(spell) = client_manager.data_storage.get_dbc_spells()?.get(packet.spell as i32) {
        info!("Spell: {:?}", spell);

        let cast_flags = SMSG_SPELL_START_CastFlags::new_has_trajectory().set_heal_prediction();

        let spell_start = SMSG_SPELL_START {
            cast_item: character.unwrap().get_guid(),
            caster: character.unwrap().get_guid(),
            cast_count: packet.cast_count,
            spell: packet.spell,
            flags: cast_flags,
            timer: spell.start_recovery_time as u32,
            targets: packet.targets.clone(),
        };

        dbg!(&spell_start);

        spell_start.astd_send_to_client(client.clone()).await?;

        async_std::task::sleep(std::time::Duration::from_millis(
            (spell.start_recovery_time - DEALING_WITH_NETWORK_LATENCY_MS) as u64,
        ))
        .await;

        let power_update = SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate { power: Power::Health };

        let game_object_flags = SMSG_SPELL_GO_GameobjectCastFlags::new_unknown2().set_power_update(power_update);

        let spell_go = SMSG_SPELL_GO {
            cast_item: character.unwrap().get_guid(),
            caster: character.unwrap().get_guid(),
            extra_casts: 0,
            spell: packet.spell,
            flags: game_object_flags,
            timestamp: seconds_elapsed_this_year() as u32,
            hits: vec![character.unwrap().get_guid()],
            misses: vec![],
            targets: packet.targets.clone(),
        };

        spell_go.astd_send_to_client(client).await?;
    }

    Ok(())
}

fn seconds_elapsed_this_year() -> i64 {
    let now = Utc::now();
    let start_of_year = Utc.with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0).single().unwrap();
    let duration: Duration = now.signed_duration_since(start_of_year);
    duration.num_seconds()
}
