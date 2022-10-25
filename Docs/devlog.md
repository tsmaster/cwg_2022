# Development Log

## October 25, 2022

One of the first pieces I often implement when I start on a small game
project is what I call my "Game Mode" pattern, which is tightly
integrated into the main game loop. I've noticed that there's a Rust
game-loop crate:

https://crates.io/crates/game-loop

which maybe I'll look into, but for now, I don't mind coding my own
loop.

Other people have talked about how to run fast, letting your
e.g. physics ticks go at high FPS, and only doing graphics updates at
a lower (e.g. 60 or 120) FPS.

So, maybe I'll implement that, or maybe I'll pull in game-loop.

But what I wanted to talk about was Game Modes, and the Game Mode
Manager that owns them.

### Game Mode

In my projects, a Game Mode is a self-contained piece of the game,
which may include:

- Publisher screen (Big Dice Games)
- Title screen (Cars With Guns)
- Menu screen
- About screen
- Credits screen
- Settings screen
- gameplay
- in-game settings screen
- quit verification

Each mode has an Update() and Draw() method. The Update() handles mode
logic and determines if a transition to another mode is necessary. If
so, it returns a GameModeTag, which is of an enum that lists all the
game modes. The Draw() method does all the drawing, minus a very
little bit of potential debug spew that might be in the main loop.


So, I just built this system up in Rust, which is working, but it's a
little weird, and I suspect there are ways to make it a little
cleaner. The GameModeManager has a get_current_mode() method, which
borrows a mode object, which is a mutable reference to a dynamic trait
object, which seems not super complex. On that object, I call Update()
and Draw(), which works, but Update() needs to take &mut self, while
Draw() only needs to take &self, but what I'm seeing is that since the
main loop borrows a mutable reference to the GameModeManager while
calling Update(), then also Draw() needs to borrow a mutable
reference. Perhaps I can play with lifetimes or some such to clean
that up.


A few features that don't exist, but probably should:

### Preloading

Loading assets, like PNGs for backgrounds, takes some time,
potentially (typically?) more than a frame, so it's worth having a
loading screen that waits for assets to be loaded. I had poked a
little bit at having an asset manager, and then have individual modes
provide a manifest of what needs to be loaded, and then the Asset
Manager would ensure that anything that wasn't already in memory would
get loaded.

To my recollection, the issue that I ran into was that I wanted the
Asset Manager to "own" the assets, and the GameModes to keep some sort
of pointer to them for efficiency. Maybe I can re-think my way through
that problem, maybe using RefCell or RC or other fancy smart pointers,
which I know little about.

A slightly different approach is to let each GameMode manage its own
assets. This is potentially wasteful, in that I wouldn't make any
effort to avoid duplicated loads. On the flip side, when a game mode
would go away, all of its assets would also go away. This, however,
isn't that good, because game modes last the life of the app.

If game modes need to load their assets, it probably makes sense to
have some mode for loading, and/or some way to ask if a mode's assets
are loaded.

One way this might work: main() loads the assets for the BDG screen,
and does not start the main loop until BDG is ready. Then, inside BDG,
the CWG assets are loaded. Transitioning from BDG -> CWG would not
proceed before the CWG assets are loaded. (This should be fine, as
typically, one of these load screens is up for at least a second,
longer if the publisher demands, and loading a fullscreen PNG takes
much less time than that.)

I could also have loading screen, say between menu and gameplay.

