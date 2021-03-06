extern crate bowtie;
use bowtie::{
  math::general::absolute_value_f32,
  premade_components::{CollisionComponent, GravityComponent},
  Color, Component, Direction, Drawable, Entity, Message, Rectangle, Sprite,
};

pub struct PlayableCharacter<'s> {
  sprite: Sprite<'s, Rectangle>,
  speed: f32,
  components: Vec<*mut dyn Component<'s>>,
  direction: Direction,
  collision_direction: Direction,
}

impl<'e> Entity<'e> for PlayableCharacter<'e> {
  fn get_drawable(&'e self) -> &'e dyn Drawable<'e> {
    &self.sprite
  }

  fn get_x(&self) -> f32 {
    self.sprite.get_x()
  }

  fn get_y(&self) -> f32 {
    self.sprite.get_y()
  }

  fn set_x(&mut self, x: f32) -> bool {
    self.sprite.set_x(x);
    true
  }

  fn set_y(&mut self, y: f32) -> bool {
    let direction = if y < self.get_y() {
      Direction::Down
    } else {
      Direction::Up
    };
    let amount = absolute_value_f32(y - self.get_y());

    let net = direction.subtract_direction(self.collision_direction);

    if net == Direction::Stationary {
      false;
    }

    self.sprite.move_sprite(
      net,
      amount,
    );

    true
  }

  fn get_width(&self) -> f32 {
    self.sprite.get_width()
  }

  fn get_height(&self) -> f32 {
    self.sprite.get_height()
  }

  fn load_components(&mut self, component: *mut dyn Component<'e>) {
    self.components.push(component);
  }

  fn get_components(&mut self) -> &Vec<*mut dyn Component<'e>> {
    &self.components
  }

  fn recieve_message(&mut self, message: Message) {
    let message_name = message.get_message_type();

    if message_name == GravityComponent::get_message_name() {
      self.set_y(self.get_y() - message.get_values()["speed"]);
    }

    if message_name == CollisionComponent::get_message_name() {
      self.collision_direction = message.get_values()["with"].into();
      if self.collision_direction != Direction::Stationary {
        println!("Player collided {:?}", self.collision_direction);
      }
    }
  }
}

impl<'s> PlayableCharacter<'s> {
  pub fn new(sprite: Sprite<'s, Rectangle>) -> PlayableCharacter<'s> {
    PlayableCharacter {
      sprite,
      components: vec![],
      direction: Direction::Stationary,
      collision_direction: Direction::Stationary,
      speed: 0.03,
    }
  }

  pub fn set_color_overlay(&mut self, color: Color) {
    self.sprite.set_color_overlay(color);
  }

  pub fn set_collision_direction(&mut self, direction: Direction) {
    self.collision_direction = direction;
  }

  pub fn flip_horizontal(&mut self) {
    self.sprite.flip_horizontal();
  }

  pub fn handle_direction_change(
    &mut self,
    direction: Direction,
    subtract: bool,
  ) {
    if subtract {
      self.direction = self.direction.subtract_direction(direction);
    } else {
      self.direction = self.direction.add_direction(direction);
    }
  }

  pub fn move_character(
    &mut self,
    direction: Direction,
    subtract: bool,
    speed: f32,
  ) {
    self.handle_direction_change(direction, subtract);
    let net_direction =
      self.direction.subtract_direction(self.collision_direction);
    println!(
      "Player requested to move to {:?}, but subtracting {:?} resulted in {:?}",
      direction, self.collision_direction, net_direction
    );
    self.sprite.move_sprite(net_direction, speed);
  }

  pub async fn respond_to_event(&mut self, event: &glfw::WindowEvent) {
    let is_release = match event {
      glfw::WindowEvent::Key(_, _, glfw::Action::Release, _) => true,
      _ => false,
    };

    match event {
      glfw::WindowEvent::Key(glfw::Key::Right, _, _, _) => {
        self.move_character(Direction::Right, is_release, self.speed);
      }
      glfw::WindowEvent::Key(glfw::Key::Left, _, _, _) => {
        self.move_character(Direction::Left, is_release, self.speed);
      }
      glfw::WindowEvent::Key(glfw::Key::Up, _, _, _) => {
        self.move_character(Direction::Up, is_release, self.speed * 1.5);
      }
      glfw::WindowEvent::Key(glfw::Key::Down, _, _, _) => {
        self.move_character(Direction::Down, is_release, self.speed);
      }
      _ => {
        self.direction = Direction::Stationary;
      }
    }
  }
}
