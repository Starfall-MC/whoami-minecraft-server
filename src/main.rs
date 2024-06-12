#![allow(clippy::type_complexity)]

mod server_list_ping;

use std::net::SocketAddr;

use valence::prelude::*;

const SPAWN_Y: i32 = 64;

fn main() {
    let mut app = App::new();
    app.world.insert_resource(NetworkSettings {
        callbacks: ErasedNetworkCallbacks::new(server_list_ping::Callbacks),
        max_connections: 10,
        max_players: 69420,
        address: "0.0.0.0:25565".parse::<SocketAddr>().unwrap(),
        ..Default::default()
    });

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // .add_systems(Update, ())
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let mut layer: LayerBundle = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -25..25 {
        for x in -25..25 {
            layer
                .chunk
                .set_block([x, SPAWN_Y, z], BlockState::LIGHT_GRAY_WOOL);
        }
    }

    commands.spawn(layer);
}
