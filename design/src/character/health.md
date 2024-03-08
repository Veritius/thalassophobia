# Health
Health is the system governing wounding on player characters. More accurately, 'vitality' is a measure of how healthy the player character is, calculated based on the wounds the character has.

Vitality is decreased based on [status effects], with a negative status effect decreasing vitality corresponding to how severe it is. Changing vitality can trigger [health state changes](#states). Status effects can also be positive, boosting vitality.

## Body segments
Individual body segments track wounding and status effects, affecting overall vitality. While limb segments affect overall vitality, they can only affect it so much. Having your leg be destroyed cannot kill you. However, wounds on the torso or head (aka 'vital' segments) do not have such limits and can easily kill you.

Body segments also affect character stats. Wounded arms will reduce weapon accuracy, and wounded legs will reduce movement speed. Arms that are wounded enough to no longer be usable prevents most interactions, and legs that are in a similar condition prevent effective movement.

## Diagnosis
A player can see their own status effects (with some exceptions). Most of the time, a patient can simply tell the doctor what's wrong with them, though some status effects are not visible to the patient themselves, and so an outside observer is necessary.

However, if the patient is incapacitated or unable to communicate, outside observers will have to use a health scanner to view the player's status. Some wounds can also be judged visually based on the appearance of the patient, ie bleeding can stain clothes red.

### Health Scanners
A handheld device that shows a rough health estimate of whoever or whatever you point it at. The estimate becomes more accurate the longer the scanner is held on target, and will eventually begin to show individual [status effects].

A health scanner can also be used on a creature, but the initial estimate is extremely inaccurate, and accuracy improves much slower. This doesn't provide much practical benefit, but it's funny.

## Treatment
Individual status effects have attached treatment information. For example, burns can be treated with bandages. These treatments are permanent, and will make a wound less severe, or even remove it if the treatment is effective enough.

Treatment can also be temporary. If the appropriate treatment isn't available, generic treatments like painkillers or other combat drugs can reduce the vitality loss caused by the wound, but will eventually dissipate.

## States
Player characters follow a state machine based on vitality. They can be in three states, **conscious**, **critical**, and **dead**. A character with no wounds will be in the conscious state. Being wounded can move them to the critical state, or even the dead state, based on the vitality detriment from the wounds.

Conscious characters are fully functional. They can speak, interact with the world, use items, etc. This is the usual state of the character. You can still be wounded and conscious, but severe enough wounds will cause a transition to the critical state.

Critical characters are incapacitated, similar to being [full stunned][stunning]. They are ragdolled, and cannot speak or otherwise interact with the world. Critical characters will slowly [suffocate][suffocation], and without medical attention will eventually die.

Dead characters are dead. They will eventually [respawn][respawning] in a new body. A character in this state cannot exit this state through medical treatment.

<!-- Links used in the page -->
[respawning]: ./respawning.md
[status effects]: ./statuses.md
[stunning]: ./stunning.md
[suffocation]: ./statuses.md#suffocation
[health scanner]: medicaltools.md#health-scanner