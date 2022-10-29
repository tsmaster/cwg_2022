# TODO

- implement One Lone Coder (OLC)'s popup menu
- screens
  - BDG Logo
  - Title
  - Main Menu
  - About
- sounds
  - wilhelm scream
  - engine noise
  - tire squeal
- music
  - https://crates.io/crates/mod_player
  - http://www.retrospekt.com.au/2020/05/tiny-music-a-massive-curated-collection-of-music-in-mod-xm-s3m-other-formats/
- gamepad input
  - wasm-compatible crate?
  - per https://github.com/not-fl3/macroquad/issues/95 gilrs is not macroquad+wasm compatible
  - https://github.com/Bombfuse/gamepad (does not claim to support Linux)
    - guh, this uses gilrs. WHY
  - possibly use https://github.com/gamma-delta/quad-wasmnastics
  - possibly wrap my own https://macroquad.rs/articles/wasm/
- overmap large world
- city maps
- barrel scramble
  - arena
  - behavior tree AI
- racetrack
  - generated
  - neural net AI
  - context steering
- draw rotated sprites
  - see draw_texture_ex, which takes a DrawTextureParams struct

- copy CWG 2021 TODO doc here:
  - https://docs.google.com/document/d/1KzLhiYRT9y-VcntPWE6nFCW7YzGDtNtknmOXlOtz9Ok/edit?usp=sharing

# Done
- screens
  - BDG Logo (placeholder)
  - Title (placeholder)
  - Main Menu (placeholder)
  - About (placeholder)
- gamepad input
  - gilrs (implemented, removed, as does not work with WASM)
- mouse input
- https://crates.io/crates/macroquad
- https://crates.io/crates/macroquad-canvas
