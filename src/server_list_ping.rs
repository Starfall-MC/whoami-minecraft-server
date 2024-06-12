use std::net::SocketAddr;

use async_trait::async_trait;
use valence::{
    network::{CleanupFn, HandshakeData, ServerListPing},
    prelude::*,
    MINECRAFT_VERSION, PROTOCOL_VERSION,
};

pub struct Callbacks;

#[async_trait]
impl NetworkCallbacks for Callbacks {
    async fn server_list_ping(
        &self,
        _shared: &SharedNetworkState,
        remote_addr: SocketAddr,
        handshake_data: &HandshakeData,
    ) -> ServerListPing {
        let mut text =
            Text::text("") + Text::text("Connect to get technical info! ").color(Color::GOLD);
        text += Text::text("You are ");
        text += Text::text(format!("{remote_addr}"))
            .color(Color::DARK_PURPLE)
            .underlined();
        text += Text::text(" and you're talking to the server at ");
        text += Text::text(format!(
            "{}:{}",
            handshake_data.server_address, handshake_data.server_port
        ))
        .color(Color::RED)
        .underlined();

        ServerListPing::Respond {
            online_players: 69,
            max_players: 420,
            player_sample: vec![],
            description: text,
            favicon_png: include_bytes!("../assets/command-block-icon.png"),
            version_name: MINECRAFT_VERSION.to_owned(),
            protocol: PROTOCOL_VERSION,
        }
    }

    async fn login(
        &self,
        _shared: &SharedNetworkState,
        info: &NewClientInfo,
    ) -> Result<CleanupFn, Text> {
        let mut text = Text::text(
            "Hello! You can't play on this server, it exists to show information about you!\n\n",
        );
        text += Text::text("Your username is ")
            + Text::text(info.username.to_string())
                .color(Color::GOLD)
                .underlined();
        text += Text::text("\n\nYour player UUID is ")
            + Text::text(info.uuid.to_string())
                .color(Color::GREEN)
                .underlined();
        text += Text::text("\n\nYou're connecting from ")
            + Text::text(info.ip.to_string()).color(Color::DARK_PURPLE);

        text += Text::text("\n\nYour client sent the following extra info: ");

        for item in info.properties.0.iter() {
            text += Text::text(item.name.clone())
                .color(Color::DARK_GREEN)
                .underlined()
                // + Text::text("=")
                // + Text::text(item.value.clone())
                //     .color(Color::DARK_BLUE)
                //     .underlined()
                + Text::text("; ");
        }

        Err(text)
    }
}
