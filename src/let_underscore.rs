async fn bar() -> i32 {
    0
}

async fn foo() {
    let _ = bar();
    _ = bar();
    let _ = async { bar().await };

    // fine!
    let _ = bar().await;
    let x = bar();

    let _ = ();
    let _ = std::thread::sleep_ms(10);
    let _ = vec![].push(0);
}
