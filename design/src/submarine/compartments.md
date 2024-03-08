# Structure Compartments
Compartments in the submarine can be thought of as rooms. They define the habitable space of the submarine, and track flooding, oxygen, and other values, and are themselves defined as a closed polygon mesh. Each compartment has a computed finite volume (`v`) based on its shape, used in various game calculations.

Compartments are also used as [ballast], with their volume affecting the force they apply to the ship.

<!-- Links used in the page -->
[ballast]: navigation.md#engines-and-ballasts