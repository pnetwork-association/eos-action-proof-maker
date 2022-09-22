pub static USAGE_INFO: &str = "
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
";
