# Author

Pranaw Bajracharya

# Guitar Tools

This is a simple app made in rust to help with your guitar tuning. The home page has some information regarding how to use the app.

You can use the radio buttons on the top to switch to the two other modes. With the "Tune by ear" mode, there are several common tunning schemes, example: Standard tuning: EADGBE. Pressing the buttons will play a generated audio signal at that note. You can change the volume of the audio played. WARNING, depending on how loud your speakers are, this can be very loud.

Similarly, with the "Tune by recording" mode, you have several tuning schemes to choose from. You then select the string you want to tune. You play the string on your guitar, and the program will listen to your audio and tell you whether you need to tighten or loosen the tension in your string, as well as show you how far it is from the proper pitch.

This project was created for my CS 410P - Rust and CS 410P - Music and Sound classes at Portland State.

# How to run

1. Install Rust with Rustup: [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. If on linux, run this on the command line for eframe to work:

sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev

3. Extract files if it's in a zip
4. Enter "cargo run" in terminal while in project directory.

# Testing

Unfornately, there are no unit tests or integration tests as of yet.
The project was in a small enough scope, and with its music/sound nature, manual testing was the best fit.
I did some manual testing along the way. Ex: validating that the proper pitches were being played, trying to break the interface, etc.
There is definitely work to due in this regard.

# Progress and Future Work

My progress log from across this project can be seen [./Progress.md](/Progress.md).

This project had many feature goals that were not accomplished, such as a note transcriber or reader.
Additionally, the functionality of the "tune-by-recording" portion is inconsistent. However, the project did accomplish its primary goals:

1. Apply the lessons from CS 410P - Music and Sound to create a tool that is personally useful, i.e. a guitar tuning app.
2. Apply the lessons from CS 410P - Rust to create an application.

The project has been a great learning experience towards those goals. Although it is in a very protoype-like state, I am proud of its existence.
The challenge was learning the concepts of both classes and applying them to the project. I would get stuck on as aspect of the code because I am still relatively new to Rust, then I would get stuck conceptually because of the difficult concepts in music theory.

However, there is still much to be done in the future. One major flaw is that the "tune by recording" is very inconsistent, especially at pitches lower than 82 Hz.
One improvement is to make the "tunning by recording" have real-time processing rather than post recording.
Ultimately, the recording should be constant once the user decides that is what they would like to do, i.e. is in constant note interpreting mode.

I would also like to implement a guitar tab/sheet music transcriber. It would listen in and generate a proper guitar tab file. The logical extention to that would be to have it be able to play that tablature as well.

# License

The license can be viewed at ["./LICENSE"](/LICENSE)
