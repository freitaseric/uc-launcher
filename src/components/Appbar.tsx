import { HomeIcon, Settings2Icon } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { Sidebar, SidebarItem } from './ui/Sidebar'

export function Appbar() {
	const navigate = useNavigate()

	return (
		<Sidebar>
			<SidebarItem
				icon={<HomeIcon />}
				text="Início"
				onClick={() => navigate('/')}
			/>
			<SidebarItem
				icon={<Settings2Icon />}
				text="Configurações"
				onClick={() => navigate('/settings')}
			/>
		</Sidebar>
	)
}
