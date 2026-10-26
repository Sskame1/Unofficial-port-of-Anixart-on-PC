(() => {
  function escape(value) {
    return String(value ?? "").replace(/[&<>"']/g, char => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[char]));
  }
  function formatMinutes(value) {
    const minutes = Math.round(Number(value || 0) / 60);
    if (minutes < 60) return `${minutes} мин.`;
    return `${Math.floor(minutes / 60)} ч ${minutes % 60} мин.`;
  }
  function renderChart(items) {
    if (!items?.length) return `<div class="profile-chart-empty">Нет данных активности</div>`;
    const recent = items.slice(-14);
    const values = recent.map(item => Number(item.count || 0));
    const max = Math.max(...values, 1);
    return `<div class="profile-chart" aria-label="Динамика просмотра">${values.map((value, index) => { const item = recent[index] || {}; const rawDate = item.date || item.day || item.timestamp; const label = typeof rawDate === "number" ? new Date(rawDate * 1000).toLocaleDateString("ru-RU", { day: "2-digit", month: "2-digit" }) : rawDate; return `<div class="profile-chart-column"><strong>${value}</strong><div class="profile-chart-bar" style="--height:${Math.max(8, Math.round(value / max * 100))}%" title="${value} серий"></div><span>${escape(label || "")}</span></div>`; }).join("")}</div>`;
  }
  function renderProfile() {
    const root = document.getElementById("screen-content");
    if (!window.appStorage?.get("anixart_token")) {
      root.innerHTML = `<div class="placeholder account-placeholder"><div class="placeholder-icon">👤</div><p>Войдите в аккаунт для просмотра профиля</p><button class="btn btn-primary" type="button" id="profile-login">Войти в аккаунт</button></div>`;
      root.querySelector("#profile-login")?.addEventListener("click", () => window.app.navigate("signin"));
      return;
    }
    const profile = window.app.profileData;
    if (!profile) {
      root.innerHTML = `<div class="profile-container"><div class="placeholder"><div class="placeholder-icon">👤</div><p>${escape(window.app.profileError || "Загружаем профиль…")}</p></div></div>`;
      return;
    }
    window.app.currentProfileId = profile.id;
    const hidden = profile.is_counts_hidden || profile.is_stats_hidden;
    const avatar = profile.avatar ? `<img src="${escape(profile.avatar)}" alt="${escape(profile.login)}" loading="eager" referrerpolicy="no-referrer">` : "👤";
    const stats = hidden ? `<div class="profile-section"><p>Статистика скрыта настройками приватности.</p></div>` : `<div class="profile-section"><h2>Статистика</h2><div class="profile-stats"><div class="stat-item"><div class="stat-value">${profile.watching_count ?? 0}</div><div class="stat-label">Смотрю</div></div><div class="stat-item"><div class="stat-value">${profile.plan_count ?? 0}</div><div class="stat-label">В планах</div></div><div class="stat-item"><div class="stat-value">${profile.completed_count ?? 0}</div><div class="stat-label">Завершено</div></div><div class="stat-item"><div class="stat-value">${profile.hold_on_count ?? 0}</div><div class="stat-label">Отложено</div></div><div class="stat-item"><div class="stat-value">${profile.dropped_count ?? 0}</div><div class="stat-label">Брошено</div></div><div class="stat-item"><div class="stat-value">${profile.watched_episode_count ?? 0}</div><div class="stat-label">Серий просмотрено</div></div></div><p class="profile-meta">Общее время просмотра: ${formatMinutes(profile.watched_time)}</p></div>`;
    const history = profile.history || [];
    const joined = profile.added_date || profile.register_date;
    root.innerHTML = `<div class="profile-container"><div class="profile-hero"><div class="profile-avatar">${avatar}</div><div class="profile-username"><span>${escape(profile.login || "Пользователь")}</span>${profile.is_verified ? `<span class="profile-badge" title="Подтверждённый профиль">✓</span>` : ""}</div><div class="profile-bio">${escape(profile.status || "")}</div><div class="profile-rating">Рейтинг: <strong>${profile.rating_score ?? "—"}</strong></div><div class="profile-joined">${joined ? `На проекте с ${new Date(joined * 1000).toLocaleDateString("ru-RU")}` : ""}</div><button class="profile-friends-btn" id="profile-friends" type="button">👥 Друзья (${profile.friend_count ?? 0})</button></div>${stats}<div class="profile-section"><h2>Динамика просмотра</h2>${renderChart(profile.watch_dynamics)}</div><div class="profile-section"><h2>Просмотрено недавно</h2>${history.length ? `<div class="release-grid profile-history-grid">${history.slice(0, 12).map(release => `<div data-history-id="${release.id}"></div>`).join("")}</div>` : `<div class="profile-chart-empty">История просмотров пуста</div>`}</div></div>`;
    root.querySelector("#profile-friends")?.addEventListener("click", () => {
      root.insertAdjacentHTML("beforeend", `<div class="friends-overlay" id="friends-overlay"><section class="friends-dialog"><header><h2>Друзья</h2><button type="button" aria-label="Закрыть" id="friends-close">×</button></header><div id="friends-content" class="friends-list"><p class="placeholder">Загружаем друзей…</p></div></section></div>`);
      root.querySelector("#friends-close")?.addEventListener("click", () => root.querySelector("#friends-overlay")?.remove());
      window.appIpc.send({ action: "load_friends", page: 1, profile_id: window.app.currentProfileId });
    });
    history.slice(0, 12).forEach(release => {
      const target = root.querySelector(`[data-history-id="${release.id}"]`);
      if (target) target.replaceWith(window.releaseCard.render(release));
    });
  }
  function showFriends(page) {
    const content = document.getElementById("friends-content");
    if (!content) return;
    const items = page.content || [];
    content.innerHTML = items.length ? items.map(friend => `<article class="friend-row">${friend.avatar ? `<img src="${escape(friend.avatar)}" alt="" loading="lazy" referrerpolicy="no-referrer">` : `<span>👤</span>`}<strong>${escape(friend.login || "Пользователь")}</strong>${friend.is_verified ? "✓" : ""}</article>`).join("") : `<p class="placeholder">Список друзей пуст.</p>`;
  }
  function showFriendsError(message) {
    const content = document.getElementById("friends-content");
    if (content) content.innerHTML = `<p class="placeholder">Не удалось загрузить друзей: ${escape(message)}</p>`;
  }
  window.appScreens = { ...(window.appScreens || {}), profile: renderProfile, showFriends, showFriendsError };
})();
