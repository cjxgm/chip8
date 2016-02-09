//! Traits for peripherals.

/// Synchronization.
pub trait Timer {
    /// Wait for the next V-Sync (60 Hz)
    fn wait_next_frame(&self);
}

/// Graphics controller.
pub trait Video {
    /// Clears the screen.
    fn clear(&self);
    /// Flip the color at (x, y) coordinate of the screen, returning the new value.
    fn flip(&self, x: usize, y: usize) -> bool;
    /// Present the screen to display.
    fn present(&self);
}

/// Sound controller.
pub trait Audio {
    /// Enable or disable buzzer
    fn buzz(&self, on: bool);
}

/// Input state controller.
pub trait Input {
    /// Determine if key is pressed down (`true`) or not (`false`)
    fn keydown(&self, which: u8) -> bool;
}

