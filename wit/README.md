# WIT contracts

`lsharp:frame@0.1.0` is the draft public boundary between L# Components and the Rust L#frame host.

The initial contract deliberately uses event/effect batches and stable logical IDs. Native objects remain in Rust resource registries. A later revision may introduce WIT resources for selected stateful objects when ownership and async semantics are fixed.

The WIT files are design contracts in Milestone 0. Binding generation and parser validation are Milestone 1 work.
