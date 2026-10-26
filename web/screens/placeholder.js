(() => {
  const screens = {
    collections: { title: "Коллекции", icon: "▣", text: "Каталог подборок пока не подключён к публичному API." },
  };

  function renderPlaceholder(route) {
    const screen = screens[route];
    if (!screen) return false;
    document.getElementById("screen-content").innerHTML = `
      <section class="screen secondary-screen" aria-labelledby="screen-title">
        <h1 class="screen-title" id="screen-title">${screen.title}</h1>
        <div class="placeholder">
          <div class="placeholder-icon" aria-hidden="true">${screen.icon}</div>
          <strong>${screen.title}</strong>
          <span>${screen.text}</span>
        </div>
      </section>`;
    return true;
  }

  window.appScreens = { ...(window.appScreens || {}), renderPlaceholder };
})();
