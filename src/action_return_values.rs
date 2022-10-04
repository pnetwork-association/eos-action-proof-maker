use crate::{
    get_action_digest::get_action_digest,
    types::{EosActionReceipts, Result},
};
use eos_chain::Action as EosAction;

pub fn action_return_value_protocol_feature_is_enabled(
    action: &EosAction,
    action_receipts: &EosActionReceipts,
) -> Result<bool> {
    // NOTE: These will tell us if the feature is enabled _for sure_. However, it can't say for
    // sure that it's not enabled, since the target action digest may not be amongst the set of
    // receipts at all. This behaviour however is sufficient for this proof maker.
    get_action_digest(action, false).map(|digest_from_action| {
        action_receipts
            .iter()
            .map(|receipt| receipt.act_digest.as_bytes().to_vec())
            .any(|digest_from_receipt| digest_from_receipt == digest_from_action)
    })
}
