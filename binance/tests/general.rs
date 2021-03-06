use binance::Binance;

#[tokio::test]
async fn ping() {
    let exchange = Binance::new(true);
    assert_eq!("pong", exchange.ping().await.unwrap());
}

#[tokio::test]
async fn get_server_time() {
    let exchange = Binance::new(true);
    exchange.get_server_time().await.unwrap();
}

#[tokio::test]
async fn get_exchange_info() {
    let exchange = Binance::new(true);
    exchange.get_exchange_info().await.unwrap();
}
