//! Add theme animations to widgets.

use iced_core::Color;

/// Hover animation of the widget
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HoverPressedAnimation {
    /// Animation direction: forward means it goes from non-hovered to hovered state
    pub direction: AnimationDirection,
    /// The instant the animation was started at (`None` if it is not running)
    pub started_at: Option<std::time::Instant>,
    /// The progress of the animationn, between 0.0 and 1.0
    pub animation_progress: f32,
    /// The progress the animation has been started at
    pub initial_progress: f32,
    /// The type of effect for the animation
    pub effect: AnimationEffect,
}

/// The type of effect for the animation
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AnimationEffect {
    /// Transition is linear.
    #[default]
    Linear,
    /// Transition is a cubic ease out.
    EaseOut,
    /// Transistion is instantaneous.
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// Direction of the animation
pub enum AnimationDirection {
    #[default]
    /// The animation goes forward
    Forward,
    /// The animation goes backward
    Backward,
}

impl HoverPressedAnimation {
    /// Create a hover animation with the given transision effect
    pub fn new(effect: AnimationEffect) -> Self {
        Self {
            effect,
            ..Default::default()
        }
    }

    /// Check if the animation is running
    pub fn is_running(&self) -> bool {
        self.started_at.is_some()
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.direction = AnimationDirection::Forward;
        self.started_at = None;
        self.animation_progress = 0.0;
        self.initial_progress = 0.0;
    }

    /// Update the animation progress, if necessary, and returns the need to request a redraw.
    pub fn on_redraw_request_update(
        &mut self,
        forward_duration_ms: u32,
        backward_duration_ms: u32,
        now: std::time::Instant,
    ) -> bool {
        // Is the animation running ?
        if let Some(started_at) = self.started_at {
            if forward_duration_ms == 0 {
                self.animation_progress = 1.0;
            }

            // Reset the animation once it has gone forward and now fully backward
            if self.animation_progress == 0.0 && self.direction == AnimationDirection::Backward {
                self.started_at = None;
            } else {
                // Evaluate new progress
                match &mut self.effect {
                    AnimationEffect::Linear => match self.direction {
                        AnimationDirection::Forward => {
                            self.animation_progress = (self.initial_progress
                                + (((now - started_at).as_millis() as f64)
                                    / (forward_duration_ms as f64))
                                    as f32)
                                .clamp(0.0, 1.0);
                        }
                        AnimationDirection::Backward => {
                            self.animation_progress = (self.initial_progress
                                - (((now - started_at).as_millis() as f64)
                                    / (backward_duration_ms as f64))
                                    as f32)
                                .clamp(0.0, 1.0);
                        }
                    },
                    AnimationEffect::EaseOut => match self.direction {
                        AnimationDirection::Forward => {
                            self.animation_progress = (self.initial_progress
                                + ease_out_cubic(
                                    ((now - started_at).as_millis() as f32)
                                        / (forward_duration_ms as f32),
                                ))
                            .clamp(0.0, 1.0);
                        }
                        AnimationDirection::Backward => {
                            self.animation_progress = (self.initial_progress
                                - ease_out_cubic(
                                    ((now - started_at).as_millis() as f32)
                                        / (backward_duration_ms as f32),
                                ))
                            .clamp(0.0, 1.0);
                        }
                    },
                    AnimationEffect::None => {}
                }
            }
            return true;
        }
        false
    }

    /// Update the hovered state and return the need to request a redraw.
    pub fn on_cursor_moved_update(&mut self, is_mouse_over: bool) -> bool {
        if is_mouse_over {
            // Is it already running ?
            if self.started_at.is_some() {
                // This is when the cursor re-enters the widget's area before the animation finishes
                if self.direction == AnimationDirection::Backward {
                    // Change animation direction
                    self.direction = AnimationDirection::Forward;
                    // Start from where the animation was at
                    self.initial_progress = self.animation_progress;
                    self.started_at = Some(std::time::Instant::now());
                }
            } else {
                // Start the animation
                self.direction = AnimationDirection::Forward;
                self.started_at = Some(std::time::Instant::now());
                self.animation_progress = 0.0;
                self.initial_progress = 0.0;
            }
            self.animation_progress != 1.0
        } else if self.started_at.is_some() {
            // This is when the cursor leaves the widget's area
            match self.direction {
                AnimationDirection::Forward => {
                    // Change animation direction
                    self.direction = AnimationDirection::Backward;
                    // Start from where the animation was at
                    self.initial_progress = self.animation_progress;
                    self.started_at = Some(std::time::Instant::now());
                    true
                }
                AnimationDirection::Backward => true,
            }
        } else {
            false
        }
    }

    /// Start the animation when pressed.
    pub fn on_press(&mut self) {
        self.started_at = Some(std::time::Instant::now());
        self.direction = AnimationDirection::Forward;
        self.animation_progress = 0.0;
        self.initial_progress = 0.0;
    }

    /// End the animation when released.
    pub fn on_released(&mut self) {
        self.started_at = Some(std::time::Instant::now());
        self.direction = AnimationDirection::Backward;
        self.initial_progress = self.animation_progress;
    }

    /// End the animation (go backgwards), skipping the forward phase.
    pub fn on_activate(&mut self) {
        self.started_at = Some(std::time::Instant::now());
        self.direction = AnimationDirection::Backward;
        self.initial_progress = 1.0;
        self.animation_progress = 1.0;
    }
}

/// Based on Robert Penner's infamous easing equations, MIT license.
fn ease_out_cubic(t: f32) -> f32 {
    let p = t - 1f32;
    p * p * p + 1f32
}

/// Mix with another color with the given ratio (should be in `iced/core/src/color.rs` ?)
pub fn mix(mut color: Color, other: Color, ratio: f32) -> Color {
    let self_ratio = 1.0 - ratio;
    color.r = (color.r * self_ratio + other.r * ratio).clamp(0.0, 1.0);
    color.g = (color.g * self_ratio + other.g * ratio).clamp(0.0, 1.0);
    color.b = (color.b * self_ratio + other.b * ratio).clamp(0.0, 1.0);
    color.a = (color.a * self_ratio + other.a * ratio).clamp(0.0, 1.0);
    color
}