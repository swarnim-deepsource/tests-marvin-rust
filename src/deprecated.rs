fn foo() {
    std::thread::sleep_ms(10);

    // fine
    std::thread::sleep(std::time::Duration::from_millis(10));

    let connected = ["hello", "world"].connect(" ");
}
