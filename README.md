# :herb: EOS Action-Proof Maker

&nbsp;

A tool for making __`merkle-proofs`__ for proving action retirement in an __`EOS`__ block!

***

&nbsp;

### :biohazard: Important Caveat:

One key Dan Larrimer quote:

> Given an action, somewhere in the [EOS] blockchain, it is possible to succinctly prove the retirement of that action by first proving that it was committed to by a block’s Action Root, and then that the given block was committed to by a trusted irreversible block header’s Block Root.

...found in __[this article here](https://steemit.com/eos/@dan/inter-blockchain-communication-via-merkle-proofs-with-eos-io)__ and which article I've attempted to archive via the __[WayBackMachine](https://web.archive.org/web/20191112104752/https://steemit.com/eos/@dan/inter-blockchain-communication-via-merkle-proofs-with-eos-io)__ to no avail due presumably to something on Steemit's end...and so but however back on point the quote above is verbatim and points out the thorny issue that no EOS __`action`__ can be proven to have been __irreversibly__ retired _without_ trust. Tying the __`action`__'s retirement to an _irreversible_ block requires that that _irreversible_ block is a trusted one.

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

### :microscope: Example

If you go and look in the __`/example`__ directory you'll find a sample __EOS__ block __JSON__ & a shell script you can run to see how the tool works. Run it via: __`❍ ./example.sh`__. The built binary it's calling is in the __`/bin`__ folder and has a __`sha256sum`__ of __`6e9f0b50d847e4c6e9a65f04d8e0893b627c9bcc069ca2a4a736e7796716be5c`__.

Output of example:

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

__`❍ cargo build --release`__

You'll find your binary in the __`./target/release/`__ directory.

&nbsp;

***

&nbsp;

### :black_nib: Notes

 - This tool does __DOES NOT__ (yet) validate the block header of the passed in block.

 - This tool __DOES NOT__ (yet) validate the action's individual fields with respect to that action's hex data. It simply extracts the latter for use in making the __`merkle-tree`__.

 - The tool __DOES__ validate that the supplied __`actions`__ all merkle together to create the __`action_mroot`__ in the block supplied. If this validation step does not pass, the proof will not be generated and instead the tool will exit with code __`1`__ and return:

```
✘ Error validating action receipts!
✘ Action receipt merkle root does NOT match `action_mroot`!
```

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

- [x] Verify proof as last step before emitting proof.
- [ ] Add more tests!
- [ ] Correct the EOS primitives path in the __`Cargo.toml`__.
- [ ] Proof verifier CLI option? (Logic already in place for verification).
- [ ] Validate the block header too?
- [ ] Validate the individual actions too?
- [ ] Validate the action at the supplied index w/r/t to it's hex data!!
- [ ] Figure out how to parse the data in an action that's a contract creation in order to validate it!
