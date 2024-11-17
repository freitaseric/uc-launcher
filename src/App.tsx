import { useState } from 'react'
import { Outlet } from 'react-router-dom'
import { AppMenu } from './components'

export function App() {
	const [width, setWidth] = useState(200)

	return (
		<main className="bg-zinc-300 text-zinc-950 flex flex-row ">
			<AppMenu
				width={width}
				setWidth={setWidth}
			/>
			<Outlet />
		</main>
	)
}
