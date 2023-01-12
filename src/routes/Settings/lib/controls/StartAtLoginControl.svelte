<script lang="ts">
  import Slider from '../CheckBox.svelte';
  import { preferences, paths } from '../../../../cache';
  import { writeTextFile } from '@tauri-apps/api/fs';
  import { invoke } from '@tauri-apps/api/tauri';
  export let startAtLogin: boolean = false;

  function updateLaunchAtLoginPreference() {
    const checkBox = document.getElementById(
      'launch-on-login-checkbox'
    ) as HTMLInputElement;
    if (preferences.get('launchAtLogin') === checkBox.checked) {
      return;
    }
    preferences.set('launch_on_login', checkBox.checked);
    writeTextFile(
      `${paths.get('appDataDirPath')}/preferences.json`,
      JSON.stringify(Object.fromEntries(preferences), null, 4)
    ).then(() => {
      console.log('Updated launch at login preference');
      invoke('launch_on_login', { enable: checkBox.checked });
    });
  }
</script>

<div class="shortcut-control">
  <p>Start at login:</p>
  <Slider
    id={'launch-on-login-checkbox'}
    isChecked={startAtLogin}
    onToggle={updateLaunchAtLoginPreference}
  />
</div>

<style>
  .shortcut-control {
    width: 30%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }
  p {
    font-size: 16px;
    color: var(--primary-text-color);
    margin: 0;
  }
</style>
