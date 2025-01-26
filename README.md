*Work in progress, not ready for general use*

Khutulun
========

An orchestrator for clouds.

Its goal is to provide an unopinionated platform for managing a diversity of compute and networking
technologies distributed across clusters of computers. And to make it easy to add support for new
technology trends as they emerge.

Out of the box we provide implementations for the following types of workloads:

* Bare processes: self-contained or installable executables and scripts (Khutulun *does not* force
  you to use containers)
* Containers and pods: via [Podman](https://podman.io/), [Docker](https://www.docker.com/), or
  [systemd-nspawn](https://www.freedesktop.org/software/systemd/man/systemd-nspawn.html)
* Pristine containers: via [Distrobox](https://distrobox.privatedns.org/) (on top of Podman or
  Docker)
* Virtualized containers: via [Kata Containers](https://katacontainers.io/)
* Virtual machines: via [libvirt](https://libvirt.org/)

For networking, we currently have:

* [OVN](https://www.ovn.org/)
* Straightforward TCP/UDP port reservation/discovery with [Firewalld](https://firewalld.org/)
  protection (simple is often best)

Bias
----

Khutulun rolls out the red carpet for two standards:

[Wasm](https://webassembly.org/) is our best-supported choice for implementing orchestration logic.
We love that Wasm is natively sandboxed and runs well on the many kinds of CPU architectures found
in clouds. One blob for everywhere. Even if you don't use Wasm for your own implementations, all
the built-in ones do and will "just work" in your clouds. Khutulun comes with an SDK to make it
easy to develop your own Wasm extensions in Rust, but any programming language that targets Wasm
would also work.

[TOSCA](https://www.oasis-open.org/committees/tosca/) is a YAML-based language for designing cloud
services. It's object-oriented, extensible, and has a packaging format, CSAR, which can bundle Wasm
implementation blobs together with the design specifications. Khutulun natively supports TOSCA and
CSAR via the [Puccini](https://puccini.cloud/) library, which is developed in tandem with Khutulun.
All our included examples are based on TOSCA, and we provide TOSCA profiles for all supported
workloads and networking types. Even if you don't choose to use TOSCA for your own workflow, you
will indirectly benefit from our efforts to support all of its powerful features. TOSCA is an OASIS
open standard, and the developers of Khutulun are longtime active participants in its evolution.

Design Choices
--------------

Khutulun is intentionally dual-paradigm, both data-driven (declarative) and event-driven
(imperative).

This crucial starting point is a deliberate reaction against fanatically declarative approaches,
which are often reductively opinionated and only clumsily extensible. Their automagical
orchestration pipelines can make some hard tasks very simple, but unfortunately can make the
simplest tasks hard. For example, you might be able to solve an orchestration challenge by
writing a straightforward script that tweaks a few configuration parameters, calls a few commands,
etc. But if your orchestrator is anti-code, then your experience is going to be anything but
straightforward.

If you're an architect, you will appreciate that Khutulun aims to get out of your way of you by
allowing you to inject code that realizes *your* opinion. Do you not agree with the design of our
built-in networking implementation? Then tweak it or roll your own. You provide the technology and
logic while Khutulun manages the boilerplate of coordination and delivery.

And if you are a sysadmin, you will appreciate that our built-in implementations embrace and
promote a culture by which the end result is something that you could have put together yourself.
We want you to be able to interact with individual cloud machines and programs using your everyday
tools, again without Khutulun getting in your way. In other words, our goal is comprehensibility,
because if experts can't make heads or tails of the cloud, then you'll be at the mercy of the
orchestrator. Khutulun can actually help your human work by deploying the systems that you need for
manual maintenance.

Another problem with fanatically declarative approaches it that they want the data to be your
"single source of truth". In reality, however, clouds are untameable, often deviating from your
intent as soon as you deploy your workloads. In other words, Day 1 is already Day 2. From changes
that happen due to hardware failures and network fluctuations, to self-healing and self-scaling
software, it's only where and what you deploy that is "true", *not* your intent. Thus Khutulun
avoids trying to achieve the impossible. It doesn't waste resources by continuously polling the
cloud in order to pull in data that would already be out-of-date. There is no endless
reconciliation loop. Instead, Khutulun provides tools to update and validate data only when you're
working on it, e.g. when responding to an event.

Clout
-----

Khutulun's data model is called Clout (short for "cloud topology").

Clout topologies are graphs of nested nodes (graph vertexes) with relationships (graph edges)
between them. Both nodes and relationships are first-class citizens with custom properties and
metadata.

Nodes can represent software or hardware components at any level from infrastructure to
application, or logical configurations that exist purely as data (and metadata). Relationships can
represent actual connections, such as network routes, ports, and secure channels, as well as
logical dependencies. If nodes are the bones and muscles, then relationships are the connective
tissue.

Additionally, Clout can represent templates for these topologies. Though you can design templates
directly in Clout, a higher level of abstraction is provided via Khutulun's native TOSCA support.
TOSCA service templates are compiled directly into Clout topology templates, which are designed to
support TOSCA's rich set of features.

Clout is not just a data schema. It also defines interfaces for code interactions. Code is used for
event handling, data retrieval and validation, and for topology template instantiation. The latter
is an orchestration scalability feature that allows generic templates to be "self-adaptive" to
their target clouds, e.g. optimizing for constrained edge sites, choosing components per hardware
vendor, etc. This feature is inspired by the [Nephio](https://nephio.org/) project.

Wasm is supported for all code implementations.

Clout is designed to be portable. It can be stored in in graph databases, as well as relational and
"no-SQL". It can be exported to and imported from YAML, JSON, CBOR, and MessagePack.

FAQ
---

### Why is it called "Khutulun"?

[Khutulun](https://en.wikipedia.org/wiki/Khutulun) (Mongolian: Хотулун) was a Mongolian
warrior princess and daughter of Kaidu Khan.

She was likely the inspiration for *Turandot*, the protagonist of Count Carlo Gozzi's
[*commedia dell'arte* play](https://en.wikipedia.org/wiki/turandot_(Gozzi)), which in turn
inspired [Giacomo Puccini](https://en.wikipedia.org/wiki/Giacomo_Puccini)'s
[opera of the same name](https://en.wikipedia.org/wiki/Turandot).

And [Puccini](https://puccini.cloud/) is Khutulun's TOSCA frontend.

See, it all makes perfect sense.

### How do I pronounce "Khutulun"?

3. International level: "KOO-too-loon"
2. Cosmopolitan level: "KHOO-too-loon" ("kh" like the "ch" in "Johann Sebastian Bach")
1. Expert level: "Хотулун" ([video](https://www.youtube.com/watch?v=uP0BagZ-ZCE&t=58s))

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT license](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
