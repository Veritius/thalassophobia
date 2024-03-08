# Navigation
> Bevy's coordinate system is a [right-handed y-up] system. As such, X is left/right, Y is up/down, and Z is forward/back. Notably, forward is -Z. Bevy also uses quaternions, but for the sake of simplicity, this description will use [aircraft principle axes], being pitch (X), yaw (Y), and roll (Z).


## Navigation Terminal
The **navigation terminal** allows control of the submarine. It requires [power] to function and controls the functionality of other navigation components.

Navigation terminals can also operate automatically to reach an objective. However, they are much slower than manual control, incentivising a player to be on the helm and keeping track of things.

If multiple navigation terminals are present and being manned, the force being applied is the average of all terminals. Navigation terminals set to automatic, or unpowered terminals, are ignored.

## Engines and Ballasts
The **engines** allow control of position on the forward (Z) axis. It requires power to apply force, and takes input from the navigation terminal. If no input is provided (such as when the terminal is unpowered or unmanned) it will turn off.

The **ballasts** allow control of position on the vertical (Y) axis, and of rotation on the pitch (X) and roll (Z) axes. Ballasts do not require power to apply force, but instead require power to fill or drain the ballast tanks to change the amount of force applied. The amount of force ballasts apply is based on the size of the [compartments] they flood, and can be changed with [build mode]. Notably ballast pumps fill based on *volume*, not a percentage of the compartment, so larger compartments lead to slower changes in force.

The **drive thrusters** allow control of position on the left and right (X) axis, and of rotation on the yaw (Y) axis. They can also be added in [build mode] to enhance their effect on all rotational axes. The drive thrusters operate similarly to engines, requiring power to apply force and turning off without a signal.

## Minutiae
The rotation of the submarine is effectively locked. The further the submarine deviates from the normal rotation, the greater an invisible force will act to right it. The yaw axis is not affected by this, and can be at any angle. This prevents flipping yourself, which is never fun. Also, this prevents situations where it's impossible to fix the submarine's rotation, because it's at such a dramatic offset you can't reach any control terminals. Bad rotation also changes how the engines apply force, which makes navigation way harder until it's fixed.

<!-- Links used in the page -->
[right-handed y-up]: https://bevy-cheatbook.github.io/fundamentals/coords.html
[aircraft principle axes]: https://en.wikipedia.org/wiki/Aircraft_principal_axes
[compartments]: compartments.md
[build mode]: construction.md
[power]: electricity.md