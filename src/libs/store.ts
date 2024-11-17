import { load } from '@tauri-apps/plugin-store'

export const store = {
	cache: await load('theme.json'),
}
