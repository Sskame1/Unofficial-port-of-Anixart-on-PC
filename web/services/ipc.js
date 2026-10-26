(() => {
  window.appIpc = {
    send(message) {
      if (window.ipc && window.ipc.postMessage) {
        window.ipc.postMessage(JSON.stringify(message));
      }
    }
  };

  window.addEventListener("message", event => {
    if (typeof event.data === "string") {
      try {
        const message = JSON.parse(event.data);
        if (window.app && window.app.receive) {
          window.app.receive(message);
        }
      } catch {}
    }
  });
})();
