(() => {
  function send(command) {
    if (!window.ipc || typeof window.ipc.postMessage !== "function") {
      throw new Error("Native application connection is unavailable.");
    }
    window.ipc.postMessage(JSON.stringify(command));
  }

  window.appIpc = { send };
})();
