// Integration test for SigmaOS-alsa-subsystem
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use alsa_subsystem as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
