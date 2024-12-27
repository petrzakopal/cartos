use std::{ffi::CStr};

use pcsc::*;
use tracing::info;
use tracing_log::log::debug;

/// Checks if the reader is dead
fn is_dead(rs: &ReaderState) -> bool {
    rs.event_state().intersects(State::UNKNOWN | State::IGNORE)
}

// Removes dead readers and preserves the live readers
fn remove_dead_reader(mut reader_states: Vec<ReaderState>) -> Vec<ReaderState> {
    for rs in &reader_states {
        if is_dead(rs) {
            info!("Removing dead reader: {:?}", rs.name());
        }
    }

    // Actually removing the dead readers
    // when !is_reader is false - it is removed from the vec
    reader_states.retain(|rs| !is_dead(rs));

    return reader_states;
}

fn add_new_reader<'a>(
    ctx: &Context,
    mut reader_states: Vec<ReaderState>,
    readers_buf: &'a mut [u8; 2048],
) -> (Vec<ReaderState>, Vec<&'a CStr>) {
    //let mut readers_buf_clone = readers_buf.clone();

    // is is in bytes, do not know if needed, but will see
    let reader_names = ctx
        .list_readers(readers_buf)
        .expect("Failed to get the list of readers.");

    let mut readers = ctx
        .list_readers(readers_buf)
        .expect("Failed to get the list of readers.")
        .collect::<Vec<_>>();

    info!("Obtained readers {:?}", readers);
    //info!("Obtained reader names {:?}", reader_names);

    for name in readers.clone() {
        if !reader_states.iter().any(|rs| rs.name() == name) {
            info!("Adding reader {:?}", name);
            reader_states.push(ReaderState::new(name, State::UNAWARE));
        }
    }

    for rs in &mut reader_states {
        rs.sync_current_state();
    }
    return (reader_states, readers);
}

fn print_reader_current_state(reader_states: &Vec<ReaderState>) {
    for rs in reader_states {
        if rs.name() != PNP_NOTIFICATION() {
            debug!("Reader current state: {:?} {:?} {:?}", rs.name(), rs.event_state(), rs.atr());
        }
    }
}

//fn get_connection(ctx: &Context, readers) -> Card {
//
//    let mut connection = match ctx.connect(re, share_mode, preferred_protocols)
//
//}

pub fn initialize_readers() -> (Context, Vec<pcsc::ReaderState>) {
    // Create context
    let ctx =
        Context::establish(Scope::User).expect("Failed to estabilish context for the reader.");

    // Do not know if used, it is used for storing
    // the raw bytes of the reader names
    let mut readers_buf: [u8; 2048] = [0; 2048];
    // Used for reader states
    let mut reader_states = vec![ReaderState::new(PNP_NOTIFICATION(), State::UNAWARE)];

    reader_states = remove_dead_reader(reader_states);

    let mut readers : Vec<&CStr> = Vec::new();

    (reader_states, readers) = add_new_reader(&ctx, reader_states, &mut readers_buf);

    print_reader_current_state(&reader_states);
    // wait until state changes to be able to communicate
    ctx.get_status_change(None, &mut reader_states).expect("Failed to get the status change.");
    print_reader_current_state(&reader_states);


    return (ctx, reader_states);
}


pub async fn read_loop() {

    let (ctx, mut reader_states) : (Context, Vec<pcsc::ReaderState>) = initialize_readers();

    loop {
    print_reader_current_state(&reader_states);
          ctx.get_status_change(None, &mut reader_states).expect("Failed to get the status change.");
           print_reader_current_state(&reader_states);
    }
}
