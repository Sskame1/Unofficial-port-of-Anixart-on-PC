(() => {
  const tabs = [
    { id: "home", icon: "home", label: "Главная" },
    { id: "discover", icon: "discover", label: "Обзор" },
    { id: "bookmarks", icon: "bookmark", label: "Закладки" },
    { id: "feed", icon: "feed", label: "Лента" },
    { id: "profile", icon: "profile", label: "Профиль" },
  ];

  const paths = {
    home: '<path d="M3 10.8 12 3l9 7.8v9.7a1.5 1.5 0 0 1-1.5 1.5h-5.2v-6.5h-4.6V22H4.5A1.5 1.5 0 0 1 3 20.5z"/>',
    discover: '<circle cx="12" cy="12" r="9"/><path d="m15.8 8.2-2.5 5.1-5.1 2.5 2.5-5.1z"/>',
    bookmark: '<path d="M6 4.5A1.5 1.5 0 0 1 7.5 3h9A1.5 1.5 0 0 1 18 4.5V22l-6-3.8L6 22z"/>',
    feed: '<rect x="4" y="3" width="16" height="18" rx="2"/><path d="M8 8h8M8 12h8M8 16h5"/>',
    profile: '<circle cx="12" cy="8" r="3.5"/><path d="M4.5 21a7.5 7.5 0 0 1 15 0z"/>',
  };

  function renderNavigation(activeRoute) {
    const nav = document.getElementById("bottom-nav");
    nav.replaceChildren(...tabs.map(tab => {
      const button = document.createElement("button");
      button.type = "button";
      button.className = "nav-item";
      button.dataset.route = tab.id;
      button.setAttribute("aria-label", tab.label);
      button.setAttribute("aria-current", tab.id === activeRoute ? "page" : "false");
      const icon = document.createElement("span");
      icon.className = "nav-icon";
      icon.setAttribute("aria-hidden", "true");
      icon.innerHTML = `<svg viewBox="0 0 24 24" aria-hidden="true">${paths[tab.icon]}</svg>`;
      const label = document.createElement("span");
      label.className = "nav-label";
      label.textContent = tab.label;
      button.append(icon, label);
      button.addEventListener("click", () => window.app.navigate(tab.id));
      return button;
    }));
  }

  window.appNavigation = { render: renderNavigation };
})();
