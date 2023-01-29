use iced::widget::{button, column, row, text, scrollable, progress_bar};
use iced::{Alignment, Element, Command, Application, Length, Settings, Color, Theme};
use iced::theme;
use iced::executor;
use std::process::Command as stdCommand;
use std::path::{Path};
use iced_futures::futures;
use futures::channel::mpsc;
use std::time::{Duration, Instant};
use std::thread::sleep;

mod get_dirlist;
mod dirpressx;
mod diroutpressx;
mod dump_file;
mod copypressx;
use get_dirlist::get_dirlist;
use dirpressx::dirpressx;
use diroutpressx::diroutpressx;
use copypressx::copypressx;

pub fn main() -> iced::Result {
    Conv1080x::run(Settings::default())
}

struct Conv1080x {
    dir_value: String,
    mess_color: Color,
    msg_value: String,
    do_progress: bool,
    scrol_value: String,
    outdir_value: String,
    progval: f32,
    tx_send: mpsc::UnboundedSender<String>,
    rx_receive: mpsc::UnboundedReceiver<String>,
}

#[derive(Debug, Clone)]
enum Message {
    DirPressed,
    OutDirPressed,
    CopyPressed,
    ProgressPressed,
    ProgRtn(Result<Progstart, Error>),
    CopyxFound(Result<Copyx, Error>),
}

