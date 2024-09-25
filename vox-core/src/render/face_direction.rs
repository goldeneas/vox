#[derive(Debug, Clone, Copy)]
pub enum FaceDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    FRONT,
    BACK,
}

impl FaceDirection {
    pub fn to_index(direction: Self) -> u32 {
        match direction {
            FaceDirection::UP    => 0,
            FaceDirection::DOWN  => 1,
            FaceDirection::RIGHT => 2,
            FaceDirection::LEFT  => 3,
            FaceDirection::FRONT => 4,
            FaceDirection::BACK  => 5,
        }
    }

    pub fn from_bgm(bgm_direction: usize) -> Self {
        debug_assert!(bgm_direction < 6, "Unknown bgm direction");

        match bgm_direction {
            0 => FaceDirection::UP,
            1 => FaceDirection::DOWN,
            2 => FaceDirection::RIGHT,
            3 => FaceDirection::LEFT,
            4 => FaceDirection::FRONT,
            5 => FaceDirection::BACK,
            _ => unreachable!(),
        }
    }
}
