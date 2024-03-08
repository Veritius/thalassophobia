# Power Systems
The power system is a [directed acyclic graph], where the nodes consist of **sources** supplying power, **junctions** routing and transforming power, **batteries** storing power, and **sinks** consuming power. Nodes in the graph can have as many connections between them as desired, as long as a loop is not constructed.

Electrical supply and demand is modelled based on the graph. Demand is calculated based on sinks and batteries, and sources will try to match the total demand of all sinks it supplies. Sources will try to even out the load between them, based on 'priority'. Sources with greater priority will supply more of the demand than other lower priority sources serving the same demand.

**TODO: Why this design.**

## Sources
A **source** is any device that supplies electricity. Most often, this is the nuclear reactor, but a different source can be used, or even more than one at a time. Since the graph is directed, sources can be attached at any point in the graph and will supply sinks below it. This means that if a room proves too power hungry, the crew can set up a small generator to locally supplement the power.

## Junctions
A **junction** is any device that can route and organise electricity. Technically, a submarine could be set up without junctions of any kind, but it'd make locally supplementing the supply for sinks impossible. Junctions can route into eachother, but cannot form loops of any kind.

## Batteries
A **battery** is any device that can take in, store, and dispense electricity, to meet demand. Like junctions, batteries can route into eachother, but cannot form loops. You can think of a battery as a junction with a buffer, where if supply outweighs demand, the buffer will fill itself. When demand outweighs supply, the buffer will drain to match it, while sources try to catch up. If supply and demand match, the battery will increase demand slightly to fill itself, at which point it simply acts as a junction.

## Sinks
A **sink** is any device that consumes the electricity. These nodes are at the end of the graph, and can be things like lights, engines, fabricators, etc. Sinks that suddenly fluctuate in power demands should have a battery inbetween it and the source.

<!-- Links used in the page -->
[reactor]: ./reactor.md
[directed acyclic graph]: https://en.wikipedia.org/wiki/Directed_acyclic_graph