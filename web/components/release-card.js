(() => {
  function render(release, onSelect) {
    const card = document.createElement("div");
    card.className = "release-card";
    card.dataset.releaseId = release.id;

    const poster = release.image || release.poster_url || release.poster || "";
    const title = release.title_ru || release.title_original || release.title || release.name || "Без названия";
    const episodes = release.episodes_total || release.episodes_released || release.episodes_count || release.episodes || 0;
    const year = release.year || "";

    card.innerHTML = `
      <div class="release-poster">
        ${poster ? `<img src="${poster}" alt="${title}" loading="lazy" referrerpolicy="no-referrer">` : `<div class="poster-empty">🎬</div>`}
        ${episodes > 0 ? `<div class="release-badge">${episodes} эп.</div>` : ""}
      </div>
      <div class="release-title">${title}</div>
      ${year ? `<div class="release-meta">${year}</div>` : ""}
    `;

    card.addEventListener("click", () => {
      if (typeof onSelect === "function") {
        onSelect(release);
        return;
      }
      const targetId = Number(release.action || release.id);
      if (targetId > 0) window.appIpc.send({ action: "open_release", release_id: targetId });
    });

    return card;
  }

  window.releaseCard = { render };
})();
