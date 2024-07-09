use std::fmt::Formatter;

use azalea_client::{
    inventory::{CloseContainerEvent, ContainerClickEvent, InventoryComponent},
    packet_handling::game::PacketEvent
};
use azalea_core::position::BlockPos;
use azalea_inventory::{operations::ClickOperation, ItemSlot, Menu};
use azalea_protocol::packets::game::ClientboundGamePacket;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{component::Component, prelude::EventReader, system::Commands};
use futures_lite::Future;
use std::fmt::Debug;

use crate::bot::BotClientExt;