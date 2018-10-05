extern crate distrand;
extern crate hmac;
extern crate rand;
extern crate sha2;

use distrand::{Exchange, Secret};

use hmac::Hmac;

use sha2::Sha512;

fn main() {
    // The first step is creating a source for randomness.
    let mut rng = rand::thread_rng();

    // Next, we choose our contribution to the random number. Generally speaking
    // this should be generated randomly as well, but for this example we're
    // using a constant: 5.
    let secret: Secret<u64, Hmac<Sha512>> = Secret::new(&mut rng, 5);

    // After creating a secret, we create a "commit" for it. A commit guarantees
    // that Alice, an evil participant, cannot change her secret value to
    // influence the output once she has knowledge of other participants'
    // choices.
    let commit = secret.commit().unwrap();

    // At this point, in a real application, you'd distribute every
    // participant's commit to every other participant. In this example, there
    // is only one participant, so we just directly add it to the exchange.
    let mut exchange = Exchange::new();
    exchange.insert("myself", commit).unwrap();

    // Once the exchange of commits is complete, each participant reveals their
    // secret. Again, in a real application you'd have to distribute each secret
    // to each participant.
    let mut reveal = exchange.reveal().unwrap();
    reveal.insert("myself", secret).unwrap();

    // Finally, once all the secrets are inserted, you can get the random value.
    let value = reveal.get().unwrap();

    println!("Constant Value: {}", value);
}
