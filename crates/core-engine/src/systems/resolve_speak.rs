use crate::{
    components::{intend_speak::IntendSpeak, speaks::Speaks, User},
    resources::MessageSenderSingleClient,
};
use bevy::prelude::*;
use core_api::ServerMessageSingleClient;

pub fn resolve_speak_system(
    speaker_query: Query<(Entity, &IntendSpeak, &User)>,
    target_query: Query<(&Name, &Speaks)>,
    sender_single_client: Res<MessageSenderSingleClient>,
    mut commands: Commands,
) {
    for (ent, intend_speak, user) in speaker_query.iter() {
        if let Ok((name, speaks)) = target_query.get(intend_speak.target) {
            sender_single_client
                .0
                .send((
                    user.0,
                    ServerMessageSingleClient::ShowDialogue {
                        entity_name: name.to_string(),
                        dialogue_map: speaks.0.clone(),
                    },
                ))
                .ok();
        }
        commands.entity(ent).remove::<IntendSpeak>();
    }
}
