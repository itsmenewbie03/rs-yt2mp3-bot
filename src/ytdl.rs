use rusty_ytdl::*;

pub async fn ytdl(url: &str) -> String {
    let video = Video::new(url).unwrap();
    // TODO: gen a filenam from the title
    let video_info = video.get_info().await.unwrap();
    let title = video_info.video_details.title;
    let filename = format!("{}.mp3", title);
    let path = std::path::Path::new(&filename);
    video.download(path).await.unwrap();
    filename
}
