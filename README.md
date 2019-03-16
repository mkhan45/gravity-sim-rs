# nbody-rs
N body gravity sim with rust and ggez, made to learn rust

![](preview.gif)


## Controls

Arrow keys to move

Scroll to zoom in/out

Q/A to increase/decrease radius of next placed body

W/S to increase/decrease density (try making it negative)

E/D to increase/decrease trail length (removing trails increases performance by a lot)

Left click to place a body, dragging before releasing makes an initial velocity vector.

Right click over a body to delete it.

## Known Errors:

If you have a body with exactly 0 velocity it will crash. It's unlikely for this to happen because while velocities can be very close to zero, they're never near zero unless they've never been affected by gravity at all, i.e it pretty much only happens if you create a new body with 0 velocities and there are no other bodies.
