(() => {
  let currentRoute = "home";
  let previousRoute = "discover";

  function showRoute(route) {
    if (route === "home") {
      window.appScreens.home();
    } else if (route === "discover") {
      window.appScreens.discover();
    } else if (route === "search") {
      window.appScreens.search();
    } else if (["signin", "signup", "restore", "verify"].includes(route)) {
      window.appScreens.auth(route);
    } else if (route === "bookmarks") {
      window.appScreens.bookmarks();
    } else if (route === "feed") {
      window.appScreens.feed();
    } else if (route === "profile") {
      window.appScreens.profile();
    } else if (route === "settings") {
      window.appScreens.settings();
    } else if (route === "notifications") {
      window.appScreens.notifications();
    } else if (route === "schedule") {
      window.appScreens.schedule();
    } else if (route === "filter") {
      window.appScreens.filter();
    } else if (route === "collections") {
      window.appScreens.collections();
    } else if (route === "release") {
      return;
    } else if (!window.appScreens.renderPlaceholder(route)) {
      setStatus("Неизвестный раздел", "Вернитесь на главный экран и попробуйте снова.");
      return;
    }
    currentRoute = route;
    document.getElementById("app")?.classList.toggle("auth-mode", ["signin", "signup", "restore", "verify"].includes(route));
    window.appNavigation.render(route);
    const hint = document.getElementById("search-hint");
    if (hint) hint.textContent = ({ home: "Поиск аниме", discover: "Поиск аниме", search: "Поиск аниме", bookmarks: "Поиск в закладках", feed: "Поиск записей и блогов", profile: "Поиск пользователей" })[route] || "Поиск аниме";
  }

  function setStatus(title, message) {
    let status = document.getElementById("toast-status");
    if (!status) {
      status = document.createElement("div");
      status.id = "toast-status";
      status.className = "toast-status";
      status.setAttribute("role", "status");
      document.body.append(status);
    }
    status.textContent = `${title}: ${message}`;
    status.classList.add("visible");
    const discoverButton = document.getElementById("discover-random");
    if (discoverButton) discoverButton.disabled = false;
  }

  function setEpisodePending(pending) {
    const button = document.getElementById("episode-submit");
    if (button) button.disabled = pending;
  }

  function receive(message) {
    if (!message || typeof message !== "object") return;
    if (message.event === "ready") {
      showRoute(message.payload?.route || currentRoute);
    } else if (message.event === "navigate") {
      showRoute(message.payload?.route);
    } else if (message.event === "status") {
      setEpisodePending(false);
      setStatus(message.payload?.title || "Статус", message.payload?.message || "");
    } else if (message.event === "episode") {
      setEpisodePending(false);
      setStatus(message.payload?.name || "Эпизод", "Открываем страницу источника.");
      const container = document.getElementById("player-container");
      if (container) {
        container.classList.remove("hidden");
        container.innerHTML = "";
        const frame = document.createElement("iframe");
        frame.className = "player-frame";
        frame.title = "Плеер эпизода";
        frame.allow = "autoplay; fullscreen; picture-in-picture";
        frame.allowFullscreen = true;
        frame.referrerPolicy = "strict-origin-when-cross-origin";
        frame.src = message.payload.url;
        container.append(frame);
      }
    } else if (message.event === "random_release") {
      if (currentRoute === "home") window.appScreens.showHomeRelease(message.payload || {});
      else window.appScreens.showRandomRelease(message.payload || {});
    } else if (message.event === "random_release_error") {
      if (currentRoute === "home") window.appScreens.showHomeError(message.payload?.message || "Ошибка запроса.");
      else window.appScreens.showRandomError(message.payload?.message || "Ошибка запроса.");
    } else if (message.event === "release_detail") {
      currentRoute = "release";
      window.appNavigation.render(previousRoute);
      window.releaseDetailScreen.render(message.payload || {});
    } else if (message.event === "episode_types") {
      window.releaseDetailScreen.showTypes(message.payload || []);
    } else if (message.event === "episode_sources") {
      window.releaseDetailScreen.showSources(message.payload || []);
    } else if (message.event === "episodes") {
      window.releaseDetailScreen.showEpisodes(message.payload || []);
    } else if (message.event === "feature_error") {
      if (currentRoute === "release") window.releaseDetailScreen.showError(message.payload?.message || "Ошибка запроса.");
      else setStatus("Ошибка", message.payload?.message || "Ошибка запроса.");
    } else if (message.event === "search_pending") {
      window.appScreens.showSearchPending(message.payload?.query || "");
    } else if (message.event === "search_results") {
      window.appScreens.showSearchResults(message.payload || []);
    } else if (message.event === "search_error") {
      window.appScreens.showSearchError(message.payload?.message || "Ошибка запроса.");
    } else if (message.event === "discover_page") {
      window.appScreens.showDiscoverPage(message.payload || {});
    } else if (message.event === "discover_page_error") {
      window.appScreens.showDiscoverError(message.payload?.message || "Ошибка запроса.");
    } else if (message.event === "schedule_data") {
      window.appScreens.showSchedule(message.payload || {});
    } else if (message.event === "schedule_error") {
      window.appScreens.showScheduleError(message.payload?.message || "Ошибка запроса.");
    } else if (message.event === "filter_results") {
      window.appScreens.showFilterResults(message.payload || {});
    } else if (message.event === "filter_error") {
      window.appScreens.showFilterError(message.payload?.message || "Ошибка запроса.");
    } else if (message.event === "types_data") {
      if (window.appScreens.loadFilterTypes) {
        window.appScreens.loadFilterTypes(message.payload || []);
      }
    } else if (message.event === "types_error") {
    } else if (message.event === "alert") {
      setEpisodePending(false);
      if (currentRoute === "search") {
        window.appScreens.showSearchError(message.payload?.message || "Неизвестный запрос.");
      } else if (["signin", "signup", "restore", "verify"].includes(currentRoute)) {
        window.appScreens.showAuthError(message.payload?.message || "Неизвестная ошибка.");
      } else {
        setStatus("Ошибка", message.payload?.message || "Неизвестный запрос.");
      }
    }
  }

  window.app = {
    navigate(route) { window.appIpc.send({ action: "navigate", route }); },
    auth(route = "signin") { showRoute(route); },
    openRelease(releaseId) { previousRoute = currentRoute; window.appIpc.send({ action: "open_release", release_id: releaseId }); },
    backFromRelease() { showRoute(previousRoute); },
    setStatus,
    setEpisodePending,
    receive,
  };

  document.addEventListener("DOMContentLoaded", () => {
    document.getElementById("global-search")?.addEventListener("click", () => window.app.navigate("search"));
    document.getElementById("open-settings")?.addEventListener("click", () => window.app.navigate("settings"));
    document.getElementById("open-notifications")?.addEventListener("click", () => window.app.navigate("notifications"));
    document.getElementById("screen-content")?.addEventListener("click", event => {
      const feature = event.target.closest(".home-feature[data-release-id]");
      if (feature) { window.app.openRelease(Number(feature.dataset.releaseId)); return; }
      const card = event.target.closest("[data-release-id]");
      if (card) window.app.openRelease(Number(card.dataset.releaseId));
    });
    document.getElementById("screen-content")?.addEventListener("keydown", event => {
      const feature = event.target.closest(".home-feature[data-release-id]");
      if (feature && (event.key === "Enter" || event.key === " ")) {
        event.preventDefault();
        window.app.openRelease(Number(feature.dataset.releaseId));
        return;
      }
      const card = event.target.closest('[data-release-id][role="button"]');
      if (card && (event.key === "Enter" || event.key === " ")) {
        event.preventDefault();
        window.app.openRelease(Number(card.dataset.releaseId));
      }
    });
    showRoute(currentRoute);
    window.appNavigation.render(currentRoute);
  });
})();
