<script lang="ts">
    import Slider from "../CheckBox.svelte";
    import { preferences, paths } from "../../../../cache";
    import { writeTextFile } from "@tauri-apps/api/fs";
    export let menuBarIcon: boolean = false;

    function updateMenuBarIconPreference() {
        const checkBox = document.getElementById(
            "menu-bar-icon-checkbox"
        ) as HTMLInputElement;
        if (preferences.get("menu_bar_icon") === checkBox.checked) {
            console.log("No change in menu bar icon preference");
            return;
        }
        preferences.set("menu_bar_icon", checkBox.checked);
        writeTextFile(
            `${paths.get("appDataDirPath")}/preferences.json`,
            JSON.stringify(Object.fromEntries(preferences), null, 4)
        ).then(() => {
            console.log("Updated launch at login preference");
            // make it work
            // invoke("toggle-menu_bar_icon", { enable: checkBox.checked });
        });
    }
</script>

<div class="shortcut-control">
    <p>Show in menu bar:</p>
    <Slider
        id={"menu-bar-icon-checkbox"}
        isChecked={menuBarIcon}
        onToggle={updateMenuBarIconPreference}
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