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
     - [x] !!!! Moving entities in real-time !!!!
       - I replaced random_move with monsters_move, where all monsters move 1 square in the player's direction (basic algorithm, you could do an A* algo).
       - Then I split the program, because before the monsters logic was in the ECS system, which is executed every turn (every player input).
         - I divided it into main_schedule and periodic schedule (mod.rs).
         - Then, in the tick function (main.rs), I call the periodic schedule, which moves every 1s (after which it changes according to the BTC price) and the main schedule as before.

   - **Health and Melee Combat**
     - [ ] Giving entities hit points.
     - [ ] Adding a heads-up display.
     - [ ] Implementing combat.
     - [ ] Waiting as a strategy.

   - **Victory and Defeat**
     - [ ] Building a smarter monster.
     - [ ] Implementing a game over screen.
     - [ ] Finding the Amulet of Yala.

3. **API and Game Integration**
   - [ ] Establish a connection between the API and the game.
   - [ ] Develop mechanisms to dynamically adjust in-game elements based on real-world market data.
   - [ ] Implement events triggered by changes in external information, affecting the game environment.

4. **Multiplayer Integration**
   - [ ] Add multiplayer functionality to the game.
   - [ ] Implement multiplayer interactions, such as cooperative gameplay or competitive challenges.

5. **Player-driven Actions**
   - [ ] Enable players to perform external actions that influence the game.
   - [ ] Define a set of actions players can take outside the game to disadvantage or challenge other players.
