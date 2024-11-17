import { useEffect, useState } from 'react'
import { ThemeContext } from '../contexts'
import { store } from '../libs'
import { type AppTheme, AppThemes } from '../types'
import './ThemeProvider.css'

interface ThemeProviderProps {
	defaultTheme: AppTheme
	children: React.ReactNode
}

export const ThemeProvider = ({
	children,
	defaultTheme,
}: ThemeProviderProps) => {
	const [themeState, setThemeState] = useState<AppTheme>(defaultTheme)

	function setTheme(theme: AppTheme) {
		setThemeState(theme)
		store.cache.set('theme', theme).catch(console.error)
	}

	useEffect(() => {
		store.cache
			.get<AppTheme>('theme')
			.then(cachedTheme => setThemeState(cachedTheme ?? defaultTheme))
			.catch(console.error)
	}, [defaultTheme])

	return (
		<ThemeContext.Provider
			value={{ theme: themeState, themes: AppThemes, setTheme }}
		>
			<body className={`${themeState}-mode`}>{children}</body>
		</ThemeContext.Provider>
	)
}
