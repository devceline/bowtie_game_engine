#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
  UpRight,
  DownRight,
  UpLeft,
  DownLeft,
  Stationary,
}

impl From<i32> for Direction {
  fn from(num: i32) -> Self {
    match num {
      0 => Direction::Up,
      1 => Direction::Right,
      2 => Direction::Down,
      3 => Direction::Left,
      4 => Direction::UpRight,
      5 => Direction::DownRight,
      6 => Direction::UpLeft,
      7 => Direction::DownLeft,
      8 => Direction::Stationary,
      _ => {
        panic!("Not a direction float {num}");
      }
    }
  }
}

impl Into<i32> for Direction {
  fn into(self) -> i32 {
    match self {
      Direction::Up => 0,
      Direction::Right => 1,
      Direction::Down => 2,
      Direction::Left => 3,
      Direction::UpRight => 4,
      Direction::DownRight => 5,
      Direction::UpLeft => 6,
      Direction::DownLeft => 7,
      Direction::Stationary => 8,
    }
  }
}

impl From<f32> for Direction {
  fn from(num: f32) -> Self {
    let num_int = num as i32;
    return Direction::from(num_int);
  }
}

impl Into<f32> for Direction {
  fn into(self) -> f32 {
    let num_int: i32 = self.into();
    return num_int as f32;
  }
}

impl Direction {
  pub fn subtract_direction(&self, other: Direction) -> Direction {
    let res = match self {
      Direction::Stationary => Direction::Stationary,
      Direction::Right => match other {
        Direction::Right => Direction::Stationary,
        Direction::UpRight => Direction::Stationary,
        Direction::DownRight => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::Left => match other {
        Direction::Left => Direction::Stationary,
        Direction::UpLeft => Direction::Stationary,
        Direction::DownLeft => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::Up => match other {
        Direction::Up => Direction::Stationary,
        Direction::UpRight => Direction::Stationary,
        Direction::UpLeft => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::Down => match other {
        Direction::Down => Direction::Stationary,
        Direction::DownLeft => Direction::Stationary,
        Direction::DownRight => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::UpLeft => match other {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::UpLeft => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::UpRight => match other {
        Direction::Right => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::UpRight => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::DownRight => match other {
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::DownRight => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::DownLeft => match other {
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::DownLeft => Direction::Stationary,
        _ => self.to_owned(),
      },
    };

    res
  }
  pub fn add_direction(&self, other: Direction) -> Direction {
    let res = match self {
      Direction::Stationary => other,
      Direction::Right => match other {
        Direction::Up => Direction::UpRight,
        Direction::Down => Direction::DownRight,
        Direction::Left => Direction::Left,
        _ => self.to_owned(),
      },
      Direction::Left => match other {
        Direction::Up => Direction::UpLeft,
        Direction::Down => Direction::DownLeft,
        Direction::Right => Direction::Right,
        _ => self.to_owned(),
      },
      Direction::Up => match other {
        Direction::Left => Direction::UpLeft,
        Direction::Right => Direction::UpRight,
        Direction::Down => Direction::Down,
        _ => self.to_owned(),
      },
      Direction::Down => match other {
        Direction::Right => Direction::DownRight,
        Direction::Left => Direction::DownLeft,
        Direction::Up => Direction::Up,
        _ => self.to_owned(),
      },
      Direction::UpLeft => match other {
        Direction::Right => Direction::UpRight,
        Direction::Down => Direction::DownLeft,
        _ => self.to_owned(),
      },
      Direction::UpRight => match other {
        Direction::Left => Direction::UpLeft,
        Direction::Down => Direction::DownRight,
        _ => self.to_owned(),
      },
      Direction::DownRight => match other {
        Direction::Left => Direction::DownLeft,
        Direction::Up => Direction::UpRight,
        _ => self.to_owned(),
      },
      Direction::DownLeft => match other {
        Direction::Right => Direction::DownRight,
        Direction::Up => Direction::UpLeft,
        _ => self.to_owned(),
      },
    };

    res
  }
  pub fn as_vector(&self) -> (f32, f32) {
    match self {
      Direction::Up => (0.0, 1.0),
      Direction::Down => (0.0, -1.0),
      Direction::Left => (-1.0, 0.0),
      Direction::Right => (1.0, 0.0),
      Direction::UpLeft => (-1.0, 1.0),
      Direction::UpRight => (1.0, 1.0),
      Direction::DownLeft => (-1.0, -1.0),
      Direction::DownRight => (1.0, -1.0),
      Direction::Stationary => (0.0, 0.0),
    }
  }
}
