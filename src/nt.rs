use std::{error::Error, net::{Ipv4Addr, SocketAddrV4}};

use network_tables::v4::{Client, Config, Subscription, SubscriptionOptions};


pub async fn setup_nt_client() -> Result<(Client, Subscription), Box<dyn Error>> {
    let client = Client::try_new_w_config(
        SocketAddrV4::new(Ipv4Addr::new(10, 36, 36, 2), 5810),
        Config {
            ..Default::default()
        },
    )
    .await?;
    let subscription = client.subscribe_w_options(&[""], Some(SubscriptionOptions {
        all: Some(true),
        ..Default::default()
    })).await?;
    Ok((client, subscription))
}
