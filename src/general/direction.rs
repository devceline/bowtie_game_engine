
#[derive(Debug, Copy, Clone)]
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

impl Direction {
  pub fn subtract_direction(&self, other: Direction) -> Direction {
    let res = match self {
      Direction::Stationary => Direction::Stationary,
      Direction::Right => match other {
        Direction::Right => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::Left => match other {
        Direction::Left => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::Up => match other {
        Direction::Up => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::Down => match other {
        Direction::Down => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::UpLeft => match other {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Up,
        _ => self.to_owned()
      }
      Direction::UpRight => match other {
        Direction::Right => Direction::Up,
        Direction::Up => Direction::Right,
        _ => self.to_owned()
      }
      Direction::DownRight => match other {
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Down,
        _ => self.to_owned()
      }
      Direction::DownLeft => match other {
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Down,
        _ => self.to_owned()
      }
    };

    res
  }
  pub fn add_direction(&self, other: Direction) -> Direction {
    let res = match self {
      Direction::Stationary => other,
      Direction::Right => match other {
        Direction::Up => Direction::UpRight,
        Direction::Down => Direction::DownRight,
        Direction::Left => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::Left => match other {
        Direction::Up => Direction::UpLeft,
        Direction::Down => Direction::DownLeft,
        Direction::Right => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::Up => match other {
        Direction::Left => Direction::UpLeft,
        Direction::Right => Direction::UpRight,
        Direction::Down => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::Down => match other {
        Direction::Right => Direction::DownRight,
        Direction::Left => Direction::DownLeft,
        Direction::Up => Direction::Stationary,
        _ => self.to_owned(),
      },
      Direction::UpLeft => match other {
        Direction::Right => Direction::Stationary,
        Direction::Down => Direction::Stationary,
        _ => self.to_owned()
      }
      Direction::UpRight => match other {
        Direction::Left => Direction::Stationary,
        Direction::Down => Direction::Stationary,
        _ => self.to_owned()
      }
      Direction::DownRight => match other {
        Direction::Left => Direction::Stationary,
        Direction::Up => Direction::Stationary,
        _ => self.to_owned()
      }
      Direction::DownLeft => match other {
        Direction::Right => Direction::Stationary,
        Direction::Up => Direction::Stationary,
        _ => self.to_owned()
      }
    };

    res
  }
}
