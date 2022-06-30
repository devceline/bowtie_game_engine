# Bowtie Game Engine

2D Game engine written in Rust

# Current Capabilities

## Rendering

Rendering through [OpenGl](https://www.opengl.org/). Naturally, this makes it 
somewhat incompatible with the newer M1 Macs since since they are [deprecating
OpenGl support](https://developer.apple.com/forums/thread/694866).

- Shapes:
  - Rectangle
- Sprites:
  - Can take any of the implemented Shapes
  - Can take any PNG file and load it as a texture (with different filtering options)
  - Utilities to move them around seamlessly with [Directions](#directions)

## Entity System:

Entities are a dynamic way to add your own entities and component systems and
load them into the game engine. You can implement your own components and have
them do practically whatever you want through the messaging system.

- Entity trait to make any `struct` loadable in the game engine
- Component trait to add functionalities dynamically to any entity
- Messaging system so components and entities can asynchronously communicate

### Pre-implemented Components
- Collision: Reports collision between entities (multi directional)
- Gravity: Drags objects down with acceleration
- Event: To allow for any type of message sending

## General utilitites

### Directions

Directions enum with capabilities to "add direction", eg:

```rust
let direction = Direction::Up;
let new_direction = direction.add_direction(Direction::Right);
new_direction == Direction::UpRight
```

### Color

Color struct that uses -1.0 to 1.0 normalized color values with helper enum, eg:

```rust
  let color = Color(1.0, 0.0, 0.0, 1.0);
  let red: Color = COLORS::Red.into();
  color == red
```


# Planned Capabilities

- All the things a game engine (for 2D games) is supposed to do

## Next up:
- More convenient entity loading
- More seamless movement
- More Newtonian-ly accurate gravity

# Author

Yours truly, Celine Sarafa


# Contributions

Contributions are not welcome at this time sorry :>






