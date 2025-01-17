use common::{
    hw::gpio::gpio_indicate_user_authorized,
    types::{
        channels::CardData,
        database::{CardRead, LogEntry},
        websockets::{WebsocketMessageBody, WebsocketMessageData},
    },
};
use tracing::{debug, error, field::debug, info, warn};
// provides `try_next`
use futures::TryStreamExt;
// provides `try_get`
use sqlx::{Pool, Row, Sqlite};

use crate::initialize_db::connect_db_sqlite;

pub async fn get_sqlite_db_pool() -> Pool<Sqlite> {
    let pool_res = connect_db_sqlite().await;

    let pool = pool_res.unwrap_or_else(|e| {
        error!("The db pool is not accessible. {:#?}", e);
        panic!("Failed to connect to the sqlite db pool.")
    });

    return pool;
}

pub async fn user_validation(
    card_data_channel_sender: tokio::sync::broadcast::Sender<CardData>,
    ws_body_channel_sender: tokio::sync::broadcast::Sender<WebsocketMessageBody>,
) {
    let mut receiver = card_data_channel_sender.subscribe();

    while let Ok(card_data) = receiver.recv().await {
        debug!(
            "Received serial number string data {} proceeding to validate in the db.",
            card_data.serial_number_string
        );

        let pool_res = connect_db_sqlite().await;

        let pool = match pool_res {
            Ok(pool) => pool,
            Err(e) => {
                error!("Error creating a pool {:#?}", e);
                return;
            }
        };

        let card_read: CardRead = CardRead {
            card_serial_number: card_data.serial_number_string.clone(),
        };

        let ws_body: WebsocketMessageBody = WebsocketMessageBody {
            action: common::types::websockets::MessageAction::CardRead,
            data: WebsocketMessageData::CardRead(card_read),
        };

        debug!("Sending ws data by channel {:#?}", ws_body);

        let _ = ws_body_channel_sender.send(ws_body);

        // Expecting that when very simple db table with users and their respective cardSerialNumber
        // is used, there is always only one entry with the serial_card_number which is being searched for
        // so the loop can be broken if one entry is found.
        let mut users_fetched =
            sqlx::query(r#"SELECT * FROM user WHERE card_serial_number = ? AND status = 'active'"#)
                .bind(&card_data.serial_number_string)
                .fetch(&pool);

        let mut validated_user_email: String = String::default();
        let mut is_user_validated: bool = false;

        // Iterating through the results
        loop {
            match users_fetched.try_next().await {
                Ok(Some(data)) => {
                    // Just getting the email of the user which is trying to access the resource
                    // (card scan)
                    let email_from_db: &str = match data.try_get("email") {
                        Ok(email) => email,
                        Err(e) => {
                            error!(
                                "Cannot fetch the data for serial_number {}: {}",
                                card_data.serial_number_string, e
                            );
                            return;
                        }
                    };

                    // Use in the access logging to db function
                    validated_user_email = email_from_db.to_string();
                    is_user_validated = true;

                    info!(
                        "Fetched data from the db for serial_card_number {}: {}",
                        &card_data.serial_number_string, email_from_db
                    );
                    break;
                }
                // No user with provided serial_card_number is found
                Ok(None) => {
                    warn!(
                        "No user found with serial_card_number {}.",
                        &card_data.serial_number_string
                    );
                    break; // Exit the loop when no more rows are available
                }
                Err(e) => {
                    error!(
                        "There has been an error fetching data for serial_card_number {}: {}",
                        &card_data.serial_number_string, e
                    );
                    return;
                }
            }
        }

        if is_user_validated {
            debug!("Will log the SUCCESSFUL action of serial_card_number: {} by user email: {} to the db.", &card_data.serial_number_string, validated_user_email);

            let mut inserted_log_entry : LogEntry = sqlx::query_as(
                r#"INSERT INTO log (card_serial_number, email, status) VALUES (?, ?, ?) RETURNING *;"#,
            )
            .bind(&card_data.serial_number_string)
            .bind(validated_user_email)
            .bind("authenticated")
            .fetch_one(&pool)
            .await
            .unwrap(); //.expect("could not insert log to the db");

            let gpio_res = gpio_indicate_user_authorized();

            match gpio_res {
                Ok(v) => {
                    debug!("Successfully performed the gpio operation.")
                }
                Err(e) => {
                    error!("Did not perform the gpio operation successfully. {:#?}", e)
                }
            };

        let logged_user: LogEntry = LogEntry {
                id: inserted_log_entry.id,
                timestamp: inserted_log_entry.timestamp,
                card_serial_number: inserted_log_entry.card_serial_number,
                email: inserted_log_entry.email,
                status: inserted_log_entry.status,
                note: inserted_log_entry.note
        };

        let ws_body: WebsocketMessageBody = WebsocketMessageBody {
            action: common::types::websockets::MessageAction::NewLogEntry,
            data: WebsocketMessageData::LogEntry(logged_user),
        };

        debug!("Sending ws data by channel {:#?}", ws_body);

        let _ = ws_body_channel_sender.send(ws_body);
        } else {
            debug!("Will log the UNSUCCESSFUL action of serial_card_number: {} by user email: {} to the db.", &card_data.serial_number_string, validated_user_email);

            let mut inserted_log_entry = sqlx::query(
                r#"INSERT INTO log (card_serial_number, email, status) VALUES (?, ?, ?);"#,
            )
            .bind(&card_data.serial_number_string)
            .bind(validated_user_email)
            .bind("not_authenticated")
            .execute(&pool)
            .await; //.expect("could not insert log to the db");
        }
    }
}
