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
