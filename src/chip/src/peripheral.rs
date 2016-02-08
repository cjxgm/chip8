//! Traits for peripherals.

/// Display images on screen.
pub trait Video {
    /// clears the screen
    fn clear();
    /// flip the color at (x, y) coordinate, returning the new value
    fn flip(x: usize, y: usize) -> bool;
}

/// Playback sound to speaker.
pub trait Audio {
}

/// Wait for or poll input from keyboard.
pub trait Input {
}

