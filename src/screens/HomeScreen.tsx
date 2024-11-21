import { invoke } from '@tauri-apps/api/core'

export function HomeScreen() {
	// useEffect(() => {
	// 	invoke('update_activity', {
	// 		state: 'Página Inicial',
	// 		details: 'Navegando...',
	// 		large_image_key: 'icon',
	// 		large_image_label: 'UC Launcher',
	// 	}).catch(console.error)
	// }, [])

	const updateActivity = async () =>
		await invoke('update_activity', {
			details: 'Página Inicial',
			state: 'Navegando...',
			largeImageKey: 'icon',
			largeImageLabel: 'UC Launcher',
			smallImageKey: 'icon',
			smallImageLabel: 'UC Launcher',
		})

	const clearActivity = async () => await invoke('clear_activity')

	return (
		<section className="flex flex-col items-center justify-center gap-4 h-screen w-screen">
			<h1>Página Inicial</h1>

			<div className="flex flex-row justify-center items-center gap-8">
				<button
					type="button"
					onClick={updateActivity}
					className="px-4 py-2 bg-indigo-700 rounded transition-all hover:bg-opacity-90 hover:scale-110"
				>
					Atualizar Atividade
				</button>
				<button
					type="button"
					onClick={clearActivity}
					className="px-4 py-2 bg-indigo-700 rounded transition-all hover:bg-opacity-90 hover:scale-110"
				>
					Limpar Atividade
				</button>
			</div>
		</section>
	)
}
