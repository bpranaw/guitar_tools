LOG 1: Been researching and deciding on what GUI technology to use. Settled on egui and its supporting eframe. I have managed to set up a basic window so far.
LOG 2: Took a while to get egui in the state I wanted it to be in. There are some confusing parts in regards to default apps/setting variables for them.
LOG 3: Got a basic note player going. The biggest problem I encountered with this was for some reason the audio does not play without putting the main thread to sleep. I'm guessing this has something to do with calling the audio player in the update function, which refreshes periodically.
Log 4: I'm attempting to add a volume control to the enote player, but for some reason it only plays at 100.
Log 5: The volume control took a while but I managed to get it to a "reasonable" level. It still plays quite loud however.
Log 6: I updated the homepage.
