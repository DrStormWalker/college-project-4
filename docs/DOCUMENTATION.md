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

- [ ] Basic Rendering
- [ ] Collisions
- [ ] Spikes
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

The Goal of this project is to create a fun multiplayer game that allows people to
work as a team to solve a variety of interesting and challenging puzzles.

### Why am I Making This Game?

I am making this game to encourage people to work as a team, resulting in people
enhancing their teamwork, social skills and ability to solve complex problems
that require the use of multiple people. Enhancing these skills can give people
a significant advantage in employment as all of these skills are used in almost
all fields.

## Analysis

### A1 - Computational Approach and Methods

A team building application such as a video game is suitable for a computational
approach as it allows for the game to provide great consequences for lack of
teamwork in turn encouraging the team to work better together.

The game will be built using reusable components to allow for easy addition of new
features and game mechanics. Another advantage of using reusable components is that
the code will be easier to maintain.

To streamline the use of reusable components an Entity Component System (ECS)
will be used. This will both help to overcome some design challenges related 
to the Rust programming allowing for the consistent use of memory safe code,
while also creating an easy method of interaction between components by being 
able to write queries to access all entities that match a certain criteria such
as colliding with a player.

### A2 - Stakeholders

| Stakeholder | Descriptions | Role | Stakeholder Needs / Requirements* How they will use the system |
|-------------|--------------|------|----------------------------------------------------------------|
|             |              |      |                                                                |


### A3 - Research

### A4 - Essential Features

- Ability to play with other people: A team building game can't help to strengthen
  teamwork in a team of one person.
- Platforms for the players to traverse: Allows for additional control over what the
  can and can't do forcing them to work as a team.
- Intractable objects to open doors and close bridges: Adds more room for puzzle
  problems.

### A5 - Limitations of Proposed Solution

The game will not include the ability to change key bindings or to use a game controller
as input, this is due to the unnecessary complexity of adding these features. However,
due to the modular nature of the code it should be possible to add these with existing
infrastructure.

The game will also only use simple graphics as graphics are not the focus of this game
and would take a long time to make.
 
### A6 - Requirements

The game will have no software requirements for the end user, and the only hardware
requirement for the end user will be that they must have a graphics card that supports
the OpenGL rendering API most graphics cards should support it, but it could still cause
problems for some. This is to prevent the need to spend time implementing unnecessary
graphics APIs while still allowing as many people as possible to play the game.

To develop the game I will be Using NeoVim because of the extreme levels of
customisation that can be achieved, and I will be using JetBrains' CLion on any
computer where I do not have a NeoVim setup.

I will be using Git for version control and GitHub as a cloud storage for the project
as they integrate nicely together as well as with the two editors I am using.

### A7 - Success Criteria
