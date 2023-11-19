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
     - [ ] Storing the dungeon map.
     - [ ] Adding the adventurer.
     - [ ] Building a dungeon.
         
   - **Compose Dungeon Denizens**
     - [ ] Composing entities.
     - [ ] Installing and using Legion (an ECS library).
     - [ ] Composing the player.
     - [ ] Managing complexity with systems.
     - [ ] Adding monsters.
     - [ ] Collision detection.

   - **Take Turns with the Monsters**
     - [ ] Making monsters wander randomly.
     - [ ] Moving entities in a turn-based game.
     - [ ] Sending messages of intent.

   - **Health and Melee Combat**
     - [ ] Giving entities hit points.
     - [ ] Adding a heads-up display.
     - [ ] Implementing combat.
     - [ ] Waiting as a strategy.

   - **Victory and Defeat**
     - [ ] Building a smarter monster.
     - [ ] Implementing a game over screen.
     - [ ] Finding the Amulet of Yala.

   - **Fields of View**
     - [ ] Defining an entity’s field of view.
     - [ ] Limiting monsters’ fields of view.
     - [ ] Adding spatial memory.

   - **More Interesting Dungeons**
     - [ ] Creating traits.
     - [ ] Creating cellular automata maps.
     - [ ] Creating drunkard’s walk maps.
     - [ ] Prefabricating map sections.

   - **Map Themes**
     - [ ] Theming your dungeon.
     - [ ] Rendering with themes.
     - [ ] Unleashing your imagination.

   - **Inventory and Power-Ups**
     - [ ] Designing items.
     - [ ] Managing inventory.
    
   - **Deeper Dungeons**
     - [ ] Adding stairs to the map.
     - [ ] Tracking game level.
     - [ ] Displaying the current level on the HUD.

   - **Combat Systems and Loot**
     - [ ] Designing data-driven dungeons.
     - [ ] Extending the combat system.
     - [ ] Adding more swords.

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
