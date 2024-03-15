use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddrV4},
    sync::{Arc, Mutex},
};

use network_tables::{v4::{Client, Config, Subscription, Type}, Value};

#[repr(u64)]
pub enum NoteState {
    None = 0,
    Handoff = 1,
    Shooter = 2,
}
impl From<u64> for NoteState {
    fn from(val: u64) -> Self {
        match val {
            0 => NoteState::None,
            1 => NoteState::Handoff,
            2 => NoteState::Shooter,
            _ => NoteState::None,
        }
    }
}

pub async fn setup_nt_client() -> Result<(Client, Subscription), Box<dyn Error>> {
    let client = Client::try_new_w_config(
        SocketAddrV4::new(Ipv4Addr::new(10, 36, 36, 1), 5810),
        Config {
            ..Default::default()
        },
    )
    .await?;
    let topic = client
        .publish_topic("RGB/NoteState", Type::Int, None)
        .await?;
    client.publish_value(&topic, &Value::Integer(0u64.into())).await?;
    let subscription = client.subscribe_w_options(&["RGB/NoteState"], None).await?;
    Ok((client, subscription))
}

pub async fn nt_subscription_handler(
    mut subscription: Subscription,
    note_state: Arc<Mutex<NoteState>>,
) -> Result<(), Box<dyn Error + Send>> {
    loop {
        let Some(update) = subscription.next().await else {
            break;
        };
        println!("{:?}", update);
        if update.topic_name == String::from("RGB/NoteState") {
            let state = update.data.as_u64().unwrap_or(0);
            let state = state.into();
            let mut lock = note_state.lock().unwrap();
            *lock = state;
        }
    }

    Ok(())
}
