LOG 1: Been researching and deciding on what GUI technology to use. Settled on egui and its supporting eframe. I have managed to set up a basic window so far.

LOG 2: Took a while to get egui in the state I wanted it to be in. There are some confusing parts in regards to default apps/setting variables for them.

LOG 3: Got a basic note player going. The biggest problem I encountered with this was for some reason the audio does not play without putting the main thread to sleep. I'm guessing this has something to do with calling the audio player in the update function, which refreshes periodically.

Log 4: I'm attempting to add a volume control to the enote player, but for some reason it only plays at 100.

Log 5: The volume control took a while but I managed to get it to a "reasonable" level. It still plays quite loud however.

Log 6: I updated the homepage.

Log 7: I added more tuning. There are now five in total.

Log 8: It's been quite difficult today. First, soundio really did not want to compile. More recently, I've been running into issues with getting cpal to convert certain values correctly. It took copying multiple parts from https://github.com/jocelyn-stericker/oxygen/commit/7f52f99c516c47668d1e02d6227f9775485db25a and https://github.com/RustAudio/cpal/blob/master/examples/record_wav.rs#L129, and using an older version of cpal to finally get the program to compile again.
I was able to test the output of the recorded audio by just playing it with rodio. It captured my voice so I think that's a success right there. The next step is to work on the fourier transforms of the audio to generate pitch checker.

Log 9: I was able to get a fourier tranformation out of a generated audio signal, but working with it has been tricky.

Log 10: I took some inspiration from a fast fourier transform source code at https://docs.rs/audioviz/latest/src/audioviz/fft.rs.html#55-62. It gave me the idea to normalize the complex data. After that, the index corresponding to the generated frequency seems to match the data. Of course this is just for the generated frequency, I'm sure the recorded one will be much less consistent. All that is left now is to hook it up with the recording and hope for the best.

Log 11: I managed to hook it up and it seems to work. It is very inconsistent. Sometimes the fourier transform will catch the second harmonic more than the first, but if I play loud enough it is kind of clear. Now I just need to wrap it all up and do the finishing touches.

Log 12: I've been cleaning it up the program. It works sometimes. My working theory is it keeps picking up on the harmonics which have their values amplified by the fourier transform. So it kind of works. I still have to document it and record the video. I did some kind of arbitrary math to guard from a value that approaches the second harmonic. I believe this is as robust as it will get with the time I have unfortunately.

Log 13: I updated the documentation and made the video.
