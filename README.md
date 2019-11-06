# :herb: EOS Action-Proof Maker

&nbsp;

A tool for making __`merkle-proofs`__ for proving action retirement in an __`EOS`__ block!

&nbsp;

***

&nbsp;

### :point_right: Usage:

```
❍ EOS Action-Proof Maker ❍

    Copyright Greg Kapka 2019
    Questions: greg@oraclize.it

❍ Info ❍

A tool to make merkle-proofs over EOS actions in order to prove that action's
retirement in a given block.

❍ Usage ❍

Usage:
        eos_action_proof_maker [--help]
        eos_action_proof_maker generate (<JSON> | --file=<PATH>) <INDEX>

Options:

    --help              ❍ Show this message.

    generate            ❍ Command to generate a merkle-proof for the action at
                          the given INDEX in the supplied EOS block. Returns an
                          array of hex-string digests forming the branch of the
                          merkle-tree connecting the action-digest at INDEX
                          to the `action_mroot` of the supplied EOS block.

    --file=<path>       ❍ Path to a file containg a JSON of an EOS block and all
                          its actions in the format described above.

    <JSON>              ❍ A valid JSON string of an object containing keys:
                          `block`    ➔ The EOS block header.
                          `actions`  ➔ An array of all the actions retired in
                                       the block in question.
                          `receipts` ➔ An array of all the action receipts
                                       pertaining to the block in question.

    <PATH>              ❍ Path to a file containing the valid <JSON> string
                          described above.

    <INDEX>             ❍ An integer between 0 & the number of actions included
                          in the supplied block minus one.
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

__`❍ cargo build --release`__

You'll find your binary in the __`./target/release/`__ directory.

&nbsp;

***

&nbsp;

### :black_nib: Notes

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
