import './style.css';
// @ts-ignore
import App from './routes/App/App.svelte';
import { appWindow } from '@tauri-apps/api/window';
import { register } from '@tauri-apps/api/globalShortcut';
import { appDataDir, join, resolveResource } from '@tauri-apps/api/path';
import { readTextFile } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api/tauri';
import { preferences, paths } from './cache';
import { listen } from '@tauri-apps/api/event';

// Create the app
const app = new App({
  target: document.getElementById('app'),
});

const fetchPreferencesData = async () => {
  const preferencesData = await readTextFile(
    await join(paths.get('appDataDirPath'), `preferences.json`)
  ).then(data => JSON.parse(data));
  Object.keys(preferencesData).forEach(key => {
    preferences.set(key, preferencesData[key]);
  });
};

const reloadTheme = async () => {
  const theme = await readTextFile(
    await join(paths.get('appDataDirPath'), `theme.json`)
  ).then(data => JSON.parse(data));
  // @ts-ignore
  const style = document.styleSheets[0].cssRules[0].style;
  style.setProperty('--primary-bg-color', theme.primary_bg_color);
  style.setProperty('--secondary-bg-color', theme.secondary_bg_color);
  style.setProperty('--primary-text-color', theme.primary_text_color);
  style.setProperty('--secondary-text-color', theme.secondary_text_color);
  style.setProperty('--primary-accent-color', theme.primary_accent_color);
  style.setProperty('--secondary-accent-color', theme.secondary_accent_color);
};

(async () => {
  // get and set values
  paths.set('appDataDirPath', await appDataDir());
  await fetchPreferencesData();
  await reloadTheme();

  document.addEventListener('keydown', event => {
    if (event.key === 'Escape') {
      appWindow.hide();
    }
  });

  // Listen for Menu Bar event to open preferences emitted from main.rs
  await listen('PreferencesClicked', data => {
    app.$set({
      appState: {
        app: false,
        settings: true,
      },
    });
  });

  await invoke('launch_on_login', {
    enable: preferences.get('launch_on_login'),
  });

  await listenForHotkey(preferences.get('shortcut'));
})();

export async function listenForHotkey(shortcut: string) {
  await register(shortcut, async () => {
    if (document.hasFocus()) {
      await appWindow.hide();
    } else {
      await appWindow.show();
      await appWindow.center();
      await appWindow.setFocus();
      document.getElementById('searchBarInput').focus();
    }
  });
}

export default app;
