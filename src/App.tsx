import { Outlet } from 'react-router-dom'
import { ThemeProvider } from './components'
import './globals.css'
import './variables.css'

export default () => {
	return (
		<ThemeProvider defaultTheme="diurnal">
			<Outlet />
		</ThemeProvider>
	)
}
