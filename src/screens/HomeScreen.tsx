import { useEffect } from 'react'
import { useDiscordPresence } from '../hooks'

export function HomeScreen() {
	const { setPresence, clearPresence } = useDiscordPresence()

	useEffect(() => {
		setPresence({
			details: 'Página Inicial',
			state: 'Navegando numa boa',
			large_image_key: 'home',
			large_image_label: 'Página Inicial',
			buttons_labels: ['Código Fonte'],
			buttons_urls: ['https://github.com/freitaseric/uc-launcher#readme'],
		})
	}, [setPresence])

	return (
		<section className="flex flex-col items-center justify-center gap-4 h-screen w-screen">
			<h1>Página Inicial</h1>

			<div className="flex flex-row justify-center items-center gap-8">
				<button
					type="button"
					onClick={() =>
						setPresence({
							details: 'Página Inicial',
							state: 'Navegando numa boa',
							large_image_key: 'home',
							large_image_label: 'Página Inicial',
							buttons_labels: ['Código Fonte'],
							buttons_urls: [
								'https://github.com/freitaseric/uc-launcher#readme',
							],
						})
					}
					className="px-4 py-2 bg-indigo-700 rounded transition-all hover:bg-opacity-90 hover:scale-110"
				>
					Atualizar Atividade
				</button>
				<button
					type="button"
					onClick={clearPresence}
					className="px-4 py-2 bg-indigo-700 rounded transition-all hover:bg-opacity-90 hover:scale-110"
				>
					Limpar Atividade
				</button>
			</div>
		</section>
	)
}
