<script lang="ts">
  import { appWindow, LogicalSize } from "@tauri-apps/api/window";
  import SearchBar from "./lib/SearchBar.svelte";
  import Footer from "./lib/Footer.svelte";
  import SearchResult from "./lib/SearchResult.svelte";
  import Settings from "../Settings/Settings.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  export let appState = {
    app: true,
    settings: false,
  };
  let results: string[] = [];
  let footerText: string = "verve.app";
  let executionTime: number = 0;

  document.onkeyup = function (event) {
        if (event.metaKey && event.key === ",") {
            appState.app = false;
            appState.settings = true;
        }
    };

  const onBlur = async () => {
    await appWindow.hide();
    appState.app = true;
    appState.settings = false;
  };

  const searchResultClicked = async (event: any) => {
    await invoke("open_command", { path: event.target.id });
    const searchBarInput = document.getElementById(
      "searchBarInput"
    ) as HTMLInputElement;
    results = [];
    searchBarInput.value = "";
    await appWindow.hide();
  };

  const search = async (searchPrompt: string) => {
    footerText = "Loading...";
    [results, executionTime] = await invoke("handle_input", {
      input: searchPrompt,
    });
    if (results.length === 0) {
      footerText = "No results found";
      return;
    }
    footerText = `~ ${Math.floor(executionTime * 1000)} ms`;
  };

  document.addEventListener("keydown", async (event: any) => {
    if (event.keyCode == 13) {
      event.preventDefault();
      if (event.target.value.startsWith("/")) {
        search(event.target.value);
      }
    }
  });

  const handleInput = async (event: any) => {
    results = [];
    if (event.target.value === "") {
      footerText = "verve.app";
      return;
    }
    if (event.target.value.startsWith("/")) {
      // this signifies a full drive search. Searches in /Users folder
      footerText = "Press Enter to search";
      return;
    }
    search(event.target.value);
  };
</script>

<svelte:window on:blur={onBlur} />

<main class="container">
  {#if appState.app}
    <!-- svelte-ignore empty-block -->
    {#await appWindow.setSize(new LogicalSize(750, 100))}{/await}
    <SearchBar on:input={handleInput}/>
    <SearchResult {results} on:click={searchResultClicked} />
    <Footer {footerText} />
  {/if}
  {#if appState.settings}
    <!-- svelte-ignore empty-block -->
    {#await appWindow.setSize(new LogicalSize(750, 400))}{/await}
    <Settings />
  {/if}
</main>

<style>
  .container {
    height: 100%;
    width: 750px;
    background: var(--primary-bg-color);
    border-radius: 10px;
  }
</style>
