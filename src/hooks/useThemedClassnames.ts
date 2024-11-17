import { useTheme } from './useTheme'

export const useThemedClassname = (className?: string) => {
	const { theme } = useTheme()

	return `${theme}-mode ${className}`
}
