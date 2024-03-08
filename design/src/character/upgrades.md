# Genetics and Implants
Genetics and implants are a system used to upgrade your character. Unlike equipment, genetic modifications and implants are persistent and will stay present even in the event of death. They're not strictly 'permanent' and can be removed after application, though this can come at a cost.

## Genetics
Genetic modifications are character upgrades that provide **passive stat buffs** - for example, faster run speed, more health, etc. This provides a way to gradually and organically specialise into a role, like increasing your movement speed for scouting, increasing your health for doing security tasks, etc.

Genetic modifications use a linear technology tree pattern similar to [submarine upgrades][sub-upgrades]. Rather than credits, genetic modifications are unlocked with special points. The crew can purchase these points at high-tier settlements, increasing a global tally. Individual layers can then spend any amount of points up to the global tally, and doing this does not subtract from it. This lets players jumping in mid-campaign instantly get up to speed with the rest of the crew.

You can disable genetic modifications, refunding the points and letting you allocate them elsewhere. This [encourages experimentation][creativity-tenet] and sillier builds, because you can always go back if it doesn't work. **TODO: Restrictions on refunding to prevent changing on the fly.**

### Genemod idea list
This is a list of genetic modification trees that could potentially be added to the game. WIP, subject to change.

- Resilience
    - Increases maximum vitality by a percentage
    - Reduces the severity of [pain shock]
    - Useful for all crew but especially combatants
- Metabolism
    - Increases the speed at which wounds heal naturally
    - Increases the speed at which chemicals apply their effects
- Steadiness
    - Reduces incoming [balance depletion][balance] by a percentage
    - Increases the maximum balance of the characters
    - Useful for mechanics, captains, and combatants
- Agility
    - Improves movement speed without any load
    - Improves attack speed with light melee weapons
    - Increases the speed of actions like vaulting
    - Useful for scouts and mechanics
- Strength
    - Improves movement speed when under load
    - Improves attack speed with heavy melee weapons
    - Improves damage with melee weapons
    - Useful for salvagers, cargo techs, and ship gun operators
- Fine Motor Control
    - Increases interaction speed for various tasks
    - Improves accuracy with ranged weapons
    - Useful for armed combatants
- Optimised Cell Respiration
    - Decreases oxygen consumption
    - Critical state suffocation is slower
    - Useful for frequent divers like salvagers
- Neural Adaptations
    - Increases the amount of implant points available

## Implants
Implants are character upgrades that provide **active abilities** - for example, a toggleable flashlight built into your body, an AR display that highlights enemies, etc. Implant abilities are very strong and grant further progression to characters specialising into a role, allowing you to 'double down' on your current build and make it that much better.

Characters can have multiple implants, but each implant takes a certain amount of "implant points" (name pending) to install. A character only has so many of these points that can be used, and the available amount can be increased with certain genetic modifications. This is a balancing measure so you cannot have literally every implant in the game installed into you at the same time. Characters may not install the same implant more than once.

These abilities are intentionally extremely strong, and they are correspondingly expensive. They are bought from traders or fabricated using blueprints. Implants are not lost or destroyed on death, and appear back on the character when respawning, but also cannot be extracted from player corpses to prevent duping.

### Implant idea list
This is a list of implants that could potentially be added to the game. WIP, subject to change, you get the idea. Some of these may not be acceptable in their current condition and need a revision of their design.

- Inhibitory Circuit Override
    - Uses 4 implant points
    - Actively triggered ability, stops after 30 seconds.
    - Can be activated again to stop the effect early.
    - If the total time spent active is over 30 seconds:
        - Multiplies 'damage points' being applied by the additional time, times some factor to make it smaller
        - Passively increases damage points when the ability is active, affected by the above multiplier.
        - The counter for time spent active decreases by 0.1 * the number of seconds passed.
        - This means that at most, you can spend 1/10 of the time with the implant active before taking additional damage.
    - While active, it:
        - Negates the effects of pain shock
        - Increases the threshold for going into critical condition
        - Dramatically increases movement speed, melee attack speed, and melee attack damage
        - Builds up 'damage points' when the character moves or attacks
        - Continually applies a portion of the built-up damage points as [internal bleeding]
    - When it ends:
        - Applies all remaining un-applied damage points as internal bleeding over 3 seconds
        - Applies the sum of all collected damage points as internal bleeding, multiplied by 0.3x, over 5 seconds
            - Yes, this applies the same damage more than once, just multiplied to be less.

<!-- Links used in the page -->
[balance]: ./stunning.md#balance
[creativity-tenet]: ../design/tenets/creativity.md
[respawn-time]: ./respawning.md#respawn-time
[sub-upgrades]: ../submarine/upgrades.md
[pain shock]: ./statuses.md#pain-shock
[internal bleeding]: ./statuses.md#internal-bleeding