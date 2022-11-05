<script lang="ts">
    export let icon: string;
    export let text: string;
    export let active: boolean = false;
    import { resolveResource } from "@tauri-apps/api/path";
    import { convertFileSrc } from "@tauri-apps/api/tauri";

    const getIconPath = async (iconName) => {
        const icon_path = await resolveResource(`assets/${iconName}.svg`);
        return convertFileSrc(icon_path);
    };
</script>

<div class={active ? "tab-button" : "tab-button active-tab"}>
    {#await getIconPath(icon) then iconPath}
        <img src={iconPath} alt="general" />
    {/await}
    <p class="button-text">{text}</p>
</div>

<style>
    .button-text {
        font-size: 14px;
        color: var(--primary-text-color);
        margin-top: 5px;
        margin-bottom: 0;
    }

    .tab-button {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 2px;
    }

    .active-tab {
        padding: 10px;
        background-color: var(--secondary-bg-color);
        border-radius: 10px;
    }
</style>
