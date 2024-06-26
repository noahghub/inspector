// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod db;
mod parse;
mod stats;
mod track;

use core::panic;
use diesel::SqliteConnection;
use parse::HandDetail;
use std::path::Path;

use crate::db::establish_connection;

fn main() {
  let directory = "/mnt/windows/Users/noah/AppData/Local/PokerStars.BE/HandHistory/PokerZhyte/play";
  let mut files = Vec::new();

  // add file content in vectore
  for entry in std::fs::read_dir(directory).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    let path_str = path.to_str().unwrap();
    files.push(path_str.to_string());
  }

  let file = "./test/test_hands.txt";

  // parse the first file
  let hands_detail = match parse::parse_file(file) {
    Err(e) => panic!("Error {}\nparsing file : {:#?}", e, files[0]),
    Ok(h) => h,
  };
  println!("Number of hands {}", hands_detail.len());

  // compute the stats for PokerZhyte player
  let mut poker_zhyte = db::models::Player::new("PokerZhyte");
  stats::add(&mut poker_zhyte, &hands_detail);

  // compute hand of DB from hand detail
  let mut conn = establish_connection().unwrap();
  update_db(&mut conn, &hands_detail);

  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  use std::sync::mpsc::channel;
  let (tx, rx) = channel();

  let path = Path::new(r"/home/noah/test");
  if let Err(error) = track::watch(path, tx, rx) {
    log::error!("Error : {error:?}");
  }
}

// miss player
fn update_db(conn: &mut SqliteConnection, hands_detail: &Vec<HandDetail>) {
  for hand_detail in hands_detail {
    let actions: Vec<db::models::Action> = hand_detail.get_actions();
    db::insert_actions(conn, &actions);

    let (small_blind, big_blind) = hand_detail.get_blinds();
    db::insert_blind(conn, &small_blind);
    db::insert_blind(conn, &big_blind);

    let hand = hand_detail.get_hand();
    db::insert_hand(conn, &hand);

    let hole_cards: Vec<db::models::HoleCard> = hand_detail.get_hole_cards();
    db::insert_hole_cards(conn, &hole_cards);
  }
}
