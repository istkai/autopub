use yew::prelude::*;

#[derive(PartialEq)]
pub struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    videos: Vec<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos
        .iter()
        .map(|video| {
            html! {
                <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()::<Html>();
}
