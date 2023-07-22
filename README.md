# convert1080

Rust-Iced program to convert pictures to 1920x1080 for video slideshows

This is now part of photorot1080.

I converted one function in photorotate1080 from using gtk4 to iced. Iced is still being developed and the use of progress bar was very hard to implement. I finally go it to work by using mpsc::unbounded for the async to send status of convert and using another async sleeper that just sleeps for 5 seconds and ends which causes the update process which reads the last message and get the percent complete. You have the option to do the progress by first press the start progress button and then pressing the copy button. I am not that skilled in rust and iced, so any help to improve this program would be appreciated.

![Screenshot1080](https://user-images.githubusercontent.com/18278570/215352790-c79c38d0-e639-4f06-9713-f7e3ec7c3694.png)
