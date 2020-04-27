use crate::{
    state::State,
    types::{
        Result,
        EosInputJson,
        EosActionReceiptAndIdJson,
    },
};

fn get_action_receipts_from_transactions(
    json: &EosInputJson
) -> Result<Vec<EosActionReceiptAndIdJson>> {
    Ok(
        json
            .transactions
            .iter()
            .map(|tx|
                 tx
                    .action_traces
                    .iter()
                    .map(|trace|
                        EosActionReceiptAndIdJson::new(
                            tx.id.clone(),
                            trace.receipt.clone(),
                        )
                    )
                    .collect::<Vec<EosActionReceiptAndIdJson>>()

            )
            .flatten()
            .collect::<Vec<EosActionReceiptAndIdJson>>()
    )
}

pub fn get_eos_actions_from_transactions_and_put_in_state(
    state: State
) -> Result<State> {
    trace!("✔ Parsing EOS transactions...");
    state
        .get_eos_input_json()
        .and_then(|json| get_action_receipts_from_transactions(&json))
        .and_then(|receipts| state.add_eos_actions_with_id(receipts))
}
