import { useTheme, useThemedClassname } from '../hooks'

export function HomeScreen() {
	const { setTheme, theme } = useTheme()

	return (
		<main>
			<h1>Hello from Tauri!</h1>
			<h2>In the home screen</h2>
			<button
				onClick={() => setTheme(theme === 'diurnal' ? 'nocturne' : 'diurnal')}
				type="button"
				className={useThemedClassname()}
			>
				Mudar Tema
			</button>
		</main>
	)
}
