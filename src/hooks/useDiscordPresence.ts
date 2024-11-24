import { invoke } from '@tauri-apps/api/core'

interface PresenceData {
	details: string
	state: string
	large_image_key: string
	large_image_label: string
	small_image_key: string
	small_image_label: string
	buttons_labels: string[]
	buttons_urls: string[]
	timestamp_start: number
	timestamp_end: number
}

export function useDiscordPresence() {
	const connectClient = async () => {
		await invoke('connect_presence')
	}
	const disconnectClient = async () => {
		await invoke('disconnect_presence')
	}
	const setPresence = async (data: Partial<PresenceData>) => {
		await invoke('set_presence', {
			...data,
			timestamp_start: data.timestamp_start ?? Date.now(),
		})
	}
	const clearPresence = async () => {
		await invoke('clear_presence')
	}

	return { connectClient, disconnectClient, setPresence, clearPresence }
}
