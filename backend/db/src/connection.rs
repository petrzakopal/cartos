use common::types::channels::CardData;
use tracing::debug;

pub async fn user_validation(serial_number_channel_sender: tokio::sync::broadcast::Sender<CardData>){

    let mut receiver = serial_number_channel_sender.subscribe();


    while let Ok(mut serial_number_data) = receiver.recv().await {

        debug!("Received serial number string data {:#?} proceeding to validate in the db.", serial_number_data);

    }
}
