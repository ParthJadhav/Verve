import "./style.css";
// @ts-ignore
import App from "./routes/App/App.svelte";
import { appWindow } from "@tauri-apps/api/window";
import { register } from '@tauri-apps/api/globalShortcut'
import { appDataDir, join, resolveResource } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";
import { preferences, paths } from "./cache";
import { listen } from '@tauri-apps/api/event'

// Create the app
const app = new App({
  target: document.getElementById("app"),
});

(async () => {
  // get and set values
  paths.set("appDataDirPath", await appDataDir());

  const preferencesData = await readTextFile(await join(paths.get("appDataDirPath"), `preferences.json`)).then((data) => JSON.parse(data));
  Object.keys(preferencesData).forEach((key) => {
    preferences.set(key, preferencesData[key]);
  });

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      appWindow.hide();
    }
  });

  // Listen for Menu Bar event to open preferences emitted from main.rs
  await listen("PreferencesClicked", (data) => {
    app.$set({
      appState: {
        app: false,
        settings: true
      }
    });
    appWindow.show();
  });


  await invoke("launch_on_login", {
    enable: preferences.get("launch_on_login"),
  });

  await changeAppHotkey(preferences.get("shortcut"));
})();

export async function changeAppHotkey(shortcut: string) {
  await register(shortcut, async () => {
    if (document.hasFocus()) {
      await appWindow.hide()
    } else {
      await appWindow.show()
      await appWindow.center()
      await appWindow.setFocus()
      document.getElementById('searchBarInput').focus()
    }
  })
}

export default app;
