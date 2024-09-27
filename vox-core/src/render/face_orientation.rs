#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FaceOrientation {
    UP,
    DOWN,
    RIGHT,
    LEFT,
    FRONT,
    BACK,
}

impl FaceOrientation {
    pub fn index(&self) -> u32 {
        match *self {
            FaceOrientation::UP    => 0,
            FaceOrientation::DOWN  => 1,
            FaceOrientation::RIGHT => 2,
            FaceOrientation::LEFT  => 3,
            FaceOrientation::FRONT => 4,
            FaceOrientation::BACK  => 5,
        }
    }

    pub fn from_bgm(bgm_direction: usize) -> Self {
        debug_assert!(bgm_direction < 6, "Unknown bgm direction");

        match bgm_direction {
            0 => FaceOrientation::UP,
            1 => FaceOrientation::DOWN,
            2 => FaceOrientation::RIGHT,
            3 => FaceOrientation::LEFT,
            4 => FaceOrientation::FRONT,
            5 => FaceOrientation::BACK,
            _ => unreachable!(),
        }
    }
}
