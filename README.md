# :herb: EOS Action-Proof Maker

A tool for making __`merkle-proofs`__ for proving action retirement in an __`EOS`__ block!

&nbsp;

***

&nbsp;

### :point_right: Usage:

```

```

&nbsp;

***

&nbsp;

### :wrench: Build

To build this yourself, make sure you have __`Rust`__ installed then clone the dir:

__`❍ git clone https://github.com/gskapka/eos-action-proof-maker.git`__

Enter the dir:

__`❍ cd eos-action-proof-maker`__

And finally build via:

__`❍ cargo build`__

You'll find your binary in the __`./target/release`__ directory!

&nbsp;

***

&nbsp;

### :black_nib: Notes

&nbsp;

This tool does __NOT__ validate the block header of the passed in block! It only validates that the actions are committed to via the block's __`action_mroot`__ and then creates the requested proof!

***

&nbsp;

### :guardsman: Tests

To run the tests simply enter the dir and run:

__`❍ cargo test`__

&nbsp;

***

&nbsp;

### :black_nib: To Do:

- [ ] Add more tests!
- [ ] Correct the EOS primitives path in the json.
- [ ] Proof verifier? Though note that a proof simple hashed up the hash in the last position of the proof array.
- [ ] Validate the block header too?
