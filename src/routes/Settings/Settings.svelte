<script>
    import MenuButton from "./lib/MenuButton.svelte";
    import ShortcutControl from "./lib/controls/ShortcutControl.svelte";
    import StartAtLoginControl from "./lib/controls/StartAtLoginControl.svelte";
    import MenuBarIconControl from "./lib/controls/MenuBarIconControl.svelte";
    import { preferences } from "../../cache";
</script>

<div class="Settings prevent-select">
    <div class="menu-buttons">
        <div class="general-button">
            <MenuButton icon="setting" text="General" />
        </div>
    </div>
    <hr class="seperator" />
    <div class="controls">
        <ShortcutControl />
        <StartAtLoginControl
            startAtLogin={preferences.get("launch_on_login")}
        />
        <MenuBarIconControl menuBarIcon={preferences.get("menu_bar_icon")} />
    </div>
</div>

<style>
    .general-button {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 40px;
        width: 55px;
        height: 53.2px;
    }

    .menu-buttons {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 40px;
        padding: 20px;
        margin-left: 25%;
        width: 50%;
    }

    .controls {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 40px;
    }

    .Settings {
        position: relative;
        width: 100%;
        height: 352px;
        background: var(--primary-bg-color);
        border-radius: 10px;
        align-items: center;
        text-align: center;
        margin-bottom: 50px;
    }

    .prevent-select {
        -webkit-user-select: none; /* Safari */
        -ms-user-select: none; /* IE 10 and IE 11 */
        user-select: none; /* Standard syntax */
    }
</style>
