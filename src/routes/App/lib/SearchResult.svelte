<script lang="ts">
  export let results: string[];

  import { appDataDir, join, resolveResource } from "@tauri-apps/api/path";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { appWindow, LogicalSize } from "@tauri-apps/api/window";
  import { afterUpdate } from "svelte";
  import { preferences } from "../../../cache.js";

  afterUpdate(async () => {
    const height = document.getElementsByClassName("container")[0].clientHeight;
    await appWindow.setSize(new LogicalSize(750, height));
    if (results.length > 0 && results[0] !== "") {
      const firstResult = document.getElementById(results[0]);
      await firstResult.focus();
    }
  });

  async function getIcon(app_name: string) {
    const fallbackIcon = convertFileSrc(
      await resolveResource("assets/default.svg")
    );
    if (
      [
        "Migration Assistant",
        "System Information",
        "Calendar",
        "System Settings",
        "Photo Booth",
        "AirPort Utility",
      ].includes(app_name)
    ) {
      const icon_path = await resolveResource(
        `assets/appIcons/${app_name}.app.png`
      );
      return { icon: convertFileSrc(icon_path), fallbackIcon };
    }
    const appDataDirPath = await appDataDir();
    const filePath = await join(appDataDirPath, `appIcons/${app_name}.app.png`);
    return { icon: convertFileSrc(filePath), fallbackIcon };
  }

  async function handleKeydown(event) {
    if (event.keyCode == 38 || event.keyCode == 40) {
      const current = document.activeElement as HTMLElement | null;
      const items = [...document.getElementsByClassName("searchResult")] as
        | HTMLElement[]
        | null;
      const currentIndex = items.indexOf(current);
      let newIndex;

      if (currentIndex === -1) {
        newIndex = 0;
      } else {
        if (event.keyCode === 38)
          newIndex = (currentIndex + items.length - 1) % items.length;
        else newIndex = (currentIndex + 1) % items.length;
      }
      if (current !== null && items[newIndex] !== null) {
        current.blur();
        items[newIndex].focus();
      }
    } else if (event.key == "Enter") {
      const current = document.activeElement as HTMLElement | null;
      if (current !== null) {
        current.click();
      }
    } else {
      const searchBarInput = document.getElementById(
        "searchBarInput"
      ) as HTMLInputElement | null;
      searchBarInput.focus();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="searchResults">
  {#if results.length > 0 && results[0] !== " "}
    {#each results.slice(0, 5) as result}
      <button on:click class="searchResult" id={result}>
        {#await getIcon(result
            .split("/")
            .pop()
            .replace(/.app$/, "")) then { icon, fallbackIcon }}
          <img
            class="appIcon"
            src={icon}
            alt=""
            on:error={(event) => {
              // @ts-ignore
              event.target.src = fallbackIcon;
            }}
          />
        {/await}
        <p class="appName">{result.split("/").pop().replace(/.app$/, "")}</p>
      </button>
    {/each}
  {/if}
</div>

<style>
  .searchResult {
    margin-top: 7px;
    margin-left: 12px;
    box-sizing: border-box;
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0px 12px;
    width: 726px;
    height: 43px;
    background: transparent;
    border: none;
    border-radius: 8px;
    flex: none;
    order: 1;
    flex-grow: 0;
  }
  .searchResult:focus {
    background: var(--highlight-overlay);
    outline: 0;
  }

  .appIcon {
    width: 24px;
    height: 24px;
    margin-right: 8px;
  }

  .appName {
    font-family: Helvetica;
    font-style: normal;
    font-weight: 500;
    font-size: 14px;
    line-height: 20px;
    margin: 0;
    color: var(--primary-text-color);
  }
</style>
