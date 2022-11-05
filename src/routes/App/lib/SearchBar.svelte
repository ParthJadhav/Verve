<script lang="ts">
    import { onMount } from "svelte";
    let element;
    onMount(function () {
        element.focus();
    });
</script>

<div>
    <form action="" id="searchBarForm">
        <input
            type="text"
            placeholder="search"
            id="searchBarInput"
            bind:this={element}
            on:input
            spellcheck="false"
        />
    </form>
</div>

<style>
    div {
        margin-bottom: 16px;
    }
    #searchBarInput {
        width: 100%;
        height: 56px;
        border-radius: 8px 8px 0px 0px;
        background: var(--secondary-bg-color);
        padding-left: 16px;
        border: none;
        font-family: Helvetica;
        font-style: normal;
        color: var(--primary-text-color);
        font-weight: 500;
        font-size: 18px;
        line-height: 20px;
        outline: none;
    }

    #searchBarForm {
        width: 733px;
        height: 56px;
        background: var(--secondary-bg-color);
        border-radius: 10px;
        border: none;
    }
</style>
