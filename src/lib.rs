pub mod input;
pub mod config;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
      input::{
          KeyBindPressedEvent,
          KeyBindJustPressedEvent,
          KeyBindJustReleasedEvent,
          KeyBindPlugin
      },
      config::KeyBindConfig
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
