<div align="center">

# Duration Detective

</div>

---
**Duration detective** is a `tree` like tool that list the duration of media files inside a folder.

Written in RUST🦀


_This requires [ffmpeg](https://ffmpeg.org/) installed on the system_

## Installation:

```
//Clone the repo
git clone https://github.com/3l-d1abl0/DurationDetective-rs.git
cd duration-detective-rs/

//To see list of commands available
make

//This will build the executable
make duration-detective-build   

```
If all goes well, the binary will be placed at _'target/release/`_

### Example:
```
//This runs the executable on sample files/
make duration-detective-sample-run
```
Ideally this is how you would run it:

<video src="https://github.com/3l-d1abl0/DurationDetective-rs/assets/8142705/cabac374-a04f-423c-956e-018a68c30aa4"></video>



./duration-detective-rs --path="path/to/your/folder"

```
./target/release/duration-detective-rs --path="files/"
Path recieved: files/
Directory to Scan: files/ 
├──audios/
│   ├──mp3/
│   │   └──53sec  file_example_MP3_2MG.mp3
│   │   └──53sec  mp3
│   ├──ogg/
│   │   └──01min 15sec  file_example_OOG_2MG.ogg
│   │   └──01min 15sec  ogg
│   └──wav/
│   │   └──11sec  file_example_WAV_2MG.wav
│   │   └──11sec  wav
│   └──02min 20sec  audios
└──videos/
│   ├──avi/
│   │   └──30sec  file_example_AVI_1920_2_3MG.avi
│   │   └──30sec  avi
│   ├──m4v/
│   │   └──13sec  sample_960x540.m4v
│   │   └──13sec  m4v
│   ├──mp4/
│   │   └──30sec  file_example_MP4_640_3MG.mp4
│   │   └──30sec  mp4
│   ├──mpeg/
│   │   └──28sec  sample_1280x720.mpeg
│   │   └──28sec  mpeg
│   └──webm/
│   │   └──30sec  file_example_WEBM_640_1_4MB.webm
│   │   └──30sec  webm
│   └──02min 13sec  videos
Total Folder Duration: 04min 34sec
```

