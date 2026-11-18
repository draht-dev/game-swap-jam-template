//! Pure game logic. No rendering or engine dependency.

pub const WORLD_WIDTH: f32 = 800.0;
pub const WORLD_HEIGHT: f32 = 600.0;
pub const PLAYER_SIZE: f32 = 32.0;
pub const PLAYER_SPEED: f32 = 240.0;
pub const COLLECTIBLE_SIZE: f32 = 18.0;

/// Fixed spawn points for the collectible, cycled after each pickup.
pub const SPAWN_POINTS: [(f32, f32); 5] = [
    (120.0, 80.0),
    (640.0, 120.0),
    (200.0, 460.0),
    (680.0, 420.0),
    (400.0, 80.0),
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

impl Input {
    /// Unit movement vector. Opposing keys cancel, diagonals are normalized.
    pub fn axis(&self) -> (f32, f32) {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        if self.left {
            x -= 1.0;
        }
        if self.right {
            x += 1.0;
        }
        if self.up {
            y -= 1.0;
        }
        if self.down {
            y += 1.0;
        }
        let len = (x * x + y * y).sqrt();
        if len > 0.0 {
            (x / len, y / len)
        } else {
            (0.0, 0.0)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub player: Rect,
    pub collectible: Rect,
    pub score: u32,
    spawn_index: usize,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self::with_spawn(0)
    }

    pub fn with_spawn(index: usize) -> Self {
        let spawn_index = index % SPAWN_POINTS.len();
        let player = Rect::new(
            (WORLD_WIDTH - PLAYER_SIZE) * 0.5,
            (WORLD_HEIGHT - PLAYER_SIZE) * 0.5,
            PLAYER_SIZE,
            PLAYER_SIZE,
        );
        let mut state = Self {
            player,
            collectible: collectible_at(spawn_index),
            score: 0,
            spawn_index,
        };
        state.avoid_player_overlap();
        state
    }

    pub fn update(&mut self, input: Input, dt: f32) {
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        let dt = dt.min(0.05);
        let (ax, ay) = input.axis();
        self.player.x =
            (self.player.x + ax * PLAYER_SPEED * dt).clamp(0.0, WORLD_WIDTH - self.player.w);
        self.player.y =
            (self.player.y + ay * PLAYER_SPEED * dt).clamp(0.0, WORLD_HEIGHT - self.player.h);

        if self.player.overlaps(&self.collectible) {
            self.score += 1;
            self.spawn_index = (self.spawn_index + 1) % SPAWN_POINTS.len();
            self.collectible = collectible_at(self.spawn_index);
            self.avoid_player_overlap();
        }
    }

    fn avoid_player_overlap(&mut self) {
        let start = self.spawn_index;
        while self.player.overlaps(&self.collectible) {
            self.spawn_index = (self.spawn_index + 1) % SPAWN_POINTS.len();
            self.collectible = collectible_at(self.spawn_index);
            if self.spawn_index == start {
                break;
            }
        }
    }
}

fn collectible_at(index: usize) -> Rect {
    let (x, y) = SPAWN_POINTS[index % SPAWN_POINTS.len()];
    Rect::new(x, y, COLLECTIBLE_SIZE, COLLECTIBLE_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn right() -> Input {
        Input {
            right: true,
            ..Input::default()
        }
    }

    fn left() -> Input {
        Input {
            left: true,
            ..Input::default()
        }
    }

    fn up() -> Input {
        Input {
            up: true,
            ..Input::default()
        }
    }

    fn approx(a: f32, b: f32) {
        assert!((a - b).abs() < 1e-4, "{a} != {b}");
    }

    #[test]
    fn new_state_has_zero_score_and_centered_player() {
        let state = State::new();
        assert_eq!(state.score, 0);
        approx(state.player.x, (WORLD_WIDTH - PLAYER_SIZE) * 0.5);
        approx(state.player.y, (WORLD_HEIGHT - PLAYER_SIZE) * 0.5);
        assert!(!state.player.overlaps(&state.collectible));
    }

    #[test]
    fn right_input_moves_player_by_speed_times_dt() {
        let mut state = State::new();
        let start_x = state.player.x;
        state.update(right(), 0.025);
        approx(state.player.x, start_x + PLAYER_SPEED * 0.025);
        approx(state.player.y, (WORLD_HEIGHT - PLAYER_SIZE) * 0.5);
    }

    #[test]
    fn opposing_keys_cancel() {
        let mut state = State::new();
        let start = state.clone();
        state.update(
            Input {
                left: true,
                right: true,
                ..Input::default()
            },
            0.02,
        );
        assert_eq!(state.player, start.player);
    }

    #[test]
    fn diagonal_movement_is_normalized() {
        let mut state = State::new();
        let start_x = state.player.x;
        let start_y = state.player.y;
        let dt = 0.02;
        state.update(
            Input {
                right: true,
                down: true,
                ..Input::default()
            },
            dt,
        );
        let dx = state.player.x - start_x;
        let dy = state.player.y - start_y;
        let dist = (dx * dx + dy * dy).sqrt();
        approx(dist, PLAYER_SPEED * dt);
        assert!((dx - dy).abs() < 1e-4);
    }

    #[test]
    fn player_is_clamped_to_world_bounds() {
        let mut state = State::new();
        state.player.x = 0.0;
        state.player.y = 0.0;
        state.update(
            Input {
                left: true,
                up: true,
                ..Input::default()
            },
            1.0,
        );
        approx(state.player.x, 0.0);
        approx(state.player.y, 0.0);

        state.player.x = WORLD_WIDTH - PLAYER_SIZE;
        state.player.y = WORLD_HEIGHT - PLAYER_SIZE;
        state.update(
            Input {
                right: true,
                down: true,
                ..Input::default()
            },
            1.0,
        );
        approx(state.player.x, WORLD_WIDTH - PLAYER_SIZE);
        approx(state.player.y, WORLD_HEIGHT - PLAYER_SIZE);
    }

    #[test]
    fn collecting_increases_score_and_moves_collectible() {
        let mut state = State::with_spawn(0);
        let first = state.collectible;
        state.player.x = first.x;
        state.player.y = first.y;
        state.update(Input::default(), 0.016);
        assert_eq!(state.score, 1);
        assert_ne!(state.collectible, first);
        assert!(!state.player.overlaps(&state.collectible));
    }

    #[test]
    fn collecting_twice_cycles_spawn_points() {
        let mut state = State::with_spawn(0);
        for expected in 1u32..=2 {
            state.player.x = state.collectible.x;
            state.player.y = state.collectible.y;
            state.update(Input::default(), 0.016);
            assert_eq!(state.score, expected);
        }
        let (x, y) = SPAWN_POINTS[2];
        approx(state.collectible.x, x);
        approx(state.collectible.y, y);
    }

    #[test]
    fn non_positive_or_non_finite_dt_does_nothing() {
        let mut state = State::new();
        let start = state.clone();
        for dt in [0.0, -0.1, f32::NAN, f32::INFINITY] {
            state.update(right(), dt);
            assert_eq!(state, start, "dt={dt}");
        }
    }

    #[test]
    fn update_is_deterministic() {
        let mut a = State::new();
        let mut b = State::new();
        let input = Input {
            right: true,
            up: true,
            ..Input::default()
        };
        for _ in 0..10 {
            a.update(input, 0.016);
            b.update(input, 0.016);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn left_and_up_move_in_expected_direction() {
        let mut state = State::new();
        let start_x = state.player.x;
        let start_y = state.player.y;
        state.update(left(), 0.01);
        assert!(state.player.x < start_x);
        state.update(up(), 0.01);
        assert!(state.player.y < start_y);
    }
}
