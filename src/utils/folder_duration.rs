use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

use crate::format_seconds;

static SYMBOLS_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    [("last", "└──"), ("normal", "├──")]
        .iter()
        .cloned()
        .collect()
});

pub fn folder_duration(directory_path: &str, folder_level: usize) -> f64 {
    let mut total_duration: f64 = 0.0;
    let items = fs::read_dir(directory_path).unwrap();

    //Make a new mutable iterator with peekable
    let mut items_iter = items.peekable();

    //for entry in items_iter {
    while let Some(entry) = items_iter.next() {
        // Check if the next iteration is the last one
        let is_last = items_iter.peek().is_none();
        let symbol: &str;
        if is_last {
            symbol = SYMBOLS_MAP.get("last").unwrap();
        } else {
            symbol = SYMBOLS_MAP.get("normal").unwrap();
        }

        let path = entry.unwrap().path();
        //println!("{}", path.display());

        // Convert PathBuf to &str for the function call
        let path_str = path.to_str().unwrap();
        let current_duration: f64;

        //Directory
        if path.is_dir() {
            println!(
                "{}{}{}/",
                "│   ".repeat(folder_level),
                symbol,
                path.file_name().unwrap().to_str().unwrap()
            );

            current_duration = folder_duration(path_str, folder_level + 1);

            total_duration += current_duration;
            println!(
                "{}{}{}  {}",
                "│   ".repeat(folder_level + 1),
                SYMBOLS_MAP.get("last").unwrap(),
                format_seconds(current_duration),
                path.file_name().unwrap().to_str().unwrap()
            );
        } else {
            //If audio video file
            if check_file(path_str) {
                current_duration = get_duration(path_str);
                println!(
                    "{}{}{}  {}",
                    "│   ".repeat(folder_level),
                    symbol,
                    format_seconds(current_duration),
                    path.file_name().unwrap().to_str().unwrap()
                );
                total_duration += current_duration
            }
        }
    }

    total_duration
}

fn check_file(file_path_str: &str) -> bool {
    //println!("{:?}", infer::get_from_path(file_path_str).unwrap());
    match infer::get_from_path(file_path_str) {
        Ok(Some(info)) => {
            //println!("{:?}", info);
            //println!("mime type: {}", info.mime_type());
            //println!("extension: {}", info.extension());

            let mime_type: Vec<&str> = info.mime_type().split("/").collect();

            if mime_type[0] == "video" || mime_type[0] == "audio" {
                true
            } else {
                false
            }
        }
        Ok(None) => {
            //eprintln!("Unknown file type 😞");
            //eprintln!("If you think infer should be able to recognize this file type open an issue on GitHub!");
            false
        }
        Err(_) => {
            //eprintln!("Looks like something went wrong 😔");
            //eprintln!("{}", e);
            false
        }
    }
}

fn get_duration(file_path_str: &str) -> f64 {
    //Try ffprobe in the Shell
    let output_ffprobe = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(file_path_str)
        .output()
        .expect("Error running ffprobe");

    if !output_ffprobe.status.success() {
        println!("Command executed with failing error code");
        return 0.0;
    } else {
        let duration_str = String::from_utf8_lossy(&output_ffprobe.stdout);

        let duration = duration_str.trim();

        if duration == "N/A" {
            println!("Parsed duration: {}", duration);
            return 0.0;
        } else {
            match duration.parse::<f64>() {
                Ok(duration) => {
                    // Parsing successful, use the `duration` value
                    //println!("Parsed duration: {}", duration);
                    return duration;
                }
                Err(err) => {
                    // Handle the parsing error
                    println!("Error parsing duration: {}", err);
                    return 0.0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    //use std::env;

    use super::*;

    #[test]
    //Test if the function can detect vaious audio video file
    fn test_audio_video_files() {
        //Read each audio video file from the samples
        let mp3_file = "files/audios/mp3/file_example_MP3_2MG.mp3";
        assert_eq!(check_file(mp3_file), true, "Should detect mp3 audio file");

        let ogg_file = "files/audios/ogg/file_example_OOG_2MG.ogg";
        assert_eq!(check_file(ogg_file), true, "Should detect ogg audio file");

        let wav_file = "files/audios/wav/file_example_WAV_2MG.wav";
        assert_eq!(check_file(wav_file), true, "Should detect wav audio file");

        let avi_file = "files/videos/avi/file_example_AVI_1920_2_3MG.avi";
        assert_eq!(check_file(avi_file), true, "Should detect avi video file");

        let m4v_file = "files/videos/m4v/sample_960x540.m4v";
        assert_eq!(check_file(m4v_file), true, "Should detect m4v video file");

        let mp4_file = "files/videos/mp4/file_example_MP4_640_3MG.mp4";
        assert_eq!(check_file(mp4_file), true, "Should detect mp4 video file");

        let mpeg_file = "files/videos/mpeg/sample_1280x720.mpeg";
        assert_eq!(check_file(mpeg_file), true, "Should detect mpeg video file");

        let webm_file = "files/videos/webm/file_example_WEBM_640_1_4MB.webm";
        assert_eq!(check_file(webm_file), true, "Should detect webm video file");
    }

    #[test]
    //Test for the cumulative duration of a folder
    fn test_folder_duration() {
        //1. Check total duaration fo video sample folder
        let video_folder = "files/videos/";
        assert_eq!(
            folder_duration(video_folder, 0),
            133.266533,
            "Expected the total seconds of the video files to be equal!"
        );

        //2. Check total duaration fo audio sample folder
        let audios_folder = "files/audios/";
        assert_eq!(
            folder_duration(audios_folder, 0),
            140.90975,
            "Expected the total seconds of the audio files to be equal!"
        );

        //3. Check total duaration fo video+audio sample folder
        let test_folder = "files/";
        assert_eq!(
            folder_duration(test_folder, 0),
            274.176283,
            "Expected the total seconds of the test folders to be equal!"
        );
    }
}
