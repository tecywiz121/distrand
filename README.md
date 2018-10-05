Distrand
========

Distrand is a library for ***dist***tributed ***rand***om value generation.

It uses a simple commit-reveal algorithm that is suitable for small numbers
of participants. Each participant must communicate with every other
participant, so the number of messages increases dramatically as the number
of participants increases.

See the `examples` directory for a tutorial.
