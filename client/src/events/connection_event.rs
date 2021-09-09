use shared::clientinfo::ClientInfo;

#[derive(Clone, Debug)]
pub enum ConnectionEvent {
    // (position, player_entities, player_info)
    EnterLobby(ClientInfo),
    // (position, player_info)
    ExitGame(ClientInfo),
}
