export const AppThemes = ['nocturne', 'diurnal'] as const

export type AppTheme = (typeof AppThemes)[number]
