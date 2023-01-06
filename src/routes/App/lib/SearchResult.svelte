<script lang="ts">
  export let results: string[];
  export let resultType: number;

  import { appDataDir, join, resolveResource } from "@tauri-apps/api/path";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { appWindow, LogicalSize } from "@tauri-apps/api/window";
  import { afterUpdate } from "svelte";
  import { FALLBACK_ICON_SYMBOL, icons } from "../../../cache";
  import CalculationResult from "./CalculationResult.svelte";

  afterUpdate(async () => {
    const height = document.getElementsByClassName("container")[0].clientHeight;
    await appWindow.setSize(new LogicalSize(750, height));
    if (results.length > 0 && results[0] !== "") {
      const firstResult = document.getElementById(results[0]);
      firstResult.classList.add('searchResultFocused');
      await firstResult.focus();
    }
  });

  async function getIcon(app_name: string) {
    let icon = icons.get(app_name);
    let fallbackIcon = icons.get(FALLBACK_ICON_SYMBOL);

    if (icon && fallbackIcon) {
      return { icon, fallbackIcon };
    }

    if (!fallbackIcon) {
      fallbackIcon = convertFileSrc(
        await resolveResource("assets/default.svg")
      );
      icons.set(FALLBACK_ICON_SYMBOL, fallbackIcon);
    }

    let iconPath: string;
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
      iconPath = await resolveResource(
        `assets/appIcons/${app_name}.app.png`
      );
    } else {
      const appDataDirPath = await appDataDir();
      iconPath = await join(appDataDirPath, `appIcons/${app_name}.app.png`);
    }

    icon = convertFileSrc(iconPath);
    icons.set(app_name, icon);
    return { icon, fallbackIcon };
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
        items[newIndex].classList.add('searchResultFocused');
        current.classList.remove('searchResultFocused');
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
    {#if resultType !== 3}
      {#each results.slice(0, 5) as result}
        <button on:click class="searchResult" id={result}>
          {#await getIcon(result
              .split("/")
              .pop()
              .replace(/.app$/, ""))}
            <span class="appIcon"></span>
          {:then {icon, fallbackIcon}}
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
    {:else}
      <CalculationResult bind:results/>
    {/if}
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

  :global(.searchResultFocused) {
    background: var(--highlight-overlay) !important;
    outline: 0 !important;
    border-radius: 8px;
  }

  .appIcon {
    display: inline-flex;
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
