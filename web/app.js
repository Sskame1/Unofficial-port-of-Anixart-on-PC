(() => {
  let currentRoute = "home";
  const volatileStorage = new Map();

  function storageGet(key) {
    try { return window.localStorage.getItem(key); } catch (_) { return volatileStorage.get(key) ?? null; }
  }

  function storageSet(key, value) {
    try { window.localStorage.setItem(key, value); } catch (_) { volatileStorage.set(key, String(value)); }
  }

  function readPreference(key, fallback) {
    return storageGet(key) || fallback;
  }

  function writePreference(key, value) {
    storageSet(key, value);
  }

  function navigate(route) {
    currentRoute = route;
    document.documentElement.dataset.route = route;
    if (window.appNavigation) {
      window.appNavigation.updateActive(route);
    }
    showRoute(route);
  }

  function refreshCurrent() {
    const refreshers = {
      home: "refreshHome",
      discover: "refreshDiscover",
      search: "refreshSearch",
      filter: "refreshFilter",
      bookmarks: "refreshBookmarks",
      feed: "refreshFeed",
      profile: "refreshProfile",
      collections: "refreshCollections",
      schedule: "refreshSchedule"
    };
    const refresh = window.appScreens?.[refreshers[currentRoute]];
    if (refresh) refresh();
    else navigate(currentRoute);
  }

  function showRoute(route) {
    if (route === "home" && window.appScreens.home) {
      window.appScreens.home();
    } else if (route === "discover" && window.appScreens.discover) {
      window.appScreens.discover();
    } else if (route === "search" && window.appScreens.search) {
      window.appScreens.search();
    } else if (route === "release" && window.releaseDetailScreen) {
      window.releaseDetailScreen.render(window.app.currentRelease);
    } else if (route === "watch" && window.releaseDetailScreen) {
      window.releaseDetailScreen.renderWatch(window.app.currentRelease);
    } else if (route === "player" && window.releaseDetailScreen) {
      window.releaseDetailScreen.renderPlayer(window.app.currentRelease, window.app.currentEpisode);
    } else if (route === "bookmarks" && window.appScreens.bookmarks) {
      window.appScreens.bookmarks();
    } else if (route === "feed" && window.appScreens.feed) {
      window.appScreens.feed();
    } else if (route === "profile" && window.appScreens.profile) {
      if (window.appStorage?.get("anixart_token")) window.appIpc.send({ action: "refresh_profile" });
      window.appScreens.profile();
    } else if (route === "settings" && window.appScreens.settings) {
      window.appScreens.settings();
    } else if (route === "schedule" && window.appScreens.schedule) {
      window.appScreens.schedule();
    } else if (route === "filter" && window.appScreens.filter) {
      window.appScreens.filter();
    } else if (route === "collections" && window.appScreens.collections) {
      window.appScreens.collections();
    } else if (route === "notifications" && window.appScreens.notifications) {
      window.appScreens.notifications();
    } else if (["signin", "signup", "restore"].includes(route) && window.appScreens.auth) {
      window.appScreens.auth(route);
    } else if (window.appScreens.renderPlaceholder) {
      window.appScreens.renderPlaceholder(route);
    }
  }

  function receive(message) {
    if (!message || !message.event) return;

    if (message.event === "navigate") {
      navigate(message.payload?.route || "home");
    } else if (message.event === "ready") {
      if (message.payload?.session) {
        storageSet("anixart_session", JSON.stringify(message.payload.session));
        storageSet("anixart_token", message.payload.session.token);
      }
      navigate(message.payload?.route || "home");
    } else if (message.event === "profile_data") {
      window.app.profileData = message.payload || null;
      if (currentRoute === "profile") window.appScreens.profile?.();
    } else if (message.event === "profile_error") {
      window.app.profileError = message.payload?.message || "Не удалось загрузить профиль";
      if (currentRoute === "profile") window.appScreens.profile?.();
    } else if (message.event === "friends_data") {
      window.appScreens.showFriends?.(message.payload || {});
    } else if (message.event === "friends_error") {
      window.appScreens.showFriendsError?.(message.payload?.message || "Не удалось загрузить друзей");
    } else if (message.event === "article_data") {
      window.appScreens.showArticle?.(message.payload || {});
    } else if (message.event === "article_error") {
      window.appScreens.showArticleError?.(message.payload?.message || "Не удалось открыть публикацию");
    } else if (message.event === "bookmarks_data" || message.event === "history_data") {
      if (currentRoute === "bookmarks") window.appScreens.showBookmarks?.(message.payload || {});
    } else if (message.event === "bookmarks_error" || message.event === "history_error") {
      if (currentRoute === "bookmarks") window.appScreens.showBookmarksError?.(message.payload?.message || "Не удалось загрузить список");
    } else if (message.event === "feed_data") {
      if (currentRoute === "feed") window.appScreens.showFeed?.(message.payload || {});
    } else if (message.event === "feed_error") {
      if (currentRoute === "feed" && (!message.payload?.request_id || message.payload.request_id === window.appFeedRequestId)) window.appScreens.showFeedError?.(message.payload?.message || "Не удалось загрузить ленту");
    } else if (message.event === "collections_data") {
      if (currentRoute === "collections" && (!message.payload?.request_id || message.payload.request_id === window.appCollectionRequestId)) window.appScreens.showCollections?.(message.payload || {});
    } else if (message.event === "collections_error") {
      if (currentRoute === "collections" && (!message.payload?.request_id || message.payload.request_id === window.appCollectionRequestId)) window.appScreens.showCollectionsError?.(message.payload?.message || "Не удалось загрузить коллекции");
    } else if (message.event === "collection_data") {
      if (currentRoute === "collections" && (!message.payload?.request_id || message.payload.request_id === window.appCollectionRequestId)) window.appScreens.showCollection?.(message.payload || {});
    } else if (message.event === "account_error") {
      window.app.setStatus("Аккаунт", message.payload?.message || "Требуется авторизация");
    } else if (message.event === "discover_page") {
      window.appScreens.showDiscoverPage?.(message.payload || {});
    } else if (message.event === "discover_page_error") {
      window.appScreens.showDiscoverError?.(message.payload?.message || "Ошибка загрузки");
    } else if (message.event === "search_results") {
      window.appScreens.showSearchResults?.(message.payload || []);
    } else if (message.event === "search_error") {
      window.appScreens.showSearchError?.(message.payload?.message || "Ошибка поиска");
    } else if (message.event === "release_detail") {
      window.app.currentRelease = message.payload || {};
      window.app.navigate("release");
    } else if (message.event === "episode_types") {
      window.releaseDetailScreen?.showTypes(message.payload || []);
    } else if (message.event === "episode_sources") {
      window.releaseDetailScreen?.showSources(message.payload || []);
    } else if (message.event === "episodes") {
      window.releaseDetailScreen?.showEpisodes(message.payload || []);
    } else if (message.event === "episode") {
      window.releaseDetailScreen?.showEpisode(message.payload || {});
    } else if (message.event === "episode_error") {
      if (currentRoute === "player") window.releaseDetailScreen?.showPlayerError(message.payload?.message || "Не удалось открыть эпизод.");
      else window.releaseDetailScreen?.showError(message.payload?.message || "Не удалось открыть эпизод.");
    } else if (message.event === "feature_error") {
      window.releaseDetailScreen?.showError(message.payload?.message || "Ошибка загрузки");
    } else if (message.event === "release_comments_data") {
      window.releaseDetailScreen?.showComments(message.payload || {});
    } else if (message.event === "release_comments_error") {
      window.releaseDetailScreen?.showCommentsError(message.payload?.message || "Не удалось загрузить отзывы");
    } else if (message.event === "schedule_data") {
      window.appScreens.showSchedule?.(message.payload || {});
    } else if (message.event === "schedule_error") {
      window.appScreens.showScheduleError?.(message.payload?.message || "Ошибка загрузки");
    } else if (message.event === "filter_results") {
      if (currentRoute === "home") {
        window.appScreens.showHomeResults?.(message.payload || {});
      } else {
        window.appScreens.showFilterResults?.(message.payload || {});
      }
    } else if (message.event === "filter_error") {
      const homeResults = document.getElementById("home-results");
      if (homeResults) {
        const error = document.createElement("div");
        error.className = "placeholder";
        error.textContent = message.payload?.message || "Не удалось загрузить каталог";
        homeResults.replaceChildren(error);
      }
      window.appScreens.showFilterError?.(message.payload?.message || "Ошибка фильтра");
    } else if (message.event === "types_data") {
      window.appScreens.loadFilterTypes?.(message.payload || []);
    } else if (message.event === "auth_success") {
      if (message.payload?.token) {
        storageSet("anixart_token", message.payload.token);
        storageSet("anixart_session", JSON.stringify(message.payload));
      }
      navigate("profile");
    } else if (message.event === "auth_error") {
      const submit = document.getElementById("auth-submit");
      if (submit) { submit.disabled = false; submit.textContent = "Войти"; }
      const error = message.payload?.message || "Не удалось войти";
      if (window.appScreens.showAuthError) window.appScreens.showAuthError(error);
      else window.app.setStatus("Авторизация", error);
    } else if (message.event === "signed_out") {
      storageSet("anixart_token", "");
      storageSet("anixart_session", "");
      window.app.profileData = null;
      window.app.profileError = null;
      navigate("signin");
    }
    else if (message.event === "collection_favorite_done") window.appScreens.collectionFavoriteDone?.(message.payload || {});
    else if (message.event === "collection_favorite_error") window.appScreens.collectionFavoriteError?.(message.payload?.message || "Не удалось изменить закладку");
    else if (message.event === "release_list_done") window.releaseDetailScreen?.listDone();
    else if (message.event === "release_list_error") window.releaseDetailScreen?.listError(message.payload?.message);
  }

  window.app = {
    navigate,
    receive,
    currentRelease: null,
    currentEpisode: null,
    profileData: null,
    profileError: null,
    backFromRelease() {
      navigate("discover");
    },
    auth(route = "signin") {
      navigate(route);
    },
    setTheme(theme) {
      const nextTheme = theme === "dark" ? "dark" : "light";
      document.documentElement.setAttribute("data-theme", nextTheme);
      writePreference("theme", nextTheme);
      updateThemeControls();
    },
    toggleTheme() {
      const current = document.documentElement.getAttribute("data-theme") === "dark" ? "dark" : "light";
      window.app.setTheme(current === "dark" ? "light" : "dark");
    },
    refresh: refreshCurrent,
    setStatus(title, message) {
      window.alert(`${title}: ${message}`);
    }
  };

  window.appStorage = { get: storageGet, set: storageSet };

  function updateThemeControls() {
    const isDark = document.documentElement.getAttribute("data-theme") === "dark";
    const themeToggle = document.getElementById("theme-toggle");
    if (!themeToggle) return;
    themeToggle.classList.toggle("is-dark", isDark);
    themeToggle.classList.toggle("is-light", !isDark);
    themeToggle.setAttribute("aria-label", isDark ? "Включить светлую тему" : "Включить темную тему");
  }

  function initializeApp() {
    const savedTheme = readPreference("theme", "dark");
    document.documentElement.setAttribute("data-theme", savedTheme);

    // Рендерим навигацию
    if (window.appNavigation) {
      window.appNavigation.render();
    }

    const themeToggle = document.getElementById("theme-toggle");
    if (themeToggle && themeToggle.dataset.bound !== "true") {
      themeToggle.addEventListener("click", () => window.app.toggleTheme());
      themeToggle.dataset.bound = "true";
    }

    document.querySelector(".sidebar-footer [data-route=\"settings\"]")?.addEventListener("click", () => navigate("settings"));
    updateThemeControls();
    document.getElementById("refresh-current")?.addEventListener("click", refreshCurrent);
    window.addEventListener("keydown", event => {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "r") { event.preventDefault(); refreshCurrent(); }
    });
    let lastAutoRefresh = Date.now();
    const refreshOnReturn = () => {
      if (document.visibilityState === "hidden" || Date.now() - lastAutoRefresh < 60_000) return;
      lastAutoRefresh = Date.now();
      refreshCurrent();
    };
    document.addEventListener("visibilitychange", refreshOnReturn);
    window.addEventListener("focus", refreshOnReturn);

    // Показываем начальный маршрут
    navigate("home");
  }

  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", initializeApp, { once: true });
  else initializeApp();
})();
