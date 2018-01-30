extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate mqtt3;
extern crate bytes;
extern crate chrono;
#[macro_use]
extern crate failure;
#[cfg(feature = "jwt")]
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate crossbeam_channel;
extern crate openssl;
extern crate tokio_openssl;

mod codec;
mod packet;
mod mqttopts;
mod client;
mod error;

// expose to other crates
pub use mqttopts::{ConnectionMethod, MqttOptions, ReconnectOptions, SecurityOptions};
pub use client::MqttClient;
pub use mqtt3::{QoS, Packet, LastWill};
pub use client::Notification;