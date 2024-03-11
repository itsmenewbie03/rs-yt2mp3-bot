use std::collections::HashMap;

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
    // TODO: gen a filenam from the title
    let video_info = video.get_info().await.unwrap();
    let title = video_info.video_details.title;
    let thumbnail = &video_info.video_details.thumbnails.last().unwrap().url;
    let artist = video_info.video_details.owner_channel_name;
    // NOTE: we need to clean filename for any `/`
    let cleaned_title = title.replace("/", "⧸");
    let filename = format!("{}.m4a", cleaned_title);
    let path = std::path::Path::new(&filename);
    video.download(path).await.unwrap();
    YTDLres {
        file_name: filename,
        title,
        artist,
        thumbnail: thumbnail.to_owned(),
    }
}
