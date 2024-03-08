# Creature AI
Creatures should feel like real animals, not automatons with the sole goal of killing you. A good way to approach this is to model the psychology of real animals, though obviously simplified, and behind a lens of trying to make an engaging video game experience.

Creature AI is based on the [Utility AI](https://en.wikipedia.org/wiki/Utility_system) model. An action is picked based on its 'utility', which is how beneficial it is deemed to be to the agent.

## Understanding
**TODO: This is a very complex and hard to implement system. Verify that it's worth it with more detailed documentation before implementing it.**

The understanding system works to adjust the utility of other actions. It stores an "understanding" of how player-controlled objects act. Analysis to adjust understanding is an action in of itself, and operates under a low priority.

When a creature identifies something it does not understand, if there are no other priorities, it will seek to analyse the target. If an analysis target does not change for an extended period of time, the agent will assume it understands all it can about the target, and the understanding action will lose utility.

When an agent identifies something it does understand, the understanding action will lose utility. Other responses will take over based on the agent's understanding of the target. If the agent understands the target to be a danger, the survival action is raised in utility, whereas identifying a food source will give utility to the hunting action.

Agents are generated with an understanding more aligned to developer-defined 'true' danger values of analysis targets the denser human civilisation is in the area they spawed. The understanding action is also present only in creatures that are medium-sized or bigger. Tracking the understanding of a very small crab does not affect the game, and just wastes computing resources.

## Survival
The survival action is actually multiple actions that act to preserve the agent's life in different ways. The utility of survival actions increase with how wounded the agent is, resulting in creatures attempting to escape

## Hunting
The hunting behavior directs the agent to kill and consume anything it identifies as a target. The hunting behavior's utility is mediated by how hungry a creature is (a value randomly generated when it spawns) and how difficult it is to carry out the hunting behavior.

## Wandering
The wandering action is the lowest priority action, and simply directs the agent to move more or less randomly.