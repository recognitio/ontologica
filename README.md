
# ontologica

A small language to represent conceptual networks. Intended for configuring and developed in conjunction with the quizbowl-inspired study aid, [recognitio](https://github.com/recognitio/recognitio).

## Getting Started

Files written in ontologica should be suffixed with the extension `ontl`.

Concepts are represented syntactically with tags that may consist of upper case, lower case, numeric characters, and hyphens, and - in lieu of spaces - underscores; this is equivalent to `/^[a-zA-Z0-9_\-]+$/`

Ontologica can represent a variety of relationships between concepts.
```
hyponym <+ hypernym;
meronym << holonym;
token :+ type;
```
Relations can be combined for convenience.
```
concept
	<+ hypernym
	<< holonym
	:+ type
	;
```
Similarly, concepts can be related in bulk.
```
concept_1, concept_2
	<< holonym_1, holonym_2, holonym_3
```
