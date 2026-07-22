# Proposal

The current gameplay cue is a 42 px-high line of 21 px text such as
`JUMP / LEFT`. At AR playing distance it asks the player to read two English
tokens while moving, and color is the only distinction between hazard types.
The lane token can also be mistaken for a movement direction even though it
describes the obstacle lane.

Replace it with a shape-first action beacon rendered by Macroquad. A unique
silhouette carries the action, a three-slot mini-track carries obstacle location,
and a progress rail carries approach timing. Text remains a redundant label,
not the sole information channel.
