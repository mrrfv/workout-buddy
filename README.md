# Workout Buddy

Play your favorite workout video on YouTube and mute it. Workout Buddy will attempt to detect the time remaining for the current exercise using an on-device vision language model, and send an audible notification when it's time to stop.

## Demo

![Demo GIF](.github/demo.GIF)

Source video can be found [here](https://www.youtube.com/watch?v=UMzQonUGE_s). I am not affiliated with nor endorsed by the video author.

## Features

- Video-agnostic. It works with any YouTube video, as long as it contains a countdown timer
- Ignores irrelevant scenes (e.g. other tabs, windows)
- Runs entirely on your device. Optimized for [LM Studio](https://lmstudio.ai/) and the Gemma 3 4B model, but anything with vision and OpenAI API-compatible should work
- Analyzes 3 screenshots at once, allowing the model to find less obvious timers 
- Doesn't save captured screenshots for privacy
- Customizable notifications and sounds

## Installation

1. Download the pre-built binary from Releases or build the code yourself.
2. In the working directory, create the configuration file - `config.json` - and fill it out according to the example in `config.example.json`. Optionally, find a notification sound and point to it.
3. Ensure you have the LM Studio API running and the model is loaded.

## Usage

1. Open your workout video on YouTube.
2. Launch the binary.
3. If you disabled `send_notification_overtime`, wait until the app catches up with the video.
4. Move in silence or to your own music!

## Disclaimer

I've created this project both as a Rust exercise and to scratch an itch. While it exceeds my personal expectations, it may not handle all videos as effectively. It should be considered more as a proof-of-concept. Have fun exercising!
