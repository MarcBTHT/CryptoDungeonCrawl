# CryptoDungeonCrawl
CryptoDungeonCrawl is a text-based dungeon crawler game where the in-game environment is dynamically influenced by real-world external information, such as the price of Bitcoin. As external factors change, the game world adapts, introducing new challenges, enemies, and rewards.

## Features
- **Dynamic Environment:** The game world changes based on real-time external information.
- **Bitcoin Influence:** Fluctuations in Bitcoin prices affect in-game events, mobs, and rewards.
- **Text-Based Adventure:** Immerse yourself in a classic dungeon crawler experience with a focus on storytelling.

### Roadmap

1. **API Integration**
   - [x] Set up API to collect real-time market data, such as Bitcoin prices.

2. **Dungeon Game Development**
   - **Build a Dungeon Crawler**
     - [x] Storing the dungeon map.
     - [x] Adding the adventurer.
     - [x] Create a real-time game.
     - [x] Building a dungeon.
         
   - **Compose Dungeon Denizens**
     - [x] Composing entities.
     - [x] Installing and using Legion (an ECS library).
     - [x] Composing the player.
     - [x] Managing complexity with systems.
     - [x] Adding monsters.
     - [x] Collision detection.

   - **Take Turns with the Monsters**
     - [x] Making monsters wander randomly.
     - [x] !!!! Moving entities in real-time !!!! [**Commit**](https://github.com/MarcBTHT/CryptoDungeonCrawl/commit/cd4845fa4a12fc8b77eb950444fcbde77029d928)
       - I replaced random_move with monsters_move, where all monsters move 1 square in the player's direction (basic algorithm, you could do an A* algo).
       - Then I split the program, because before the monsters logic was in the ECS system, which is executed every turn (every player input).
         - I divided it into main_schedule and periodic schedule (mod.rs).
         - Then, in the tick function (main.rs), I call the periodic schedule, which moves every 1s (after which it changes according to the BTC price) and the main schedule as before.

   - **Health and Melee Combat**
     - [x] Giving entities hit points.
     - [x] Adding a heads-up display.
     - [x] Implementing combat:
       I implement my own solution:
         - I modify collisions.rs. I strungle because we can't borrow `*ecs` as mutable because it's also borrowed as immutable
          mutable borrowing occurs. And I add other difficulties. And I had other difficulties.

3. **API and Game Integration**
   - [x] Establish a connection between the API and the game.
   - [x] Develop mechanisms to dynamically adjust in-game elements based on real-world market data:
     - Depending on the price of BTC, the speed of monsters changes

4. **Multiplayer Integration**
   - [ ] Add multiplayer functionality to the game.
   - [ ] Implement multiplayer interactions, such as cooperative gameplay or competitive challenges.

5. **Player-driven Actions**
   - [ ] Enable players to perform external actions that influence the game.
   - [ ] Define a set of actions players can take outside the game to disadvantage or challenge other players.

## Demo

https://github.com/MarcBTHT/CryptoDungeonCrawl/assets/116173196/27caa4f5-5131-42d5-8df0-92c08d8938db


