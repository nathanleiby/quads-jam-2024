high level: implement lighting approach described here:

- https://ncase.me/sight-and-light/
- https://www.redblobgames.com/articles/visibility/

--

- [x] add asteroids moving right to left
- [..] add collisions betweeen asteroids and player
- [ ] crash = game over
- spawn "warp crystals" + allow user to collect them
- collecting N "warp cystals" completes level
  - simpler alternative: more warp crystals then game moves faster
- draw triangles for shadow and light
  - `draw_triangle(v1, v2, v3, color)`
