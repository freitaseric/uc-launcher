import { Outlet } from 'react-router-dom'
import { Appbar } from './components'

export function App() {
	return (
		<main className="flex flex-row items-center justify-between bg-zinc-800 text-zinc-50">
			<Appbar />
			<Outlet />
		</main>
	)
}
