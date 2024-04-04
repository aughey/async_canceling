pub async fn long_running_async_task() {
    // async sleep for one second
    println!("Entered long_running_async_task");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("after first sleep");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("after second sleep");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("after third sleep");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("after fourth sleep");
}
