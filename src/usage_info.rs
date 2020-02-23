pub static USAGE_INFO: &'static str = "
❍ EOS Action-Proof Maker ❍

    Copyright Greg Kapka 2019
    Questions: greg@oraclize.it

❍ Info ❍

A tool to make merkle-proofs over EOS actions in order to prove that action's
retirement in a given block.

❍ Usage ❍

Usage:
        eos_action_proof_maker [--help]
        eos_action_proof_maker generate <INDEX> (<JSON> | --file=<PATH>)

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
                          `block`           ➔ The EOS block header.
                          `actions`         ➔ An array of all the actions
                                              retired in the block in question.
                          `action_receipts` ➔ An array of all the action
                                              receipts pertaining to the block
                                              in question.

    <PATH>              ❍ Path to a file containing the valid <JSON> string
                          described above.

    <INDEX>             ❍ An integer between 0 & the number of actions included
                          in the supplied block minus one.
";

