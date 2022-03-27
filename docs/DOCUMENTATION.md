# Project 4 - Player Platforms

## Contents

1. [Todo / Success Criteria](#todo--success-criteria)
2. [Goal](#goal)
    - [Why am I Making This Game?](#why-am-i-making-this-game)
3. [Analysis](#analysis)
    - [A1 - Computational Approach and Methods](#a1---computational-approach-and-methods)
    - [A2 - Stakeholders](#a2---stakeholders)
    - [A3 - Research](#a3---research)
    - [A4 - Essential Features](#a4---essential-features)
    - [A5 - Limitations of Proposed Solution](#a5---limitations-of-proposed-solution)
    - [A6 - Requirements](#a6---requirements)
    - [A7 - Success Criteria](#a7---success-criteria)

## TODO

- [x] Basic Rendering
- [x] Collisions
- [ ] Intractable objects
- [ ] Platforms
- [ ] Levels
- [ ] Local Multiplayer

### Extras

- [ ] Rendering with Textures
- [ ] Moving platforms
- [ ] More levels
- [ ] Online Multiplayer

## Goal

The Goal of this project is to create a fun multiplayer game that allows people to work as a team to
solve a variety of interesting and challenging puzzles.

### Why am I Making This Game?

I am making this game to encourage people to work as a team, resulting in people enhancing their
teamwork, social skills and ability to solve complex problems that require the use of multiple
people. Enhancing these skills can give people a significant advantage in employment as all of these
skills are used in almost all fields.

## Analysis

### A1 - Computational Approach and Methods

A team building application such as a video game is suitable for a computational approach as it
allows for the game to provide great consequences for lack of teamwork in turn encouraging the team
to work better together.

The game will be built using reusable components to allow for easy addition of new features and game
mechanics. Another advantage of using reusable components is that the code will be easier to
maintain.

To streamline the use of reusable components an Entity Component System (ECS)
will be used. This will both help to overcome some design challenges related to the Rust programming
allowing for the consistent use of memory safe code, while also creating an easy method of
interaction between components by being able to write queries to access all entities that match a
certain criteria such as colliding with a player.

### A2 - Stakeholders

| Stakeholder           | Descriptions | Role                                                                                                                                 | Stakeholder Needs / Requirements* How they will use the system |
|-----------------------|--------------|--------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------|
| Miles Ray             |              | WIll be focusing on game robustness (Will be trying to find bugs / use cheats during multiplayer gameplay)                           |                                                                |
| Kristian Dunn         |              | Novice (Has not played puzzle platformer games before)                                                                               |                                                                |
| Oliver Kendall        |              | Will be focusing on usability features in the game (such as ability to use an XBox/Playstation controller or use of a dyslexic font) |                                                                |
| Ned Brooker           |              | WIll be focusing on the features of the game (Are the levels fun, does there need to be more features)                               |                                                                |
| Ollie McCandlish      |              | Expert (Has played many different types of puzzle platformer games)                                                                  |                                                                |
| Oscar Hitchcock-Smith |              | Expert (Has played many different types of puzzle platformer games)                                                                  |                                                                |

### A3 - Research

- Pico Park
    - Only certain players can move certain objects: <br/>
      Certain movable blocks are colour to match that of certain players to indicate that only the
      player with that colour is able to move it. I will be implementing this feature into my game
      as blocks that all players except the one matching thr block's colour will collide with as
      this makes me able to block off specific paths to be only accessible by specific players,
      forcing the players to work as a team to reach the exit.
    - Timer to encourage fast thinking: <br/>
      A level must be completed before the timer runs out otherwise the level must be restarted, I
      will not be placing any timers forcing the players to complete a level within a certain time
      into my game as I feel it limits the players too much, however I will be adding timers that
      disable an enabled button after a certain period of time if the button is pressed as it gives
      the players time to plan to reach the exit but means they still have to execute their plan
      quickly.
    - Not all 'death' objects restart level: <br/>
      This prevents the players for getting annoyed if they keep falling down a certain hole,
      meaning it is less likely they will get annoyed and quit the game as they need to redo the
      previous sections of the level, I will be doing a similar thing in that if a player dies they
      will just respawn at the beginning of a section of a level, meaning it is less likely they
      will get annoyed and close the game.
    - Ability for players to climb onto one another: <br/>
      This allows the players to access new areas just by using each other as a boost to the height
      that they can jump, I will be implementing this feature and using it to make certain areas
      inaccessible to the player's unless they jump onto of each other.
    - Size changing: <br/>
      Certain buttons within levels will increase/decrease the size of the player allowing them to
      move through smaller gaps or jump onto a larger player to reach higher areas, I will not be
      implementing this feature as I feel it is not worth implementing, however if I change my mind
      later into development, it should be possible to add this feature into the game.
    - Endless mode: <br/>
      Players can enter an 'Endless' mode where the levels are procedurally generated according to
      pre-written templates, I will not be implementing this feature as there is too much complexity
      in making the computer generate a level using pre-written templates.
    - Different player need to do different tasks in parallel with each other: <br/>
      Players are forced into different sections of the screen using, immovable objects or platforms
      and need to perform certain tasks, such as pressing a button at the same time in order to
      reach the exit. I feel this feature adds a level of complexity to the game that can be easily
      achieved and as such will be adding this feature into my game.
    - Ability to block player's movement: <br/>
      Players can stand in the way of other players preventing them from moving forward, this can be
      annoying, but it also enables players to have more interaction with each other allowing them
      to have fun annoying each other as well as trying to reach the exit. I will be implementing
      this feature as it adds a new aspect of fun to the game.
- Portal 2
    - Different players are locked into different sections: <br/>
      This forces the players to tasks specific to the player and also allows for the ability to
      force player to move objects between the different sections of the game. I using this in my
      game as it adds new ways to make levels, and can be very helpful to teach players how they
      need to think to work together.
    - Player launcher to force good timing on players: <br/>
      Players step onto a tile on the floor that launches them into the air, this is used to move
      objects as well as used in combination with item spawners forcing the players to have to catch
      the items mid air in order to use the. I will not be adding this into my game as I feel that
      it would be too complex to add and also in a 2D environment it would limit the level design as
      the level would have to be very open.
    - Lots of different ways to achieve the same goal: <br/>
      While there is one intended route to the exit there often is one or more alternative routes
      that can be taken to reach the exit, this pushes the players towards a specific route, whilst
      still allowing them some freedom to complete the level how they like.
    - Many different objects (Laser redirection, cubes, turrets, etc.)
    - Objects needed to complete the level also hinder the player(s)
    - Ability to kill/trap other players (to increase enjoyment)
    - Narrator trying to pose the players against one another
    - Gestures/emotes to enable people to communicate non-verbally

### A4 - Essential Features

- Ability to play with other people: A team building game can't help to strengthen teamwork in a
  team of one person.
- Platforms for the players to traverse: Allows for additional control over what the can and can't
  do forcing them to work as a team.
- Intractable objects to open doors and close bridges: Adds more room for puzzle problems.

### A5 - Limitations of Proposed Solution

The game will not include the ability to change key bindings or to use a game controller as input,
this is due to the unnecessary complexity of adding these features. However, due to the modular
nature of the code it should be possible to add these with existing infrastructure.

The game will also only use simple graphics as graphics are not the focus of this game and would
take a long time to make.

### A6 - Requirements

The game will have no software requirements for the end user, and the only hardware requirement for
the end user will be that they must have a graphics card that supports the OpenGL rendering API most
graphics cards should support it, but it could still cause problems for some. This is to prevent the
need to spend time implementing unnecessary graphics APIs while still allowing as many people as
possible to play the game.

To develop the game I will be Using NeoVim because of the extreme levels of customisation that can
be achieved, and I will be using JetBrains' CLion on any computer where I do not have a NeoVim
setup.

I will be using Git for version control and GitHub as a cloud storage for the project as they
integrate nicely together as well as with the two editors I am using.

### A7 - Success Criteria

1. Players can move left/right along the screen using 'S' and 'D' or the left and right arrows, jump
   up to reach platforms using Space and climb onto the head of other players' characters to reach
   the room's exit - Justification: This simple movement allows for intuitive control over the
   player and also gives the player freedom to move around the levels easily. Allowing the player to
   jump ono each other enables for lots of unique paths to the exit to be made.
2. Players will collide with objects in the level that are used as platforms - Justification: This
   prevents the players from falling indefinitely as well as blocks off / opens up new paths to get
   the room's exit. For example an exit could be placed too far off the ground for the player to
   jump to, so a platform would be placed at a medium height to allow the player to use the platform
   to access the exit.
3. Players will be able to move through certain objects that are coloured to match their character -
   Justification: This enables certain paths to be blocked off / opened up to certain players
   separating the players to work on their own tasks to reach the exit. For example a button could
   be hidden behind a coloured wall to prevent one of the players from pressing the button forcing
   them to work on another part of the level.
4. Players can interact with levers and buttons (by pressing the 'E' key on the keyboard or
   'X'/square button on an XBox/Playstation controller) that are placed throughout the rooms to make
   certain platforms appear and doors open giving the players access to new areas that are vital to
   reaching the room's exit - Justification: this can be used to prevent players from solving the
   puzzle in an obvious and easy way (such as just jumping up to the exit), forcing the players to
   have to come up with a new plan to reach the exit. For example Using a button to reach a
   previously inaccessible areas of the screen platform that then allows the players to jump up to
   the exit.
5. Local multiplayer (2 players on the same computer) and Online Multiplayer (2 players playing
   across the internet) using UDP Tunnelling - Justification: this enables people to play the game
   on a single device if there is no access to two devices but also remotely without the use of
   applications like Steam Remote Play which requires a good internet connection for a playable
   experience, as players will want to play games together, so they can both work to figuring out
   how to get to the room's exit, or to enhance their teamwork skills. The online multiplayer will
   be implemented using UDP Tunneling which is a connection protocol that enables the users to have
   a direct connection between computers without the use of port forwarding, which normally requires
   skill to set up safely. The use of UDP Tunneling also means that only a rendezvous server needs
   to be created.
6. Players can access a settings menu to change the audio volume of the game, change which keys are
   used for input, e.g. the interaction input could be changed from 'E' to 'Q', change which
   language the game menu is in / use a dyslexic font, change whether the game is played in
   fullscreen or in 'windowed' mode - Justification: Players may find that they do not want to
   listen to the audio of the game or the audio is too loud and as such can change the volume of the
   audio. Players may find certain keys hard or uncomfortable to hit due to their specific keyboard
   layout, this would allow them to use a key that is more comfortable to them allowing more players
   to play the game. Players of a different nationality or players that are dyslexic to play the
   game more easily without having to learn English or struggle to read the text.
7. Players can unlock different characters to play as and change their character's sprite to match
   their play style, giving a sense of personality - Justification: This gives players a sense of
   progression as well as a goal to strive for while playing the game, many ga,es use achievements
   to encourage players to keep playing the game to collect all the achievements, characters will be
   similar to achievements encouraging players to continue playing the game.
