# Analysis of miscellaneous games

## Floating crosshairs
Barotrauma, Helldivers 2 and War Thunder implement a 'floating crosshair.' This is comprised of two elements: the 'intent dot', and the 'aim dot' (I made these terms up). The intent dot is the direction you *want* to aim your weapon, which can be maneuvered freely, and the aim dot is where your weapon is actually aiming. Your aim dot tries to line up with the intent dot, but is constrained by factors such as traverse speed and objects in the world.

This floating crosshair system gives a sense of weight and heaviness to a weapon. This is demonstrated in War Thunder's tank barrels, and Helldivers 2's handheld weapons, as well as the HMG emplacement. Every single ranged weapon in Barotrauma also implements this system, including handheld firearms and ship cannons.

The speed at which the aim dot aligns with the intent dot is a good balancing opportunity. If a weapon is too maneuverable for its damage, it can be made slower to turn. Though, this can also become annoying or frustrating easily, especially if your weapon takes 10 seconds to do a 180 degree turn while you're being charged by rabid alien wildlife from behind.

It's worth noting that Helldivers 2 and War Thunder's aim dot appear where your bullet will actually impact in 3D space, but Barotrauma's aim dot just indicates the direction your weapon is aiming, since the viewport limits your vision and your aim dot could easily exit the viewport. Barotrauma's intent dot also changes size to show how inaccurate your weapon would be at that range.

War Thunder also uses the crosshair to convey other information, especially in Arcade mode, showing things like reload status and whether or not your aim dot is on target. I think this is better than what Barotrauma does, which is show your ammo status at the bottom of your screen, forcing you to take your eyes off the fight to check.

I think that Helldivers 2 is the best reference for implementing a floating crosshair for character-held weapons, and War Thunder is the best reference for implementing a floating crosshair for ship cannons or other mounted weapons.

So, would implementing this system in Thalassophobia benefit the game? I don't think there's an immediate answer to this question, so the best solution right now is just try out different implementations.

## Balance meter
The game Intruder implements a 'balance meter', where your character can lose their balance and ragdoll if the meter is depleted, with depletion occurring based on environmental effects. For example, standing on a railing, or walking over a banana peel. Balance recovers naturally once the source of the depletion is removed.

The balance meter in Intruder is pretty silly, but it can easily be repurposed to be more serious. Balance can be depleted by the submarine suddenly decelerating, a grenade going off near you, firing a weapon with massive recoil, or other effects that would knock you off your feet. There could even be status effects that decrease your maximum balance. There's a lot to work with there.

It could also serve as a means to make players play more carefully and be more aware of their environments, like not running in a room that was recently flooded, though something like that would have to be communicated, such as making the floor appear wet with a puddle texture or something.

A consideration that can be made is also that the balance meter isn't shown directly as a meter. Maybe some visual effect like doubled vision near the edges of the screen could communicate it instead.

## Playing dead
Intruder offers another interesting feature: playing dead. The player can act as if they're dead to fool their enemies, and spring up and shoot them in the back at an opportune time. Though, since Intruder is exclusively player vs. player, the implications of how this would work on enemy AI isn't really clear.

Barotrauma also offers the ability to ragdoll on command, though I'm not sure if it actually makes enemies think you're dead, or it's just a ragdoll button.