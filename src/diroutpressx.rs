use iced::Color;
use rfd::FileDialog;
//use crate::get_dirlist;
pub fn diroutpressx () -> (Color, String, String) {
     let errstring: String;
     let mut new_dir: String = " ".to_string();
     let colorx: Color;
     let folder = FileDialog::new()
                    .pick_folder();
     if folder == None {
         errstring = "error getting output directory -- possible cancel key hit".to_string();
         colorx = Color::from([1.0, 0.0, 0.0]);
     } else {
         new_dir = folder.as_ref().expect("REASON").display().to_string();
         errstring = "convert output directory selected".to_string();
         colorx = Color::from([0.0, 0.0, 0.0]);
     } 
    (colorx, errstring, new_dir)
}

