# Cyberengine (and codename Cyberpunk)

So, here I am. Making a game engine. With no clear goal, and deadline that is way too close (well, I did set it myself). Why do I keep doing this. Anyway, I promise I __will__ finish this.


## Engine Architecture

Cyberengine will start as a simple engine, more like a wrapper layer around some libraries. Also, I shortened it to CE for now.


### CE::graphics

High-level graphics stuff. Wrapper around gfx-rs mostly.

### CE::window

Window related stuff. Creation, events. Probably glutin.

### CE::game

Top-level Game object stuff. AKA entry point.

### CE::state

State management. Main menu, "play state" stuff. Trait definitions, mostly. I think.

### CE::ecs

That's right, ECS. Data driven or go home. inb4 "ECS is just a term" skids.

### CE::resource

Stuffs that loadable. Local files, http, that sort of stuff.

### CE::network

Networking? We'll see.


## Stuff Relation

Game -> Window -> Screen -> State -> stuff
Game is general level stuff, like configurations, resources, etc
Window is, well, window stuff
Screen is rendering stuff. It is attached to a window, I think
State makes stuff happens (until I can figure out a better way to do this)