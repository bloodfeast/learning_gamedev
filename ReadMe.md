# Rust Space Shooter Game

This is a simple 2D space shooter game built with Rust and the ggez game engine.

## Project Structure

The project is structured into two main modules:

- `actors`: This module contains the logic for the game entities such as the player, enemies, and projectiles. It also includes the models for these entities and their associated functions.

- `main`: This is the main entry point of the game. It contains the game loop and event handlers.

## Game Mechanics

- The player can move around the screen using the W, A, S, D keys.
- The player can shoot projectiles by clicking the left mouse button.
- The player unlocks a special attack on a 5s CD after 30 kills.
- Attacks and enemy HP scale with the kill count.
- Enemies spawn at random locations and move towards the player.
- The player and enemies take damage when they collide with each other or with projectiles.
- The game ends when the player's health reaches zero.

## Building and Running the Project

You need to have Rust and Cargo installed on your machine to build and run this project.

To build the project, navigate to the project directory and run:
```bash
cargo build
```
To run the project, use:
```bash
cargo run
```
## Contributing
Contributions are welcome! Please feel free to submit a pull request.  

## License
This project is licensed under the MIT License.