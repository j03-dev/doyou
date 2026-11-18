use anyhow::Result;
use rusql_alchemy::prelude::*;
use yt::data_api::types::Item;

use super::db;
use super::db::models::{History, Track};
use super::ensure_track;
use super::identity;

struct Plays {
    track: Track,
    representative_count: Integer,
    play_count: Integer,
    last_played_at: Float,
}

pub async fn record(item: &Item) -> Result<()> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    let track = ensure_track(item).await?;
    let now = now();

    match History::get(kwargs!(track_id = track.id.clone()), conn)
        .await
        .map_err(anyhow::Error::from_boxed)?
    {
        Some(mut history) => {
            history.play_count += 1;
            history.last_played_at = now;
            history
                .update(conn)
                .await
                .map_err(anyhow::Error::from_boxed)?;
        }
        None => {
            History::create(
                kwargs!(
                    track_id = track.id.clone(),
                    play_count = 1,
                    last_played_at = now
                ),
                conn,
            )
            .await
            .map_err(anyhow::Error::from_boxed)?;
        }
    }

    Ok(())
}

pub async fn recently_played(limit: usize) -> Result<Vec<Track>> {
    let mut plays = grouped_plays().await?;
    plays.sort_by(|a, b| b.last_played_at.total_cmp(&a.last_played_at));
    Ok(plays
        .into_iter()
        .take(limit)
        .map(|play| play.track)
        .collect())
}

pub async fn most_played(limit: usize) -> Result<Vec<Track>> {
    let mut plays = grouped_plays().await?;
    plays.sort_by(|a, b| {
        b.play_count
            .cmp(&a.play_count)
            .then_with(|| b.last_played_at.total_cmp(&a.last_played_at))
    });
    Ok(plays
        .into_iter()
        .take(limit)
        .map(|play| play.track)
        .collect())
}

/// Groups every variant of the same song into one entry, summing play counts
/// and keeping the most useful representative (downloads first).
async fn grouped_plays() -> Result<Vec<Plays>> {
    let history = all_history().await?;
    let tracks = all_tracks().await?;
    let mut grouped: Vec<Plays> = Vec::new();

    for entry in history {
        let Some(track) = tracks.iter().find(|track| track.id == entry.track_id) else {
            continue;
        };

        if let Some(plays) = grouped
            .iter_mut()
            .find(|plays| identity::same_media(&plays.track, track))
        {
            plays.play_count += entry.play_count;
            plays.last_played_at = plays.last_played_at.max(entry.last_played_at);
            if better_representative(
                track,
                &plays.track,
                entry.play_count,
                plays.representative_count,
            ) {
                plays.track = track.clone();
                plays.representative_count = entry.play_count;
            }
        } else {
            grouped.push(Plays {
                track: track.clone(),
                representative_count: entry.play_count,
                play_count: entry.play_count,
                last_played_at: entry.last_played_at,
            });
        }
    }

    Ok(grouped)
}

fn better_representative(
    candidate: &Track,
    current: &Track,
    candidate_count: Integer,
    current_count: Integer,
) -> bool {
    let candidate_rank = identity::rank(candidate);
    let current_rank = identity::rank(current);
    candidate_rank > current_rank
        || (candidate_rank == current_rank && candidate_count > current_count)
}

async fn all_history() -> Result<Vec<History>> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    History::all(conn).await.map_err(anyhow::Error::from_boxed)
}

async fn all_tracks() -> Result<Vec<Track>> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    Track::all(conn).await.map_err(anyhow::Error::from_boxed)
}

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .unwrap_or(0.0)
}
