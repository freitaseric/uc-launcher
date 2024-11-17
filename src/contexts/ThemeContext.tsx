import { createContext } from 'react'
import type { AppTheme, AppThemes } from '../types'

export const ThemeContext = createContext<
	| {
			theme: AppTheme
			themes: typeof AppThemes
			setTheme: (theme: AppTheme) => void
	  }
	| undefined
>(undefined)
