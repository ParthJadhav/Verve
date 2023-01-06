<script lang="ts">
    import hotkeys from "hotkeys-js";
    import { preferences, paths } from "../../../../cache";
    import { writeTextFile } from "@tauri-apps/api/fs";
    import { listenForHotkey } from "../../../../main";
    import { unregister } from "@tauri-apps/api/globalShortcut";

    let shortcutArray: string[] = preferences.get("shortcut").split("+");

    function updateShortcutPreference(newShortcut: string[]) {
        if (newShortcut.join("+") === preferences.get("shortcut")) {
            return;
        }
        const newShortcutString = newShortcut
            .join("+")
            .replace("Meta", "Command");
        writeTextFile(
            `${paths.get("appDataDirPath")}/preferences.json`,
            JSON.stringify(
                {
                    ...Object.fromEntries(preferences),
                    shortcut: newShortcutString,
                },
                null,
                4
            )
        ).then(() => {
            unregister(preferences.get("shortcut")).then(() => {
                preferences.set("shortcut", newShortcutString);
                listenForHotkey(newShortcutString);
            });
        });
    }
    const modifiers = ["Meta", "Control", "Alt", "Shift"];
    async function recordShortcut() {
        shortcutArray = [];
        hotkeys.unbind();
        hotkeys("*", { keyup: true }, function (event) {
            if (event.type === "keydown") {
                if (shortcutArray.length < 3) {
                    if (shortcutArray.length === 0) {
                        if (modifiers.includes(event.key)) {
                            if (event.key === " "){
                                shortcutArray.push("Space");
                            } else {
                                shortcutArray.push(event.key);
                            }
                            shortcutArray = shortcutArray;
                        }
                    } else {
                        if (!modifiers.includes(event.key)) {
                            const key = event.code.startsWith("Key")
                                ? event.code.substring(3)
                                : event.code;
                            shortcutArray.push(key);
                            updateShortcutPreference(shortcutArray);
                            hotkeys.unbind();
                        } else {
                            if (event.key === " "){
                                shortcutArray.push("Space");
                            } else {
                                shortcutArray.push(event.key);
                            }
                        }
                        shortcutArray = shortcutArray.filter(
                            (value, index, self) => {
                                return self.indexOf(value) === index;
                            }
                        );
                    }
                } else {
                    updateShortcutPreference(shortcutArray);
                    hotkeys.unbind();
                }
            }
            if (event.type === "keyup") {
                if (shortcutArray.length === 1) {
                    shortcutArray = ["Command", "Shift", "G"];
                    updateShortcutPreference(shortcutArray);
                    hotkeys.unbind();
                } else if (shortcutArray.length > 1) {
                    shortcutArray.pop();
                    shortcutArray = shortcutArray;
                }
            }
        });
    }
</script>

<div class="shortcut-control">
    <p>Hotkey:</p>
    <input
    id="key-recorder"
    type="button"
    value={shortcutArray.length > 0
        ? shortcutArray
              .map((key) => key)
              .join(" + ")
              .replace("Meta", "⌘")
              .replace("Command", "⌘")
              .replace("Control", "⌃")
              .replace("Alt", "⌥")
              .replace("Shift", "⇧")
        : "Recording..."}
    on:click={recordShortcut}
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
    #key-recorder {
        background: var(--secondary-bg-color);
        border-radius: 4px;
        padding: 8px;
        border: none;
        color: var(--primary-text-color);
        font-size: 12px;
        font-weight: bold;
        text-transform: uppercase;
    }
</style>
