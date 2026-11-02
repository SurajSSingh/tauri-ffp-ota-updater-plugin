// `app.withGlobalTauri` is enabled in tauri.conf.json, so the Tauri API is
// available on `window.__TAURI__` and no bundler is required.
const { invoke } = window.__TAURI__.core;

const form = document.querySelector("#ping-form");
const input = document.querySelector("#ping-input");
const output = document.querySelector("#output");

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  try {
    const response = await invoke("plugin:ffp-ota-updater|ping", {
      payload: { value: input.value },
    });
    output.textContent = `Plugin responded: ${JSON.stringify(response)}`;
  } catch (error) {
    output.textContent = `Error: ${error}`;
  }
});
