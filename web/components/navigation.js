(() => {
  const icons = {
    home: '<svg aria-hidden="true" viewBox="0 0 24 24"><path d="m3 10 9-7 9 7"/><path d="M5 9v11h14V9"/><path d="M9 20v-6h6v6"/></svg>',
    discover: '<svg aria-hidden="true" viewBox="0 0 24 24"><circle cx="11" cy="11" r="6.5"/><path d="m16 16 4.5 4.5"/></svg>',
    bookmarks: '<svg aria-hidden="true" viewBox="0 0 24 24"><path d="M6 4.5A2.5 2.5 0 0 1 8.5 2h7A2.5 2.5 0 0 1 18 4.5V21l-6-3.5L6 21z"/></svg>',
    feed: '<svg aria-hidden="true" viewBox="0 0 24 24"><rect x="4" y="3" width="16" height="18" rx="2"/><path d="M8 7h8M8 11h8M8 15h5"/></svg>',
    profile: '<svg aria-hidden="true" viewBox="0 0 24 24"><circle cx="12" cy="8" r="3.5"/><path d="M5 21a7 7 0 0 1 14 0"/></svg>'
  };
  const navItems = [
    { route: "home", label: "Главная" },
    { route: "discover", label: "Обзор" },
    { route: "bookmarks", label: "Закладки" },
    { route: "feed", label: "Лента" },
    { route: "profile", label: "Профиль" }
  ];

  function renderNavigation() {
    const nav = document.getElementById("sidebar-nav");
    if (!nav) return;

    if (!nav.children.length) {
      nav.innerHTML = navItems.map(item => `
        <button class="nav-item" type="button" data-route="${item.route}">
          <span class="nav-item-icon">${icons[item.route]}</span>
          <span>${item.label}</span>
        </button>
      `).join("");
    }

    if (nav.dataset.bound !== "true") {
      nav.addEventListener("click", event => {
        const button = event.target.closest("button[data-route]");
        if (button) window.app.navigate(button.dataset.route);
      });
      nav.dataset.bound = "true";
    }

    updateActiveNav(document.documentElement.dataset.route || "home");
  }

  function updateActiveNav(route) {
    document.querySelectorAll(".nav-item").forEach(item => {
      const itemRoute = item.dataset.route;
      item.classList.toggle("active", itemRoute === route);
    });
  }

  window.appNavigation = { render: renderNavigation, updateActive: updateActiveNav };
})();
