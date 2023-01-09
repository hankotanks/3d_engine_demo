# 3d_engine_demo
A basic rendering engine with an adjustable 3D camera, a grid-based environment system, and sliding entities with (janky) collision physics. 

<img src="https://user-images.githubusercontent.com/17710458/211341703-3e483f02-15a1-451e-82a3-de8e5553f427.gif" alt="demo" width="550"/>

## Features
- World tiles that inherit from a single trait
    - Responsible for their own geometry
    - Extensible (although only cubes are implemented at this time)
    - Emissive lighting, projects from the surfaces of the tile
- Entities
    - Built on the same `Drawable` trait used for tile geometry
         - Can emit light as a result
    - Position is FP, unlike tiles
    - Position can be set frame-by-frame, unlike tiles
    - Subject to engine physics
- Physics
    - Adjustable gravity
    - 3D collision detection/resolution (along Tile edges)
    - Entities move through the application of force vectors
- Controller
    - Lateral movement via arrow keys
    - Emissive entities can be thrown by dragging and releasing the left mouse button
- Camera
    - Orbits around a central point
    - Can be assigned to an entity
    - Individual axis can be locked or restricted
- Lighting
    - Uses the Blinn-Phong model for simplicity
    - Color of emission and its intensity can be adjusted
    
## Limitations
- Primitive physics
- Geometry cannot be loaded from files
- Light data is passed to the GPU as a fixed-size array, which caps the number of lights in the scene
- Tile meshes are non-optimal. Adjacent tiles with continuous surfaces do not combine triangles

Although I initially had greater ambitions, this project was largely an excuse to play around with the matrix math that I was learning about in Linear Algebra at the time, and many things are (and will forever be) unfinished. 

Graphics programming was brand new to me when I started this project. In hindsight, the interface between the CPU & GPU is inefficient and generally terrible, as is the rendering process itself.
