#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
    assert!(true);
}
