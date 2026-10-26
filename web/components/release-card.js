(() => {
  function renderReleaseCard(release, compact = false, clickable = true) {
    const article = document.createElement("article");
    article.className = `release-card panel${compact ? " release-card-compact" : ""}`;
    if (clickable) {
      article.tabIndex = 0;
      article.setAttribute("role", "button");
      article.setAttribute("aria-label", `Открыть релиз: ${release.title_ru || release.title_original || "Без названия"}`);
      article.dataset.releaseId = release.id;
    }

    const title = document.createElement("h2");
    title.className = "panel-title";
    title.textContent = release.title_ru || release.title_original || "Без названия";
    article.append(title);

    const imageUrl = release.image;
    if (typeof imageUrl === "string" && imageUrl.startsWith("https://")) {
      const poster = document.createElement("div");
      poster.className = "release-poster-frame";
      const image = document.createElement("img");
      image.className = "release-poster";
      image.src = imageUrl;
      image.alt = `Постер: ${title.textContent}`;
      image.loading = compact ? "lazy" : "eager";
      image.decoding = "async";
      image.referrerPolicy = "no-referrer";
      image.addEventListener("error", () => {
        poster.replaceChildren();
        poster.classList.add("release-poster-unavailable");
        poster.textContent = "Постер временно недоступен";
      }, { once: true });
      poster.append(image);
      article.append(poster);
    }

    const meta = document.createElement("p");
    meta.className = "muted release-meta";
    const details = [release.year, Number.isFinite(release.grade) && release.grade > 0 ? `★ ${release.grade.toFixed(1)}` : null]
      .filter(Boolean);
    meta.textContent = details.join(" · ");
    if (details.length) article.append(meta);

    if (release.description) {
      const description = document.createElement("p");
      description.className = "release-description";
      description.textContent = release.description;
      article.append(description);
    }

    const id = document.createElement("p");
    id.className = "muted release-id";
    id.textContent = `ID релиза: ${release.id}`;
    article.append(id);
    return article;
  }

  window.releaseCard = { render: renderReleaseCard };
})();
