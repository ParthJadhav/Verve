<script lang="ts">
  export let results: string[];
  export let resultType: number;

  import { appWindow, LogicalSize } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/tauri';
  import { afterUpdate } from 'svelte';
  import CalculationResult from './CalculationResult.svelte';
  import FileSearchResult from './FileSearchResult.svelte';

  afterUpdate(async () => {
    const container = document.querySelector<HTMLElement>('.container');
    if (!container) return;
    const height = container.clientHeight;
    await appWindow.setSize(new LogicalSize(750, height));
    if (results.length > 0 && results[0] !== '') {
      const firstResult = document.getElementById(results[0]);
      firstResult?.classList.add('searchResultFocused');
      firstResult?.focus();
    }
  });

  const searchResultClicked = async (event: any) => {
    await invoke('open_command', { path: event.target.id });
    const searchBarInput = document.getElementById(
      'searchBarInput'
    ) as HTMLInputElement;
    results = [];
    searchBarInput.value = '';
    await appWindow.hide();
  };

  async function handleKeydown(event: KeyboardEvent) {
    if (event.keyCode == 38 || event.keyCode == 40) {
      const current = document.activeElement as HTMLElement | null;
      const items = [...document.getElementsByClassName('searchResult')] as HTMLElement[];
      if (items.length === 0) return;
      const currentIndex = current ? items.indexOf(current) : -1;
      let newIndex: number;

      if (currentIndex === -1) {
        newIndex = 0;
      } else {
        if (event.keyCode === 38)
          newIndex = (currentIndex + items.length - 1) % items.length;
        else newIndex = (currentIndex + 1) % items.length;
      }
      const nextItem = items[newIndex];
      if (current !== null && nextItem) {
        nextItem.classList.add('searchResultFocused');
        current.classList.remove('searchResultFocused');
        current.blur();
        nextItem.focus();
      }
    } else if (event.key == 'Enter') {
      const current = document.activeElement as HTMLElement | null;
      if (current !== null) {
        current.click();
      }
    } else {
      const searchBarInput = document.getElementById(
        'searchBarInput'
      ) as HTMLInputElement | null;
      searchBarInput?.focus();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="searchResults">
  {#if results.length > 0 && results[0] !== ' '}
    {#if resultType !== 3}
      {#each results.slice(0, 5) as result}
        <FileSearchResult
          filePath={result}
          on:click={searchResultClicked}
          {resultType}
        />
      {/each}
    {:else}
      <CalculationResult bind:results />
    {/if}
  {/if}
</div>

<style>
  :global(.searchResultFocused) {
    background: var(--highlight-overlay) !important;
    outline: 0 !important;
    border-radius: 8px;
  }
</style>
