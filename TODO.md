high level: implement lighting approach described here:

- https://ncase.me/sight-and-light/
- https://www.redblobgames.com/articles/visibility/

--

- boundaries are lines, not rects
- more boundaries
- [..] raycasting, not hard coded
  - [..] unit test the math, for the various match cases
  - [..] visualize the math
- draw triangles for shadow and light
  - `draw_triangle(v1, v2, v3, color)`
- light influence fades with distance from source (see redblob demo)

GAMEPLAY

- implement asteroids
  - [ ] movement (momentum, wrap around)
  - [ ] floating asteroids
  - [ ] collisions (hit something and take damage + blow it up + invuln briefly)
- make it a game!
  - add collecting "warp crystals" to make a warp, which completes a level.
  - 3-5 levels of increasing difficulty. About 30s-1m each.
    - (# asteroids, size of asteroid).
    - future: could be lots of other things.
  - add game win condition once all levels complete.
  - add game lose condition, when you die in a given level.
- add juice! SFX, music, lights, particles
