//! Significantly abstract [`azalea_protocol`] so it's actually useable for
//! real clients. If you want to make bots, you should use the
//! [`azalea`] crate instead.
//!
//! [`azalea_protocol`]: https://docs.rs/azalea-protocol
//! [`azalea`]: https://docs.rs/azalea

#![feature(provide_any)]
#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(error_generic_member_access)]
#![feature(type_alias_impl_trait)]

mod account;
pub mod chat;
mod client;
pub mod disconnect;
mod entity_query;
mod events;
mod get_mc_dir;
mod local_player;
mod movement;
pub mod packet_handling;
pub mod ping;
mod player;
pub mod task_pool;

pub use account::Account;
pub use client::{
    init_ecs_app, start_ecs, Client, ClientInformation, JoinError, JoinedClientBundle, TabList,
};
pub use events::Event;
pub use local_player::{GameProfileComponent, LocalPlayer};
pub use movement::{SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection};
pub use player::PlayerInfo;
