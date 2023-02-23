//! Disconnect a client from the server.

use azalea_ecs::{
    app::{App, CoreStage, Plugin},
    component::Component,
    entity::Entity,
    event::{EventReader, EventWriter},
    query::Changed,
    schedule::IntoSystemDescriptor,
    system::{Commands, Query},
    AppTickExt,
};
use derive_more::Deref;

use crate::{client::JoinedClientBundle, LocalPlayer};

pub struct DisconnectPlugin;
impl Plugin for DisconnectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DisconnectEvent>()
            .add_system_to_stage(CoreStage::PostUpdate, handle_disconnect)
            .add_tick_system(
                update_read_packets_task_running_component.before(disconnect_on_read_packets_ended),
            )
            .add_tick_system(disconnect_on_read_packets_ended);
    }
}

/// An event sent when a client is getting disconnected.
pub struct DisconnectEvent {
    pub entity: Entity,
}

/// System that removes the [`JoinedClientBundle`] from the entity when it
/// receives a [`DisconnectEvent`].
pub fn handle_disconnect(mut commands: Commands, mut events: EventReader<DisconnectEvent>) {
    for DisconnectEvent { entity } in events.iter() {
        commands.entity(*entity).remove::<JoinedClientBundle>();
    }
}

#[derive(Component, Clone, Copy, Debug, Deref)]
pub struct ReadPacketsTaskRunning(bool);

fn update_read_packets_task_running_component(
    mut commands: Commands,
    local_player: Query<(Entity, &LocalPlayer)>,
) {
    for (entity, local_player) in &local_player {
        let running = !local_player.read_packets_task.is_finished();
        commands
            .entity(entity)
            .insert(ReadPacketsTaskRunning(running));
    }
}
fn disconnect_on_read_packets_ended(
    local_player: Query<(Entity, &ReadPacketsTaskRunning), Changed<ReadPacketsTaskRunning>>,
    mut disconnect_events: EventWriter<DisconnectEvent>,
) {
    for (entity, &read_packets_task_running) in &local_player {
        if !*read_packets_task_running {
            disconnect_events.send(DisconnectEvent { entity });
        }
    }
}
