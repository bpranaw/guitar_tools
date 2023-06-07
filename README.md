# Guitar Tools

This is a simple app made in rust to help with your guitar tuning. The home page has some information regarding how to use the app and some information about some of the concepts behind guitar tuning.

You can use the radio buttons on the top to switch to the two other modes. With the "Tune by ear" mode, there is a selection box that allows you to chose what tuning you would like to set, example: Standard tuning: EADGBE. Pressing the buttons will play a generated audio at that note. You can change the volume of the audio played. WARNING, depending on how loud your speakers are, this can be very loud.

Similarly with the "Tune by recording" mode, you can select what tuning standard you want to set. You then select the string you want to tune. You play the string on your guitar, and the program will listen to your audio and tell you whether you need to tighten or loosen the tension in your string, as well as show you how far it is from the proper pitch.

# How to run

1. Install Rust with Rustup: https://www.rust-lang.org/tools/install
2. If on linux, run this on the command line for eframe to work:

sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev

3. Extract files if it's in a zip
4. Enter "cargo run" in terminal while in project directory.

# Progress

You can see a summary of the progress of this project in the "Progress.md" file.
