#![allow(unused)]
use light::Light;
use login_play::LoginPlay;
use play::chunk_data::ChunkData;
use set_border_size::SetBorderSize;
use set_center_chunk::SetCenterChunk;
use std::collections::VecDeque;
use synchronize_player_position::SyncPlayerPos;

use std::thread;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

mod handshake;
use handshake::HandshakeData;

mod login;
use login::login_start::LoginStart;
use login::login_success::LoginSuccess;

mod nbt;
use nbt::NBT::{self};

mod configuration;
use configuration::{
    biome_registry::get_biome_registry, damage_registry::get_damage_registry,
    dimension_type::get_dimension_registry, painting_registry::get_painting_registry, registry::*,
    update_tags::*, wolf_registry::get_wolf_registry,
};
mod varint;
use varint::VarInt;

mod play;
use play::*;

#[derive(Debug, Clone, PartialEq)]
enum State {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

#[tokio::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:25565").await.unwrap();

    loop {
        let (socket, addr) = server.accept().await.unwrap();
        println!("New connection: {}", addr);

        // Handle the connection in a separate task
        tokio::spawn(async move {
            read_packets(socket).await;
        });
    }
}

async fn read_packets(stream: TcpStream) {
    let mut state = State::Handshaking;
    let mut times_in_play = 0;
    let (mut reader, mut writer) = split(stream);
    let (tx, mut rx) = mpsc::channel(32);

    let read_handle = tokio::spawn(async move {
        let mut buffer = vec![0; 1024];
        loop {
            let mut value = reader.read(&mut buffer).await;
            match value {
                Ok(0) => {
                    println!("Client disconnected");
                    break;
                }
                Ok(n) => {
                    println!("Received packet of size: {n}");
                    println!("Received packet: {:?}", &buffer[..n]);

                    // Send the received packet to the writing thread
                    if tx.send(buffer[..n].to_vec()).await.is_ok() {
                        println!("packet sent!");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from stream: {}", e);
                    break;
                }
            }
        }
    });

    let write_handle = tokio::spawn(async move {
        while let Some(packet) = rx.recv().await {
            let mut packets: VecDeque<&[u8]> = VecDeque::new();
            let mut index = 0;
            while index < packet.len() {
                let (packet_length, bytes_read) = VarInt::read_varint(&packet[index..]).unwrap();
                packets.push_back(&packet[index..index + packet_length as usize + bytes_read]);
                index += packet_length as usize + 1;
            }

            let initial_packet_length = packets.len();
            index = 0;
            while index < initial_packet_length {
                match state {
                    State::Handshaking => {
                        println!("In Handshake State");

                        let handshake_packet = packets[index];
                        let mut handshake = HandshakeData::default();
                        handshake.from_buffer(&handshake_packet[1..]);
                        match handshake.next_state {
                            1 => state = State::Status,
                            2 => state = State::Login,
                            e => eprintln!("Unknown state {e}"),
                        }
                        println!("{:#?}", handshake);
                    }
                    State::Status => {
                        println!("In Status State");
                    }
                    State::Login => {
                        println!("In Login State");

                        let mut login_start = LoginStart::default();
                        let login_start_packet = packets[index];

                        login_start.from_buffer(&login_start_packet);
                        println!("{:#?}", login_start);

                        let login_success = LoginSuccess::new(
                            login_start.uuid,
                            login_start.original_uuid_buffer,
                            login_start.username.clone(),
                        );
                        let mut success_buff = login_success.to_buffer();
                        let mut append = vec![0x02];
                        append.append(&mut success_buff);
                        append.insert(0, append.len() as u8);

                        // eprintln!("Sending login success...");
                        writer.write_all(&append).await.unwrap();
                        // println!("Sent login success!");
                        state = State::Configuration;
                    }
                    State::Configuration => {
                        println!("In Configuration State");

                        let mut vec = vec![0x01];
                        let channel = "minecraft:brand".as_bytes();
                        vec.push(channel.len() as u8);
                        vec.extend_from_slice(channel);
                        let vanilla = "vanilla".as_bytes();
                        vec.push(vanilla.len() as u8);
                        vec.extend_from_slice(&vanilla);
                        vec.insert(0, vec.len() as u8);
                        //send out clientbound plugin
                        writer.write_all(&vec).await.unwrap();

                        let known_packs = "minecraft".as_bytes();
                        let id = "core".as_bytes();
                        let version = "1.21.3".as_bytes();
                        let mut known_packs_vec = vec![0x0E];
                        known_packs_vec.push(1);
                        known_packs_vec.push(known_packs.len() as u8);
                        known_packs_vec.extend_from_slice(known_packs);
                        known_packs_vec.push(id.len() as u8);
                        known_packs_vec.extend_from_slice(id);
                        known_packs_vec.push(version.len() as u8);
                        known_packs_vec.extend_from_slice(version);
                        known_packs_vec.insert(0, known_packs_vec.len() as u8);
                        writer.write_all(&known_packs_vec).await.unwrap();

                        let biome_reg = get_biome_registry();
                        let biome_registry = RegistryDataPacket::new(
                            "minecraft:worldgen/biome",
                            biome_reg.len() as u8,
                            biome_reg,
                        );

                        writer.write_all(&biome_registry.to_bytes()).await.unwrap();

                        let biome_tags = TagArray {
                            registry: "minecraft:worldgen/biome",
                            tag_array: vec![
                                Tag {
                                    tag_name: "minecraft:is_badlands",
                                    count: VarInt(0),
                                    entries: vec![],
                                },
                                Tag {
                                    tag_name: "minecraft:is_jungle",
                                    count: VarInt(0),
                                    entries: vec![],
                                },
                                Tag {
                                    tag_name: "minecraft:is_savanna",
                                    count: VarInt(0),
                                    entries: vec![],
                                },
                            ],
                        };

                        let update_biome_tag = UpdateTags {
                            length: VarInt(1),
                            array_of_tags: vec![biome_tags],
                        };

                        writer
                            .write_all(&update_biome_tag.to_bytes())
                            .await
                            .unwrap();

                        // write all wolf_registries
                        let wolf_reg = get_wolf_registry();
                        let wolf_registry = RegistryDataPacket::new(
                            "minecraft:wolf_variant",
                            wolf_reg.len() as u8,
                            wolf_reg,
                        );
                        writer.write_all(&wolf_registry.to_bytes()).await.unwrap();

                        // write all painting_registries
                        let painting_reg = get_painting_registry();
                        let painting_registry = RegistryDataPacket::new(
                            "minecraft:painting_variant",
                            painting_reg.len() as u8,
                            painting_reg,
                        );

                        writer
                            .write_all(&painting_registry.to_bytes())
                            .await
                            .unwrap();

                        let dimension_type = get_dimension_registry();
                        let dimension_reg = RegistryDataPacket::new(
                            "minecraft:dimension_type",
                            dimension_type.len() as u8,
                            dimension_type,
                        );
                        writer.write_all(&dimension_reg.to_bytes()).await.unwrap();

                        let damage_type = get_damage_registry();
                        let damage_reg = RegistryDataPacket::new(
                            "minecraft:damage_type",
                            damage_type.len() as u8,
                            damage_type,
                        );

                        writer.write_all(&damage_reg.to_bytes()).await.unwrap();

                        // write finished config
                        writer.write_all(&[1, 3]).await.unwrap();
                        state = State::Play;
                    }
                    State::Play => {
                        if times_in_play > 0 {
                            break;
                        }
                        times_in_play += 1;
                        println!("In Play State");
                        let login_play = LoginPlay::new();
                        writer.write_all(&login_play.to_bytes()).await.unwrap();

                        let mut game_event = vec![0x23, 13];
                        game_event.extend_from_slice(&0.0f32.to_be_bytes());
                        game_event.insert(0, game_event.len() as u8);
                        writer.write_all(&game_event).await.unwrap();

                        let chunk_data = ChunkData::new();
                        writer.write_all(&chunk_data.to_bytes()).await.unwrap();

                        let sync_player_pos = SyncPlayerPos::default();
                        writer.write_all(&sync_player_pos.to_bytes()).await.unwrap();

                        let center_chunk = SetCenterChunk::default();
                        writer.write_all(&center_chunk.to_bytes()).await.unwrap();

                        let border_size = SetBorderSize::new(100.0);
                        writer.write_all(&border_size.to_bytes()).await.unwrap();

                        // let mut keep_alive = vec![0x27];
                        // keep_alive.extend_from_slice(&15u64.to_be_bytes());
                        // keep_alive.insert(0, keep_alive.len() as u8);
                        // println!("{:?}", keep_alive);
                        // writer.write_all(&keep_alive).await.unwrap();
                    }
                }

                index += 1;
            }
        }
    });

    // Wait for both threads to complete
    let _ = tokio::join!(read_handle, write_handle);
}
