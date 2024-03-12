use std::path::PathBuf;

use rusty_ytdl::*;

#[derive(Debug)]
pub struct YTDLres {
    pub title: String,
    pub file_name: String,
    pub artist: String,
    pub thumbnail: String,
}

pub async fn ytdl(url: &str) -> YTDLres {
    let video = Video::new(url).unwrap();
    let video_info = video.get_info().await.unwrap();
    let title = video_info.video_details.title;
    let thumbnail = &video_info.video_details.thumbnails.last().unwrap().url;
    let artist = video_info.video_details.owner_channel_name;

    // NOTE: we need to clean filename for any `/`
    let cleaned_title = title.replace("/", "⧸");
    let filename = format!("{}.mp3", cleaned_title);
    let path = std::path::Path::new(&filename);
    video.download(path).await.unwrap();

    YTDLres {
        file_name: filename,
        title,
        artist,
        thumbnail: thumbnail.to_owned(),
    }
}

pub fn convert(input: &str, output: &PathBuf) {
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.arg("-i")
        .arg(input)
        .arg("-vn")
        .arg("-ab")
        .arg("320k")
        .arg("-ar")
        .arg("44100")
        .arg("-y")
        .arg(output);
    cmd.output().unwrap();
}
