use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum HitResult {
    Water,
    Hit,
    Sunk,
}

