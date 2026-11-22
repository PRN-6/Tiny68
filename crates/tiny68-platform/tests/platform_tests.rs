use tiny68_platform::autostart::is_autostart_enabled;

#[test]
fn test_autostart_query_nonexistent() {
    // Querying an arbitrary non-registered app should safely return false without crashing
    let enabled = is_autostart_enabled("NonExistent_Tiny68_TestApp_9999");
    assert!(
        !enabled,
        "Non-existent app should not be registered in startup"
    );
}
