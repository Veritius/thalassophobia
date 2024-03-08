# Analysis: Barotrauma

## Progression
The progression of Barotrauma is probably the part of the game that I scrutinise the most. Barotrauma's map is laid out in a linear fashion, where going deeper towards the heart of Europa increases the dangers you face, as well as the rewards... or at least that's how it's supposed to work.

Here's the first problem: You always start in the Cold Caverns. Now, this is great for beginners, but this is such an annoying slog for experienced players starting a new campaign, because it poses next to no challenge. Almost every travel node in this region has a settlement on it, so you're never gonna end up starved for resources, and all the enemies that appear are just squishy creatures that are easily dispatched by a competent gunner.

### Submarine Progression
Submarine progression is fine, actually. The 'tier' system, while completely arbitrary, does effectively motivate you to go deeper if you want a better submarine. Although, you often go a really long time between changing submarines, and it's totally doable to defeat the final boss while manning the something from the second tier, if you try hard enough. The Typhon contains a medical fabricator, arguably the most important component to survive the late game.

### Character Progression
The player characters have some really weird progression. You can make yourself an immortal super-soldier without even leaving the starting biome. The main thing you upgrade with your characters is just equipment and [talents](#talents). Except, all the equipment in the game is crafted with resources that can be very easily obtained through purchasing them at shops or going on mining missions, and the talent tree unlocks various buffs that make you even stronger. The "rare" material Physicorium is supposed to make it so you must go deeper to progress and get the endgame items... but you can also find it in the Cold Caverns, with a bit of luck.

### The Endgame
There's nothing to do in the endgame. Seriously. You can reach your maximum possible strength by the third biome out of five. Also, it's more of a challenge to try and exterminate all human civilisation in the lower layers than it is to kill the final boss (which is also a very underwhelming thing to do).

By the point my crew is supposed to have decided to plumb the depths and reach the heart of Europa, we end up just abandoning the campaign. And we mod our game, too, so there's loads more content to toy around with, but the problem persists.

### Jovian Radiation
Jovian Radiation is a band-aid solution to the problems with Barotrauma's progression that I described earlier. It's supposed to push you deeper, but you can also counteract it by just wearing the suits nuclear engineers get, and injecting antirads occasionally. It deals with the progression problem with the stick ("fuck you, go deeper or you die") rather than the carrot ("there are greater treasures in the depths").

## Health
The health system in Barotrauma is simple but has immense room for complexity.

### Vitality
Most games use a simple health bar to store how wounded the player is. This is a well-entrenched design used across all of video games, first appearing decades ago. And it works great for almost all applications. It's simple and intuitive - get hurt, the bar goes down, and if the bar goes down too much, you lose. This system even appears in modern action RPGs with otherwise deep and complex mechanics.

Barotrauma takes a different approach, storing damage like status effects, as well as tracking their severity. If you're shot, you gain the "Bullet wounds" affliction, or the affliction becomes worse if you already have it. Afflictions model complex injuries in a digestible and intuitive way.

Of course, this is a health system, so you have to be able to heal. Different medical supplies can treat different afflictions, such as bandages treating bleeding and lacerations.

Vitality is your overall health, and is the sum total of offsets created by afflictions. It's displayed similarly to a traditional health bar, with some nuance based on the [player state](#state-machine). This lets you judge your current situation at a glance without having to analyze all your wounds and sum them up, but an additional health menu accessible at any time shows your afflictions in detail.

With that out of the way - I love this system. It's complex and simple at the same time, in just the right way. It models complex injuries but doesn't sacrifice the simplicity and intuitive nature of the health bar. It also makes being a medic fun, it rewards game knowledge and skill, and works in both fast paced, frantic situations, and downtime and calm periods.

Though, I think the vitality system is *too* simple. Once you treat a wound, you've treated it. Job done. It could be more compelling to be able to easily, quickly, and cheaply treat a wound, restoring vitality temporarily, until the treatment dissipates and you have to do something more permanent. Or you can keep appling temporary fixes forever, which will eventually become more expensive than a permanent solution. This could lead to some great gameplay, like running out of medicine on a long voyage, and desperately trying to keep the crew intact long enough to reach a station and restock.

Additionally, opioids in Barotrauma are weirdly designed. They're a permanent treatment to injuries. Bite wounds? Morphine. Gunshot wounds? Have some morphine. Took a grenade to the chest? Have some fentanyl (which is just Better Morphine in Barotrauma). Ignoring the hilarious misrepresentations of what these products do, it's also just really unintuitive. Opioids in real life are pain medicines, they don't make wounds magically stitch themselves back together. Opioids would be far more intuitive if they existed to serve the purpose in the above paragraph, restoring vitality temporarily even if the body is *severely* damaged.

### State machine
The player operates in a state machine based on vitality, with three states: conscious, critical, and dead. The state transitions are based on the vitality of the patient.

The 'conscious' state is the usual. You're alive, talking, breathing fine, etc. You can do your job fine. You can still suffer from status effects while in this state, like slowdowns or stuns, but you're still functional.

With a patient in the conscious state, a medic can operate proactively, supplying buffs to combatants when needed, and reactively, healing injuries when they occur. Conscious patients can also self-administer medicine, which can lighten the load on the medics.

The 'critical' state occurs when you take too much damage. In this state, you are immobilised, unable to talk, use your inventory, and will slowly suffocate. This is a deviation from the norm - most games you simply fall over dead when you take too much damage. I think this state is the most noteworthy, namely because of its gameplay implications.

The critical state lets you be saved even in a dire emergency. If you're gunned down in a firefight, or taken by surprise, you're dying, but you're not dead. Good communication in your team will let your medic know you need help, and they can nurse you back to health. This also punishes being stupid, like overextending, or running off somewhere without telling anyone, because then no medic will come to save you.

With a patient in the critical state, a medic must operate purely reactively, analysing your injuries in a split second and applying the correct medicine to heal you. This also rewards efficiency, since medics can only carry a limited amount of medicine on them, so using the wrong medicine would simply waste it, potentially depriving someone else who may need it. The medic might even opt to give you just the bare minimum to restore you to consciousness, and move on to a different patient.

The 'dead' state is exactly what it says on the tin. You take too much damage, move out of the critical state, and you are now finally dead. You're a spectator now. Once you enter this state, you can't exit it, you just have to wait to respawn.

### Medical Gameplay
Playing as a medic in Barotrauma is fun. That's not something I can say about most games with a healer role or even just cooperative healing mechanics. And I think that's because of two factors: triage, and an active role in the fight.

Healing in Barotrauma is a very active task. You are often in the front line of a fight, patching up conscious soldiers, but also retrieving and healing critical ones. This is your role in the fight: keeping those with guns alive, and subsequently keeping the fight going. And often, retrieving a wounded person is a dangerous task, and can put you directly in danger. Balancing your safety with the survival of your patients is a crucial part of the next section.

Triage is a critical part of working as a medic in Barotrauma, and is entirely dependent on player skill. You must constantly decide how to spend your resources, one of which is simply your attention - you can't be everywhere at once! How are the resources available to you best spent to ensure as many people as possible survive? Who ought to survive, with the limited resources you have? These questions must be constantly and subconsciously answered based on the current environment. I call this the triage loop, and it is incredibly engaging. It can be just as engaging, or even more so than being out there in the front lines shooting your enemies.

In the event of mass casualties, medics must triage patients. Who's a priority for saving, the security officer gunned down in the station assault, or the engineer who tried to be a hero and was subsequently pulverised by several guards camping a staircase?

## Jobs and Skills
Barotrauma lets you pick a 'job' when creating a character. This is almost completely irrelevant past a few hours of gameplay, and only affects what talents you can choose, since available talents are determined by your job. Every single campaign of Barotrauma I've played has ended with every crewmember being a generalist with skills in every department, with their specialties determined by the equipment they have.

The thing is, each character has 'skills', and picking a job will start you with different levels of skills. Captains are better at piloting, security officers are better at shooting, etc. But you can also increase your skills just by doing that thing, making it so the only thing that limits what you can do is just the player's skill level. With a competent, experienced crew, it's not just engineers and mechanics repairing, it's the entire crew chipping in when they can.

Also, Engineers and Mechanics have a massive overlap in their tasks. Both of them are intended to repair things, but they repair different subsystems of the ship. Engineers repair the electrical system and maintain the reactor, and Mechanics maintain the engines, pumps, and hull. But you can also just carry the tools to do all of these tasks just by fabricating them. The only downside is that you might occasionally get injured for doing something you don't have the skills for. Doing this doesn't even hurt you, it hurts the medical department, because they're the ones who have to fix you. And if you keep doing it, you don't even get hurt anymore, since your skills improve.

Submarines can also designate 'access levels' based on the ID card you spawn with as part of your job. Which is also completely ignorable because you can just ask people to open the door to the armory. My crew does this frequently - even the lowliest engineer is strapped. It's just inconvenient. Sometimes I even modify submarines to remove these restrictions.

As far as I can tell, jobs really only have roleplay value. The clothing sets help visually distinguish characters (though eventually everyone's wearing a bright red dive suit anyway), and it's good to have some kind of chain of command, but still. It doesn't provide any mechanical changes beyond those that cause mild inconveniences. I guess it helps against griefing, but that's not something I like to design around.

### Talents
The talent trees range from a little annoying to completely off the rails bizarre. The Security Officer tree is pretty reasonable, you mostly just unlock damage resistances and buffs. The Medical Officer tree is insane - one of the endgame talents completely prevents your allies from dying as long as you're nearby. For a game with such a detailed damage and healing system, this is incredibly out of place.

Across all jobs, talent trees can unlock special crafting recipes. Earlier in the trees, you might get something like "better wrench" or "handheld PDA for something". Later in the trees, you can get a handheld particle accelerator, automated assault drone, a specialised safety dive suit, or a heavy machine gun. I have mixed feelings about this. Sometimes it feels interesting, and sometimes it feels really, really, *really* annoying. And when it feels annoying, you really notice.

Locking crafting recipes behind talent trees is not an enjoyable experience in multiplayer. When your ally has a recipe but you don't, you have to get them to go out of their way to click a single button, at which point they leave immediately, because that's all it takes to do crafting. Both of you are completely capable of clicking the button, but the game arbitrarily restricts you from doing it, which ultimately results in frustration.

Also, your capacity to access recipes changes based on the amount of players you have. The more players you have, the more diverse the talent trees you have access to, especially since mutually exclusive subtrees exist. If you have just 3 people, you have less access to parts of the game than you would with 12 people. And yes, technically, when you complete a subtree you can begin a new one, but with Barotrauma's experience system, that takes so agonisingly long I don't think I've ever actually completed two trees in the same campaign. Also note it's way harder to gain talent points when you die, which happens very frequently, even if you're great at the game, because you don't gain anything if you die.

And it's ultimately just limiting. Some really interesting combos are restricted because of talent trees. The Portable Fissile Accelerator pairs really well with the PUCS (especially if you load a volatile fulgurium fuel rod), but they're in separate trees, so unless you have 2 engineers, you just can't do that. This can be a balancing measure, but it's not a fun one, it's just frustrating.

## Atmosphere
The atmosphere of Barotrauma is overall pretty good. The story of being potentially the last of humanity on an alien world, trapped in a steel sarcophagus in waters swarming with hostile alien life, surrounded by unknowable, millennia-old alien ruins - it's fantastic. The game drips with tension, each sonar pulse could be your last, as you explore the ancient tunnels of ice below Europa's surface.

The artstyle of the game benefits this. It's grimy, it's rusty and unmaintained. The machinery you see is industrial and robust, built not for appearance but just to survive the stresses of Europa. The lights are dim, flickery, punctuating the darkness as humanity ekes out some kind of life in the depths, just trying to survive.

The game can also have some truly high moments, inspiring the deepest of fear into the crew. One time, a dive team containing my friend encountered a Latcher, an ancient abyss creature with a very long tongue that snares and envelops things. Unfortunately, my friend found this out in the most terrifying way possible, and was ensnared and killed instantly with a blood-curdling crunch, right in front of the rest of the dive team. That is the peak of Barotrauma, and it feels *amazing.* You can leave a round with a story you can tell again and again and again, without it ever getting old, and I think that's the best thing a game like this can achieve.

Unfortunately, Barotrauma's atmosphere is severely undermined by the "SS13 inspiration". By this, I mean the clowns. Consider this scenario: Your crew has just completed a mission to kill an abyss creature. You're worn out, you've suffered many casualties, and you desperately need a resupply. You dock to a station to try and repair your wounded ship and restock your drained supplies. You go to the local trader to buy materials and medical supplies, and you just barely have the funds - you can live another day, you're relieved. And on your way back you see four station-dwelling clowns beating a security officer with toy hammers while honking incessantly. This is not a one off occurrence, this happens frequently in Barotrauma, and it really hurts the tone of the game.

Also, the Clown subtree of the Assistant talent tree mechanically incentivises players to run around in a full clown suit. There's also the "goblin horde" traitor objective, where you paint the ballast tanks green and sit in it while roleplaying as a 'ballast goblin.' I don't think I need to explain why this is so detrimental to the otherwise very enjoyable tone of the game.

## Misc
### Wiring
Barotrauma also has a wiring system. You get to connect devices on your submarine together with wires to form complex logic. I think it's even Turing complete. Though, in practice, this doesn't really do much for the game. Some smart players have made custom, more efficient reactor controllers, but other than that, it's mostly just used for trolling and griefing. [This](https://youtu.be/MBiQc1rCzdk) is the most interesting thing I've ever done with wiring. Though, I'm mostly talking about wiring that carries information - power wiring is actually a pretty good game system.

### Traitors
In campaign mode, traitors suck. The original traitors mode was bad, and the new one is slightly better but still bad. 

In terms of the objectives, you can get missions like "fabricate a rubber duck made of lead" which is instantly recognisable to any experienced player as a traitor objective, and you get found out immediately and maybe thrown out the airlock. And yes, you also can get missions that are interesting, like having to assassinate a VIP or something.

But overall, these missions are antagonistic to the core purpose of the game. Barotrauma is a cooperative game, and suddenly turning people against eachother isn't really fun, especially not for the traitor.

However, if you're running gimmicky single mission lobbies without any kind of persistence, I suppose this is fine. I don't know why it's integrated into the game, though, it'd make more sense as a LuaForBarotrauma addon or something. Since Thalassophobia is primarily campaign-oriented, traitors are not a good addition to the game.

### Ragdolls
In the words of a certain striped-hat-wearing man, "funny ragdoll." Ragdolls in Barotrauma are replicated over the network and can absorb bullets. This is a feature that's easy to overlook, but is pretty significant. It makes combat feel messier, and increases the skill ceiling for a gunner. Sure, you could just shoot the corpse until it stops being a problem, but you can also hit your target when it dips out from behind the corpse. This is especially prominent in submarine combat, where the corpses of dead creatures stay around a lot longer because they float.

In future, since VR is intended to be a supported platform for the game, this behavior will be especially useful for emergent gameplay. A VR player could pick up a corpse and use it as a very unethical meatshield. This could also be implemented for other players using a button prompt, but I think that takes away from the magic of discovery.

## Guns
The guns don't hit hard enough. If you shoot someone in the head with a .61 caliber bullet, it *doesn't kill them!* They can easily survive, and if they're wearing armor, they won't even go into critical condition. You can literally empty a SMG mag into someone's chest, and they'll probably survive if you treat their wounds. Guns in Barotrauma feel like you're tickling the target to death, not shooting them with high velocity pieces of metal propelled by explosions.

### Handguns
I don't think I've ever used one of the vanilla handguns to actually defeat an enemy. If you're gonna get in a fight, you're not using your other hand, so you might as well use a two handed weapon like a shotgun or assault rifle, which has way better time-to-kill. It's not like you have to store shotguns differently to pistols, either - you put it in your hotbar just the same.

Then again, captains get an enormous buff of 30% when using revolvers in a specific talent tree, along with the recipe for an Even Bigger Iron(tm) than the default revolver. I've already talked about why Barotrauma's talent trees aren't good, but hey, it's something at least.

## Death and respawning
Death in Barotrauma isn't that bad. What *is* bad is what happens to your stuff when you die. When you respawn, you start with the default job loadout, so it's up to the rest of your crew to recover your stuff from your body. This would be fine, except for disconnected players. When you disconnect, your body ragdolls and eventually turns into a bodybag. The problem with that is both are physics objects, so if the submarine's hull is breached, all your stuff can fall into the abyss and be lost forever. Also, anyone can loot either of those things, so disconnecting as a captain or security officer is especially dangerous.

Once you respawn, you can either respawn on a shuttle or directly on the submarine. Both of these options can be (and often are) a miserable experience for the player. The respawn shuttle can be attacked by random monsters before you get anywhere close to the submarine, and you can spawn in flooded compartments and die instantly from pressure. Then you have to go find your stuff, which is either on the sea floor or some compartment in the submarine.

My crew basically resorts to exploits to get this system to be somewhat more fun. We disconnect right before docking, because after the load screen bodies are removed, making it so when you reconnect you reconnect with all your stuff.