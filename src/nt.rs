use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddrV4},
    sync::{Arc, Mutex},
    time::Duration
};

use network_tables::{
    v4::{Client, Config, Subscription, Type},
    Value,
};

pub enum NoteState {
    None,
    Handoff,
    Shooter,
    AssertFailure,
}
impl From<u64> for NoteState {
    fn from(val: u64) -> Self {
        match val {
            0 => NoteState::None,
            1 => NoteState::Handoff,
            2 => NoteState::Shooter,
            3 => NoteState::AssertFailure,
            _ => NoteState::None,
        }
    }
}

async fn setup_nt_client() -> Result<(Client, Subscription), network_tables::Error> {
    let client = Client::try_new_w_config(
        SocketAddrV4::new(Ipv4Addr::new(10, 36, 36, 2), 5810),
        Config {
            ..Default::default()
        },
    )
    .await?;
    let topic = client
        .publish_topic("RGB/NoteState", Type::Int, None)
        .await?;
    client
        .publish_value(&topic, &Value::Integer(0u64.into()))
        .await?;
    let subscription = client.subscribe_w_options(&["RGB/Note State", "AdvantageKit/RealOutputs/DriveTrain/Estimated Pose"], None).await?;
    Ok((client, subscription))
}

pub async fn nt_subscription_handler(
    note_state: Arc<Mutex<Option<NoteState>>>,
) -> Result<(), Box<dyn Error + Send>> {
    let (_client, mut subscription) = loop {
        match setup_nt_client().await {
            Err(e) => {
                println!("Failed to connect to a network tables server: {}", e);
                println!("Waiting 400ms and trying again");
                tokio::time::sleep(Duration::from_millis(400)).await;
            },
            Ok(info) => break info
        }
    };

    {
        *note_state.lock().unwrap() = Some(NoteState::None);
    }

    loop {
        let Some(update) = subscription.next().await else {
            break;
        };
        println!("{:?}", update);
        if &update.topic_name == "RGB/Note State" {
            let state = update.data.as_u64().unwrap_or(0);
            let state = state.into();
            let mut lock = note_state.lock().unwrap();
            lock.replace(state);
        }
    }

    Ok(())
}
