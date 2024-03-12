use std::fs::remove_file;

use id3::{frame::Picture, Tag, TagLike};

use crate::ytdl::{self, YTDLres};
use reqwest::Client;

pub async fn add_tags(video_data: YTDLres) -> std::path::PathBuf {
    let file_name = video_data.file_name;
    let temp_file = std::env::temp_dir().join(&file_name);
    ytdl::convert(&file_name, &temp_file);
    let mut tag = Tag::new();
    tag.set_artist(video_data.artist);
    tag.set_title(video_data.title);
    tag.add_frame(Picture {
        description: "Album Art grabbed from YT Thumbnail. ~@itsmenewbie03".to_owned(),
        picture_type: id3::frame::PictureType::CoverFront,
        mime_type: "image/jpeg".to_owned(),
        data: get_image_data(&video_data.thumbnail).await,
    });
    tag.write_to_path(&temp_file, id3::Version::Id3v24)
        .expect("FAILED TO WRITE TAGS TO TEMP FILE");
    // NOTE: we gonna delete the file
    remove_file(&file_name).expect("FAILED TO DELETE FILE");
    temp_file
}

async fn get_image_data(url: &str) -> Vec<u8> {
    let client = Client::new();
    let response = client.get(url).send().await.unwrap().bytes().await.unwrap();
    response[..response.len()].to_vec()
}
