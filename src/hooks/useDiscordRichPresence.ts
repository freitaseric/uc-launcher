import { listen } from '@tauri-apps/api/event'
import { useState } from 'react'

interface ImageAsset {
	key: string
	label?: string
}

interface Button {
	label: string
	url: string
}

interface Timestamp {
	start: number
	end: number
}

interface DiscordRichPresence {
	details?: string
	state?: string
	largeImage?: ImageAsset
	smallImage?: ImageAsset
	buttons: Button[]
	timestamp?: Timestamp
}

export function useDiscordRichPresence(): [
	currentActivity: DiscordRichPresence | undefined,
] {
	const [currentActivity, setCurrentActivity] = useState<DiscordRichPresence>()

	listen<DiscordRichPresence>('current_activity', event => {
		console.log(event)
		setCurrentActivity(event.payload)
	}).catch(console.error)

	return [currentActivity]
}
