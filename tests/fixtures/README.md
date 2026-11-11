# Sample model fixtures

The `.dms` files are unchanged snapshots from the sibling `DEMOSim-models`
repository at commit `f7f53625fcffa6eeeb02adc8a9bd47ab7093e548`.
Keeping copies here makes `cargo test` independent of a sibling checkout.
Update them deliberately when changing model compatibility; do not rewrite UUIDs.

The tests check deserialisation, reference integrity, round trips, and selected
execution scenarios. They do not certify every sample against the textbook.
