Distrand
========

[![Build Status](https://travis-ci.org/tecywiz121/distrand.svg?branch=master)](https://travis-ci.org/tecywiz121/distrand)

Distrand is a library for disttributed random value generation.

It uses a simple commit-reveal algorithm that is suitable for small numbers
of participants. Each participant must communicate with every other
participant, so the number of messages increases dramatically as the number
of participants increases.

See the `examples` directory for a tutorial.

## License

Licensed under the [Mozilla Public License, Version 2.0](LICENSE.md).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the Covered Software by you, as defined in the Mozilla Public
License, shall be licensed as above, without any additional terms or conditions.
