# :herb: pToken EOS Action-Proof Maker

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
        eos_action_proof_maker generate (<JSON> | --file=<PATH>)

Options:

    --help              ❍ Show this message.

    generate            ❍ Command to generate a merkle-proof for the action in
                          the supplied JSON tying it to the `action_mroot` in
                          the EOS block in that same JSON. Returns an array of
                          hex-string digests forming the branch of the
                          merkle-tree.

    --file=<path>       ❍ Path to a file containg a JSON of an EOS block and all
                          its `action_receipts` & the action you desire a proof
                          for.

    <JSON>              ❍ A valid JSON string of an object containing keys:
                          `block`           ➔ The EOS block header.
                          `action`          ➔ The action you want a proof for.
                          `action_receipts` ➔ An array of all the action
                                              receipts pertaining to the block
                                              in question.

    <PATH>              ❍ Path to a file containing the valid <JSON> string
                          described above.

```

&nbsp;

***

&nbsp;

### :microscope: Example

If you go and look in the __`/example`__ directory you'll find some sample __EOS__ block __JSONs__ & shell scripts you can run to see how the tool works. Run the example via: __`❍ ./example-1.sh`__.

Output of example:

```

{
  "tx_id": "17e703438fd4b73a6e61c6ceca3f1dbd2482ed9235d71265c75657921dee0490",
  "block_id": "050d477498a107f74bc33eea74841fe4e6da1eabb474ba3d336d013f0eb27c64",
  "action_index": 5,
  "action_digest": "364afa1cc13bca5dce1027f089e56889171373f66f5e3e59637251aaaeac4caa",
  "action_proof": [
    "527d4fe0a20dd014ea28319ac759409b018d5160b38d8003fa4cf3aa0ff006b4",
    "601bc4e148641795a9586af5ec06ee16f46853de95471332fdf76345d026c27b",
    "cf332487474b0ab17cb6352a0a3b3c4219f1e8612278757133ca87d7327e9b3d",
    "0d42567e12f18f9075988dd2087df2ef2876de709b222b6a86b8ac026c75c48c",
    "7cc717a7e256683ab4d01c05040fc503f2436625f5ac9f639a2fd0b201231564"
  ],
  "serialized_action": "6002ca074f0569ae0000000048a592ba0190b3c858e590b1ca00000000a8ed32323c90b3c858e590b1ca50c3000000000000085042544300000023324e3238545a684c586468566546764e33706359464667744776686a37575574507737",
  "action_json": {
    "name": "redeem",
    "account": "ptokensbtc1a",
    "data": {
      "memo": "2N28TZhLXdhVeFvN3pcYFFgtGvhj7WUtPw7",
      "quantity": "0.00050000 PBTC",
      "sender": "test1test2tt"
    },
    "hex_data": "90b3c858e590b1ca50c3000000000000085042544300000023324e3238545a684c586468566546764e33706359464667744776686a37575574507737",
    "authorization": [
      {
        "actor": "test1test2tt",
        "permission": "active"
      }
    ]
  },
  "action_receipt_digest": "527d4fe0a20dd014ea28319ac759409b018d5160b38d8003fa4cf3aa0ff006b4",
  "serialized_action_receipt": "6002ca074f0569ae364afa1cc13bca5dce1027f089e56889171373f66f5e3e59637251aaaeac4caacbb8d822000000005a000000000000000190b3c858e590b1ca82000000000000000102",
  "action_receipt_json": {
    "tx_id": "17e703438fd4b73a6e61c6ceca3f1dbd2482ed9235d71265c75657921dee0490",
    "receiver": "test1test2tt",
    "act_digest": "d335c1e9ceb8abe0e58ca33284f48028ebbb3f3d141edb13a7add39383f90bde",
    "global_sequence": 584628426,
    "recv_sequence": 56,
    "auth_sequence": [
      [
        "eosdtpbtcpos",
        67
      ]
    ],
    "code_sequence": 1,
    "abi_sequence": 2
  }
}


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

 - The tool __DOES__ validate that the supplied __`actions`__ all merkle together to create the __`action_mroot`__ in the block supplied. If this validation step does not pass, the proof will not be generated and instead the tool will exit with code __`1`__ and print to __`stderr`__:

```

✘ Error validating action receipts!
✘ Action receipt merkle root does NOT match `action_mroot`!

```

 - If you supply and action whose digest is not found amongst the supplied __`action_receipts`__, the tool will exit with code __`1`__ & print to __`stderr`__:

```

✘ Could not find action digest in action receipts!

```

 - A recent EOS fork enabled actions to return values, which changed the way actions were hashed. This tool calculates both ways in order to find the correct action receipt from the set passed to it.

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
- [x] Add more tests!
- [ ] Validate the block header too?
- [x] Validate the individual actions too?
- [x] Validate the action at the supplied index w/r/t to it's hex data
- [ ] Implement full action return value handling in the digest calcualtor
