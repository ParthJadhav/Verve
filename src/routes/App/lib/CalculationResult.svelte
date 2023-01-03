<script lang="ts">
    export let results: string[];
    import { appWindow } from "@tauri-apps/api/window";

    const copyAnswer = async () => {
        await navigator.clipboard.writeText(results[0]);
    };

    const searchResultClicked = async (event: any) => {
        await copyAnswer();
        const searchBarInput = document.getElementById(
            "searchBarInput"
        ) as HTMLInputElement;
        searchBarInput.value = "";
        results = [];
        await appWindow.hide();
    };
</script>

<div class="result">
    <button class="calculation-button" id={results[0]} on:click={searchResultClicked}>
        <p>{results[0]}</p>
    </button>
</div>

<style>
    .result {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        border: none;
        border-radius: 8px;
        width: 726px;
        height: 43px;
        background-color: var(--highlight-overlay);
        margin-top: 7px;
        margin-left: 12px;
    }
    .calculation-button {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: 100%;
        background: transparent;
        border: none;
        outline: none;
    }
    p {
        margin: 0;
        padding: 0;
        color: var(--primary-text-color);
        font-size: 16px;
    }
</style>
