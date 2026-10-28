(() => {
  const prefix = "anixart_playback:";
  const nativeProgress = Object.create(null);
  function key(releaseId, typeId, sourceId, position) {
    return `${prefix}${releaseId}:${typeId}:${sourceId}:${position}`;
  }
  function read(releaseId, typeId, sourceId, position) {
    const storageKey = key(releaseId, typeId, sourceId, position);
    try {
      const local = JSON.parse(localStorage.getItem(storageKey) || "null");
      if (local) return local;
    } catch (_) {}
    return nativeProgress[storageKey.slice(prefix.length)] || null;
  }
  function write(releaseId, typeId, sourceId, position, progress) {
    const storageKey = key(releaseId, typeId, sourceId, position);
    nativeProgress[storageKey.slice(prefix.length)] = progress;
    try { localStorage.setItem(storageKey, JSON.stringify(progress)); } catch (_) {}
  }
  function mergeEpisode(release, type, source, episode) {
    const local = read(release, type, source, episode.position);
    if (!local) return episode;
    return { ...episode, playback_position: Number(local.position) || Number(episode.playback_position) || 0, is_watched: Boolean(local.watched || episode.is_watched) };
  }
  function markOpened(release, type, source, position) {
    const current = read(release, type, source, position) || {};
    const progress = { position: Number(current.position) || 0, watched: true };
    write(release, type, source, position, progress);
    window.appIpc.send({ action: "watch_episode", release_id: release, source_id: source, position });
    window.appIpc.send({ action: "save_episode_progress", release_id: release, type_id: type, source_id: source, position, playback_position: progress.position, watched: true });
    return progress;
  }
  function save(release, type, source, position, seconds, watched = false) {
    const progress = { position: Math.max(0, Math.floor(Number(seconds) || 0)), watched: Boolean(watched) };
    write(release, type, source, position, progress);
    window.appIpc.send({ action: "save_episode_progress", release_id: release, type_id: type, source_id: source, position, playback_position: progress.position, watched: progress.watched });
  }
  function load(progress) {
    Object.entries(progress || {}).forEach(([storageKey, value]) => {
      nativeProgress[storageKey] = value;
      try { localStorage.setItem(`${prefix}${storageKey}`, JSON.stringify(value)); } catch (_) {}
    });
  }
  window.appPlayback = { read, write, mergeEpisode, markOpened, save, load };
})();
