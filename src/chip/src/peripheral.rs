//! Traits for peripherals.

/// Synchronization.
pub trait Timer {
    /// Wait for the next V-Sync (60 Hz).
    /// Returns `true` on quit-request.
    fn pump(&mut self) -> bool;
}

/// Graphics controller.
pub trait Video {
    /// Clears the screen.
    fn clear(&mut self);
    /// Draw sprite by flipping color.
    /// Returns `true` if anything has been flipped to `false`.
    fn draw(&mut self, x: isize, y: isize, sprite: &[u8]) -> bool;
    /// Present the screen to display.
    fn present(&self);
}

/// Sound controller.
pub trait Audio {
    /// Enable or disable buzzer
    fn buzz(&mut self, on: bool);
}

/// Input state controller.
pub trait Input {
    /// Determine if key is pressed down (`true`) or not (`false`)
    fn keydown(&self, which: usize) -> bool;
    /// Wait until any key is pressed.
    /// Returns `None` if quit-requested.
    fn key(&self) -> Option<u8>;
}

