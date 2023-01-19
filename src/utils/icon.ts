import { resolveResource, appDataDir, join } from '@tauri-apps/api/path';
import { FALLBACK_ICON_SYMBOL, icons } from '../cache';
import { convertFileSrc } from '@tauri-apps/api/tauri';

export const getIcon = async (app_name: string) => {
  let icon = icons.get(app_name);
  let fallbackIcon = icons.get(FALLBACK_ICON_SYMBOL);

  if (icon && fallbackIcon) {
    return { icon, fallbackIcon };
  }

  if (!fallbackIcon) {
    fallbackIcon = convertFileSrc(await resolveResource('assets/default.svg'));
    icons.set(FALLBACK_ICON_SYMBOL, fallbackIcon);
  }

  let iconPath: string;
  if (
    [
      'Migration Assistant',
      'System Information',
      'Calendar',
      'System Settings',
      'Photo Booth',
      'AirPort Utility',
    ].includes(app_name)
  ) {
    iconPath = await resolveResource(`assets/appIcons/${app_name}.app.png`);
  } else {
    const appDataDirPath = await appDataDir();
    iconPath = await join(appDataDirPath, `appIcons/${app_name}.app.png`);
  }

  icon = convertFileSrc(iconPath);
  icons.set(app_name, icon);
  return { icon, fallbackIcon };
};
