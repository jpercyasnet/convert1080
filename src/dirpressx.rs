extern crate exif;
extern crate chrono;
// use gtk::prelude::*;
// mod get_dirlist;
//use std::io::BufReader;
//use std::fs::File;
use iced::Color;
// use std::path::{Path};
//use std::fs;
use rfd::FileDialog;
// use exif::{Reader, In, Tag};
//use crate::dump_file::dump_file;
//use chrono::prelude::*;
//use std::path::{PathBuf};
use crate::get_dirlist;
// use get_dirlist::get_dirlist;
// function called by Organize directory 1 & 2  buttons and Convert directory button
//  Use to get list of sorted files in the directory list in model format
// input is the directory and output is error number, error string and model
pub fn dirpressx () -> (Color, String, String, String) {
     let errstring: String;
     let mut new_dirlist: String = " ".to_string();
     let mut new_dir: String = " ".to_string();
     let colorx : Color;
     let folder = FileDialog::new()
                    .pick_folder();
     if folder == None {
         errstring = "error getting directory -- possible cancel key hit".to_string();
         colorx = Color::from([1.0, 0.0, 0.0]);
     } else {
         new_dir = folder.as_ref().expect("REASON").display().to_string();
         let current_dir = folder;
         let (errcd, errstr, newliststr) = get_dirlist(current_dir.unwrap());
         if errcd == 0 {
             new_dirlist = newliststr;
             errstring = "got directory".to_string();
             colorx = Color::from([0.0, 0.0, 0.0]);
         } else {
             errstring = errstr.to_string();
             colorx = Color::from([1.0, 0.0, 0.0]);
         }
     } 
    (colorx, errstring, new_dir, new_dirlist)
}

