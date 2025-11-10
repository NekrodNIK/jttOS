mod controller;
mod key;
mod keyboard;
mod parser;

pub use keyboard::Keyboard;

use crate::{ps2::key::KeyEvent, ringbuffer::RingBuffer, sync::IntSafe};

pub static KEY_EVENTS: IntSafe<RingBuffer<KeyEvent, 1024>> = IntSafe::new(RingBuffer::new());
