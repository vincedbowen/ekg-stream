use crate::state::{AppState, SessionState, Shared};
use ingest_proto::ecg::ingest::v1::{Sample, SampleBatch};
use tokio::sync::RwLockWriteGuard;
use tracing::warn;

/// Minimal processing/validation for a single batch.
/// Returns (received, dropped, last_seq, warning)
pub async fn process_batch(shared: &Shared, batch: SampleBatch) -> (u64, u64, u64, String) {
    let session_id = batch.session_id.clone();
    let mut received = 0u64;
    let mut dropped = 0u64;
    let mut warning = String::new();

    // quick structural checks
    if batch.samples.is_empty() {
        return (0, 0, current_last_seq(&shared, &session_id).await, "empty batch".into());
    }
    if batch.samples.len() > 2000 {
        warning = format!("batch too large: {}", batch.samples.len());
    }

    // Validate monotonicity (seq & t_s) within batch
    let mut prev: Option<&Sample> = None;
    for (i, s) in batch.samples.iter().enumerate() {
        // numeric sanity
        if !s.t_s.is_finite() || !s.mv.is_finite() {
            warning = format!("non-finite at index {}", i);
            dropped += 1;
            continue;
        }
        if let Some(p) = prev {
            if s.seq <= p.seq {
                warning = format!("non-monotonic seq at index {}", i);
                dropped += 1;
                continue;
            }
            if s.t_s <= p.t_s {
                warning = format!("non-monotonic t_s at index {}", i);
                dropped += 1;
                continue;
            }
        }
        prev = Some(s);
        received += 1;
    }

    // Update session state
    let last_seq = if let Some(last) = batch.samples.iter().filter(|s| s.seq > 0).map(|s| s.seq).max() {
        update_session(shared, &session_id, last, received, dropped).await
    } else {
        current_last_seq(&shared, &session_id).await
    };


    if batch.samples.len() >= 2 {
        let first = &batch.samples.first().unwrap();
        let last = &batch.samples.last().unwrap();
        let dt = last.t_s - first.t_s;
        if dt > 0.0 {
            let observed_fs = (batch.samples.len() as f64 - 1.0) / dt;
            let fs = batch.fs_hz;
            let drift = ((observed_fs - fs).abs() / fs) * 100.0;
            if drift > 2.0 && warning.is_empty() {
                warning = format!("fs drift {:.1}%", drift);
            }
        }
    }

    (received, dropped, last_seq, warning)
}

async fn update_session(shared: &Shared, session_id: &str, last_seq: u64, rec: u64, dropc: u64) -> u64 {
    let mut guard = shared.write().await;
    let s: &mut SessionState = guard.sessions.entry(session_id.to_string())
        .or_insert_with(SessionState::default);
    if last_seq < s.last_seq {
        warn!(session = session_id, last_seq, prev = s.last_seq, "received older seq than current; keeping previous");
    } else {
        s.last_seq = last_seq;
    }
    s.received = s.received.saturating_add(rec);
    s.dropped = s.dropped.saturating_add(dropc);
    s.last_seq
}

async fn current_last_seq(shared: &Shared, session_id: &str) -> u64 {
    let guard = shared.read().await;
    guard.sessions.get(session_id).map(|s| s.last_seq).unwrap_or(0)
}