impl Application for Conv1080x {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;
    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let (tx_send, rx_receive) = mpsc::unbounded();
        (
            Conv1080x {
                dir_value: "no directory".to_string(),
                mess_color: Color::from([0.0, 0.0, 0.0]),
                msg_value: "no message".to_string(),
                do_progress: false,
                scrol_value: " No directory selected \n ".to_string(),
                outdir_value: "no directory".to_string(),
                progval: 0.0,
                tx_send,
                rx_receive,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Convert to 1920 X 1080 -- iced")
    }

    fn update(&mut self, message: Message) -> Command<Message>  {
        match message {
            Message::DirPressed => {
               let (colorout, errstr, newdir, newliststr) = dirpressx();
               self.scrol_value  = newliststr.to_string();
               self.dir_value = newdir.to_string();
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               Command::none()
            }
            Message::OutDirPressed => {
               let (colorout, errstr, newdir) = diroutpressx();
               self.outdir_value = newdir.to_string();
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               Command::none()
            }
            Message::CopyPressed => {
               let (errcode, colorout, errstr) = copypressx(self.dir_value.clone(),self.outdir_value.clone(), self.scrol_value.clone());
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               if errcode == 0 {
                   Command::perform(Copyx::copyit(self.dir_value.clone(),self.outdir_value.clone(), self.scrol_value.clone(), self.tx_send.clone()), Message::CopyxFound)

               } else {
                   Command::none()
               }
            }
            Message::ProgressPressed => {
                   self.do_progress = true;
                   Command::perform(Progstart::pstart(), Message::ProgRtn)
            }
            Message::CopyxFound(Ok(copyx)) => {
                self.msg_value = copyx.errval.clone();
                self.mess_color = copyx.errcolor.clone();
                self.do_progress = false;
                self.progval = 0.0;
                Command::none()
            }
            Message::CopyxFound(Err(_error)) => {
                self.msg_value = "error in copyx copyit routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
                Command::none()
            }
            Message::ProgRtn(Ok(_prx)) => {
              if self.do_progress {
                let mut inputval  = " ".to_string();
                let mut bgotmesg = false;
                while let Ok(Some(input)) = self.rx_receive.try_next() {
                   inputval = input;
                   bgotmesg = true;
                }
                if bgotmesg {
                    let progvec: Vec<&str> = inputval[0..].split("|").collect();
                    let lenpg1 = progvec.len();
                    if lenpg1 == 3 {
                        let prog1 = progvec[0].clone().to_string();
                        if prog1 == "Progress" {
                            let num_int: i32 = progvec[1].clone().parse().unwrap_or(-9999);
                            if num_int == -9999 {
                                println!("progress numeric not numeric: {}", inputval);
                            } else {
                                let dem_int: i32 = progvec[2].clone().parse().unwrap_or(-9999);
                                if dem_int == -9999 {
                                    println!("progress numeric not numeric: {}", inputval);
                                } else {
                                    self.progval = 100.0 * (num_int as f32 / dem_int as f32);
                                    self.msg_value = format!("Convert progress: {}", self.progval);
                                    self.mess_color = Color::from([0.0, 0.0, 1.0]);
                                }
                            }
                        } else {
                            println!("message not progress: {}", inputval);
                        }
                    } else {
                        println!("message not progress: {}", inputval);
                    }
                }             
                Command::perform(Progstart::pstart(), Message::ProgRtn)
              } else {
                Command::none()
              }
            }
            Message::ProgRtn(Err(_error)) => {
                self.msg_value = "error in Progstart::pstart routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
                Command::none()
            }

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(30),
                 text(&self.msg_value).size(30).style(*&self.mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Directory Button").on_press(Message::DirPressed).style(theme::Button::Secondary),
                 text(&self.dir_value).size(30),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            scrollable(
                column![
                        text(format!("{}",&self.scrol_value))
                ].width(Length::Fill),
            ).height(Length::Units(100)),
            row![button("outDirectory Button").on_press(Message::OutDirPressed).style(theme::Button::Secondary),
                 text(&self.outdir_value).size(30),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text(" ").size(10), button("Copy Button").on_press(Message::CopyPressed).style(theme::Button::Secondary),
                 text(" ").size(10), button("Start Progress Button").on_press(Message::ProgressPressed).style(theme::Button::Secondary),
            ].align_items(Alignment::Center).spacing(200).padding(10),
            progress_bar(0.0..=100.0,self.progval),
         ]
        .padding(10)
        .align_items(Alignment::Start)
        .into()
    }
}
#[derive(Debug, Clone)]
struct Copyx {
    errcolor: Color,
    errval: String,
}

impl Copyx {

    async fn copyit(dir_value: String, outdir_value: String, mergescrol_value: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Copyx, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 0.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     let start_time = Instant::now();
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl].clone();
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[0].clone().to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx[1..];
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          let str_cur_dirout = outdir_value.clone();
          let fullto = str_cur_dirout.clone() + "/" + &filefromx;
          if Path::new(&fullto).exists() {
              errstring = format!("********* convert Copy: ERROR {} already exists **********", fullto);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          if numprocess < 4 {
              stdCommand::new("convert")
                           .arg(&fullfrom)
                           .arg("-resize")
                           .arg("1920x1080")
                           .arg("-background")
                           .arg("black")
                           .arg("-gravity")
                           .arg("center")
                           .arg("-extent")
                           .arg("1920x1080")
                           .arg(&fullto)
                           .spawn()
                           .expect("failed to execute process");
              numprocess = numprocess + 1;
          } else {
              let _output = stdCommand::new("convert")
                           .arg(&fullfrom)
                           .arg("-resize")
                           .arg("1920x1080")
                           .arg("-background")
                           .arg("black")
                           .arg("-gravity")
                           .arg("center")
                           .arg("-extent")
                           .arg("1920x1080")
                           .arg(&fullto)
                           .output()
                           .expect("failed to execute process");
              numprocess = 0;
              let msgx = format!("Progress|{}|{}", numrow, lenmg1);
              tx_send.unbounded_send(msgx).unwrap();

          }

          numrow = numrow + 1;
     }
     if bolok {
         let diffx = start_time.elapsed();     
         errstring = format!("converted copied {} files in {} seconds", lenmg1, diffx.as_secs());
         colorx = Color::from([0.0, 0.0, 0.0]);
     }
     Ok(Copyx {
            errcolor: colorx,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
enum Error {
    APIError,
}
// loop thru by sleeping for 5 seconds
#[derive(Debug, Clone)]
struct Progstart {
    errcolor: Color,
    errval: String,
}

impl Progstart {

    async fn pstart() -> Result<Progstart, Error> {
     let errstring  = " ".to_string();
     let colorx = Color::from([0.0, 0.0, 0.0]);
     sleep(Duration::from_secs(5));
     Ok(Progstart {
            errcolor: colorx,
            errval: errstring,
        })
    }
}
