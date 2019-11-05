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

 - This tool does __DOES NOT__ validate the block header of the passed in block.

 - This tool __DOES NOT__ validate the action's individual fields with respect to that action's hex data. It simply extracts the latter for use in making the __`merkle-tree`__.

 - The tool __DOES__ validate that the supplied __`actions`__ all merkle together to create the __`action_mroot`__ in the block supplied. If this validation step does not pass, the proof will not be generated.
<!--
 - The tool __DOES__ validate that the action at the desired index serializes to the correct data that forms a leaf of the merkle tree. <!-- Well, it will do eventually!

 - The tool __DOSE__ validate that the action at the desired index serializes to the correct __`action_digest`__ in the relevant __`action_receipt`__.
-->

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
- [ ] Validate the individual actions too?
- [ ] Validate the action at the supplied index w/r/t to it's hex data!!
- [ ] Figure out how to parse the data in an action that's a contract creation in order to validate it!
