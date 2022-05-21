pub mod input;
pub mod config;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
      input::{
          KeyBindPressedEvent,
          KeyBindJustPressedEvent,
          KeyBindJustReleasedEvent,
          KeyBindAxisEvent,
          KeyBindPlugin
      },
      config::KeyBindConfig
    };
}