<div align="center">
<h1><code>vidyut-prakriya</code></h1>
<p><i>A Paninian word generator</i></p>
</div>

`vidyut-prakriya` generates Sanskrit words with their prakriyās (derivations)
according to the rules of Paninian grammar and currently implements around
2,000 rules. Our long-term goal is to provide a complete implementation of the
Ashtadhyayi.

This [crate][crate] is under active development as part of the [Ambuda][ambuda]
project. If you enjoy our work and wish to contribute to it, please see the
[Contributing](#contributing) section below. We also encourage you to [join our
Discord server][discord], where you can meet other Sanskrit programmers and
enthusiasts.

- [Overview](#overview)
- [Usage](#usage)
- [Contributing](#contributing)
- [Data](#data)
- [Design](#design)

[crate]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
[ambuda]: https://ambuda.org
[discord]: https://discord.gg/7rGdTyWY7Z


Overview
--------

`vidyut-prakriya` has three distinguishing qualities:

1. *Fidelity*. We follow the rules of Paninian grammar as closely as possible.
   Each word we return can optionally include a prakriyā that lists each rule
   that was used as well as its result.

2. *Speed*. On my laptop (a 2.4GHz 8-core CPU with 64 GB of DDR4 RAM), this
   crate generates almost 50,000 words per second on a single thread. All else
   equal, a fast program is easier to run and test, which means that we can
   produce a larger word list at a higher standard of quality.

3. *Portability*. This crate compiles to fast native code and can be bound to
   most other progamming languages with a bit of effort. In particular, this
   crate can be compiled to WebAssembly, which means that it can run in a
   modern web browser.

`vidyut-prakriya` currently has strong support for basic verbs. For future plans,
see our [roadmap](#roadmap).


Usage
-----

To generate all basic tinantas in kartari prayoga, run:

```shell
$ make create_tinantas > output.csv
```

The first run of `make create_tinantas` will be slow since your machine must
first compile `vidyut-prakriya`. After this initial compilation step, however,
subsequent runs will be much faster, and `make create_tinantas` will likely
compile and complete within a few seconds.

To generate prakriyas programmatically, you can use the starter code below:

```rust
use vidyut_prakriya::Vyakarana;
use vidyut_prakriya::args::*;

let v = Vyakarana::new();

let dhatu = Dhatu::mula("BU", Gana::Bhvadi);
let args = Tinanta::builder()
    .dhatu(dhatu)
    .lakara(Lakara::Lat)
    .prayoga(Prayoga::Kartari)
    .purusha(Purusha::Prathama)
    .vacana(Vacana::Eka)
    .build().unwrap();

let prakriyas = v.derive_tinantas(&args);

for p in prakriyas {
    println!("{}", p.text());
    println!("---------------------------");
    for step in p.history() {
        let terms: Vec<_> = step.result().iter().map(|x| x.text()).filter(|x| !x.is_empty()).collect();
        let result = terms.join(" + ");
        println!("{:<10} | {}", step.rule().code(), result);
    }
    println!("---------------------------");
    println!("\n");
}
```

Output of the code above:

```text
Bavati
---------------------------
1.3.1      | BU
3.2.123    | BU + la~w
1.3.2      | BU + la~w
1.3.3      | BU + la~w
1.3.9      | BU + l
1.3.78     | BU + l
3.4.78     | BU + tip
1.3.3      | BU + tip
1.3.9      | BU + ti
3.4.113    | BU + ti
3.1.68     | BU + Sap + ti
1.3.3      | BU + Sap + ti
1.3.8      | BU + Sap + ti
1.3.9      | BU + a + ti
3.4.113    | BU + a + ti
7.3.84     | Bo + a + ti
1.4.14     | Bo + a + ti
6.1.78     | Bav + a + ti
8.4.68     | Bav + a + ti
---------------------------
```

The left column shows a simple string label for each rule that was applied
during the derivation, and you can find details about what values these labels
can take in the comments on the `Rule` type. We suggest using `ashtadhyayi.com`
to learn more about these rules.

The right column shows the in-progress prakriya. We use an output convention
that is common on other Ashtadhyayi websites. The encoding format for this
text is SLP1, which is the encoding format we use throughout the crate.

[sv]: https://github.com/drdhaval2785/SanskritVerb

For more details, see the following methods on the `Vyakarana` struct:

- `derive_tinantas` (for verbs)
- `derive_subantas` (for nominals)
- `derive_krdantas` (for verbal suffixes)
- `derive_taddhitantas` (for nominal suffixes)


Contributing
------------

`vidyut-prakriya` is an ambitious project, and your contributions can help it
grow.

### Reporting errors

The easiest way to help is to [file a GitHub issue][gh-issue] if you notice an
error. Please let us know what form you expected to see. We would also greatly
appreciate relevant citations from the grammatical literature so that we can
better understand and resolve the issue.

[gh-issue]: https://github.com/ambuda-org/vidyut-pada-snapshot/issues

### Modifying the code

First, see if you can run our existing code on your machine. We suggest
that you start by running our integration tests:

```shell
$ make create_test_files
$ make test_all
```

Next, try using our prakriya debugger, which shows exactly how a given word was
derived:

```shell
$ make debugger
```

Once you've confirmed that your setup works, we suggest that you read through
the documentation for `Term` (in the `term` module) and `Prakriya` (in the
`prakriya` module). Almost every part of the code touches these two structs.

To get familiar with our rules, we suggest that you skim through the
`ashtadhyayi` module, which defines our high-level API and wraps all of the
rules that we use in the system. We encourage you to read our extensive
comments and explore the smaller modules that we use within `ashtadhyayi`.

Now you're ready to make changes to the code. After you make your changes, run
`make test_all` to verify the impact of your code.

If you are satisfied with your changes, you will need to update our integration
test file. This process has three steps. First, run the steps below and confirm
that your tests fail:

```shell
$ make create_test_files
$ make test_all
```

`make test_all` should fail on a hash comparison error. Copy the new hash code,
replace the existing hash code in the `test_all` in our `Makefile` with that
copied value. Then, run `make test_all` again and confirm that all tests pass.


Data
----

This crate includes a Dhatupatha sourced from [ashtadhyayi.com][a-com],
and the author of ashtadhyayi.com has graciously agreed to share this file with
us under an MIT license.

For details on the lineage of this Dhatupatha, see our separate [data
README][data-readme].

[a-com]: https://ashtadhyayi.com
[data-readme]: data/README.md


Design
------

See `ARCHITECTURE.md` for details.
