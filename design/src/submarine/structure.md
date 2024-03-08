# Structure
The structure of the submarine is its 'skeleton', to which hull pieces are attached. It is a dynamically generated network-like arrangement, created based on the layout of [compartments] and any hull-mounted elements of the submarine.

The generation process also generates the [hull] pieces to form an outer shell around the structure, similar to a [convex hull]. This system exists to make [build mode] easier to use, since you don't have to define every single hull piece around the submarine.

Once the structure is generated, it is considered immutable. Even if a theoretical infinite-damage, infinite-range explosion occurred, the structure would remain intact. This is mainly for performance and simplicity reasons - a single static hull mesh can be used for large-scale physics calculations, and there doesn't need to be handling for innumerable weird edge cases. This also makes foamguns more usable, since you just aim between structural members.

<!-- Links used in the page -->
[compartments]: compartments.md
[convex hull]: https://en.wikipedia.org/wiki/Convex_hull
[hull]: hull.md
[build mode]: construction.md