use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::context::AlertProps;
use crate::repository;
use crate::repository::db::models::Track;
use crate::repository::playlist::PlaylistWithTracks;

#[component]
pub fn PlaylistProvider(children: Element) -> Element {
    use_context_provider(PlaylistContext::new);
    rsx! {
        {children}
    }
}

pub fn use_playlists() -> PlaylistContext {
    use_context::<PlaylistContext>()
}

#[derive(Clone, Copy, PartialEq)]
pub struct PlaylistContext {
    pub items: Signal<Vec<PlaylistWithTracks>>,
    pub detail: Signal<Option<PlaylistWithTracks>>,
    pub is_loading: Signal<bool>,
    pub detail_is_loading: Signal<bool>,
    pub error: Signal<Option<AlertProps>>,
    pub pending_track: Signal<Option<Item>>,
}

impl PlaylistContext {
    pub fn new() -> Self {
        Self {
            items: Signal::new(Vec::new()),
            detail: Signal::new(None),
            is_loading: Signal::new(false),
            detail_is_loading: Signal::new(false),
            error: Signal::new(None),
            pending_track: Signal::new(None),
        }
    }

    pub fn load_detail(&self, playlist_id: i32) {
        let mut detail = self.detail;
        let mut is_loading = self.detail_is_loading;
        let mut error = self.error;

        is_loading.set(true);
        error.set(None);
        detail.set(None);

        spawn(async move {
            match repository::playlist::detail(playlist_id).await {
                Ok(fetched) => detail.set(Some(fetched)),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }

    pub fn fetch_all(&self) {
        if !self.items.read().is_empty() {
            return;
        }

        let mut items = self.items;
        let mut is_loading = self.is_loading;
        let mut error = self.error;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match repository::playlist::list().await {
                Ok(fetched) => items.set(fetched),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }

    pub fn open_add(&self, item: Item) {
        let mut error = self.error;
        let mut pending_track = self.pending_track;
        error.set(None);
        pending_track.set(Some(item));
    }

    pub fn open_create(&self) {
        let mut error = self.error;
        let mut pending_track = self.pending_track;
        error.set(None);
        pending_track.set(None);
        spawn(async move {
            let _ = document::eval("create_playlist.showModal()").await;
        });
    }

    pub fn add_pending(&self, playlist_id: i32) {
        let Some(item) = self.pending_track.read().clone() else {
            return;
        };

        let Some(item_id) = item.id.as_string() else {
            return;
        };

        let mut items = self.items;
        let mut detail = self.detail;
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut pending_track = self.pending_track;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            let track = Track {
                track_id: item_id,
                title: item.snippet.title.clone(),
                channel_name: item.snippet.channel_title.clone(),
                thumbnail_url: item.snippet.thumbnails.high.url.clone(),
                ..Default::default()
            };

            match repository::playlist::add(track, playlist_id).await {
                Ok(_) => {
                    pending_track.set(None);
                    match repository::playlist::list().await {
                        Ok(fetched) => items.set(fetched),
                        Err(err) => error.set(Some(AlertProps::error(err))),
                    }
                    let in_detail = detail
                        .read()
                        .as_ref()
                        .is_some_and(|entry| entry.playlist.playlist_id == playlist_id);
                    if in_detail
                        && let Ok(fetched) = repository::playlist::detail(playlist_id).await
                    {
                        detail.set(Some(fetched));
                    }
                    error.set(Some(AlertProps::info("Added to playlist".to_string())));
                    let _ = document::eval("add_to_playlist.close()").await;
                }
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }

    pub fn create(&self, name: String) {
        let name = name.trim().to_string();
        let pending = self.pending_track.read().clone();

        let mut items = self.items;
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut pending_track = self.pending_track;

        error.set(None);

        if name.is_empty() {
            error.set(Some(AlertProps::warning(
                "The input should not empty".to_string(),
            )));
            return;
        }

        is_loading.set(true);

        spawn(async move {
            match repository::playlist::create(&name).await {
                Ok(playlist) => {
                    if let Some(item) = pending {
                        if let Some(item_id) = item.id.as_string() {
                            let track = Track {
                                track_id: item_id,
                                title: item.snippet.title.clone(),
                                channel_name: item.snippet.channel_title.clone(),
                                thumbnail_url: item.snippet.thumbnails.high.url.clone(),
                                ..Default::default()
                            };
                            if let Err(err) =
                                repository::playlist::add(track, playlist.playlist_id).await
                            {
                                error.set(Some(AlertProps::error(err)));
                            }
                        }
                        pending_track.set(None);
                    }
                    match repository::playlist::list().await {
                        Ok(fetched) => items.set(fetched),
                        Err(err) => error.set(Some(AlertProps::error(err))),
                    }
                    let _ =
                        document::eval("create_playlist.close(); add_to_playlist.close()").await;
                }
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }

    pub fn remove(&self, playlist_id: i32) {
        let mut items = self.items;
        let mut detail = self.detail;
        let mut is_loading = self.is_loading;
        let mut error = self.error;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match repository::playlist::remove(playlist_id).await {
                Ok(_) => {
                    let is_current = detail
                        .read()
                        .as_ref()
                        .is_some_and(|d| d.playlist.playlist_id == playlist_id);
                    if is_current {
                        detail.set(None);
                    }
                    match repository::playlist::list().await {
                        Ok(fetched) => items.set(fetched),
                        Err(err) => error.set(Some(AlertProps::error(err))),
                    }
                }
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }

    pub fn remove_track(&self, playlist_id: i32, track_id: String) {
        let mut items = self.items;
        let mut detail = self.detail;
        let mut is_loading = self.is_loading;
        let mut error = self.error;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match repository::playlist::remove_track(playlist_id, &track_id).await {
                Ok(_) => {
                    match repository::playlist::list().await {
                        Ok(fetched) => items.set(fetched),
                        Err(err) => error.set(Some(AlertProps::error(err))),
                    }
                    let in_detail = detail
                        .read()
                        .as_ref()
                        .is_some_and(|entry| entry.playlist.playlist_id == playlist_id);
                    if in_detail
                        && let Ok(fetched) = repository::playlist::detail(playlist_id).await
                    {
                        detail.set(Some(fetched));
                    }
                }
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }
}
