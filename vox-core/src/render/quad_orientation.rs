#[derive(Debug, Clone, Copy)]
pub enum QuadOrientation {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    FRONT,
    BACK,
}

impl QuadOrientation {
    pub fn to_index(direction: Self) -> u32 {
        match direction {
            QuadOrientation::UP    => 0,
            QuadOrientation::DOWN  => 1,
            QuadOrientation::RIGHT => 2,
            QuadOrientation::LEFT  => 3,
            QuadOrientation::FRONT => 4,
            QuadOrientation::BACK  => 5,
        }
    }

    pub fn from_bgm(bgm_direction: usize) -> Self {
        debug_assert!(bgm_direction < 6, "Unknown bgm direction");

        match bgm_direction {
            0 => QuadOrientation::UP,
            1 => QuadOrientation::DOWN,
            2 => QuadOrientation::RIGHT,
            3 => QuadOrientation::LEFT,
            4 => QuadOrientation::FRONT,
            5 => QuadOrientation::BACK,
            _ => unreachable!(),
        }
    }
}
