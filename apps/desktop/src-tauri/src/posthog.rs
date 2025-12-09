use std::{
    sync::{OnceLock, PoisonError, RwLock},
    time::Duration,
};
use tracing::error;

#[derive(Debug)]
pub enum PostHogEvent {
    MultipartUploadComplete {
        // Upload duration
        duration: Duration,
        // Length of the video
        length: Duration,
        // Size of the file in megabytes
        size: u64,
    },
    MultipartUploadFailed {
        // Upload duration
        duration: Duration,
        // Error message
        error: String,
    },
}

impl From<PostHogEvent> for posthog_rs::Event {
    fn from(event: PostHogEvent) -> Self {
        match event {
            PostHogEvent::MultipartUploadComplete {
                duration,
                length,
                size,
            } => {
                let mut e = posthog_rs::Event::new_anon("multipart_upload_complete");
                e.insert_prop("duration", duration.as_secs())
                    .map_err(|err| error!("Error adding PostHog property: {err:?}"))
                    .ok();
                e.insert_prop("length", length.as_secs())
                    .map_err(|err| error!("Error adding PostHog property: {err:?}"))
                    .ok();
                e.insert_prop("size", size)
                    .map_err(|err| error!("Error adding PostHog property: {err:?}"))
                    .ok();
                e
            }
            PostHogEvent::MultipartUploadFailed { duration, error } => {
                let mut e = posthog_rs::Event::new_anon("multipart_upload_failed");
                e.insert_prop("duration", duration.as_secs())
                    .map_err(|err| error!("Error adding PostHog property: {err:?}"))
                    .ok();
                e.insert_prop("error", error)
                    .map_err(|err| error!("Error adding PostHog property: {err:?}"))
                    .ok();
                e
            }
        }
    }
}

pub fn init() {
    // PostHog 分析已禁用
}

pub fn set_server_url(url: &str) {
    *API_SERVER_IS_CAP_CLOUD
        .get_or_init(Default::default)
        .write()
        .unwrap_or_else(PoisonError::into_inner) = Some(url == "https://cap.so");
}

static API_SERVER_IS_CAP_CLOUD: OnceLock<RwLock<Option<bool>>> = OnceLock::new();

pub fn async_capture_event(_event: PostHogEvent) {
    // PostHog 分析已禁用
}
