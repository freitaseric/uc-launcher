import { ChevronFirst, ChevronLast, MoreVertical } from 'lucide-react'
import { useContext, useState } from 'react'
import { SidebarContext } from '../../contexts'

interface SidebarProps {
	children: React.ReactNode
}

export function Sidebar({ children }: SidebarProps) {
	const [expanded, setExpanded] = useState(true)

	return (
		<aside className="h-screen">
			<nav className="h-full flex flex-col bg-indigo-950 bg-opacity-35 border-r shadow-lg">
				<div className="p-4 pb-2 flex justify-between items-center">
					<div className="flex flex-row justify-center items-center gap-2">
						<img
							src="/icon.png"
							className={`overflow-hidden transition-all ${
								expanded ? 'w-8' : 'w-0'
							}`}
							alt=""
						/>
						<h2
							className={`overflow-hidden transition-all ${
								expanded ? 'block' : 'hidden'
							}`}
						>
							UC Launcher
						</h2>
					</div>
					<button
						type="button"
						onClick={() => setExpanded(curr => !curr)}
						className="p-1.5 rounded-lg transition-all bg-white bg-opacity-0 hover:bg-opacity-15"
					>
						{expanded ? <ChevronFirst /> : <ChevronLast />}
					</button>
				</div>

				<SidebarContext.Provider value={{ expanded }}>
					<ul className="flex-1 px-3">{children}</ul>
				</SidebarContext.Provider>

				<div className="border-t flex p-3 flex-row justify-center items-center">
					<img
						src="https://api.mcheads.org/head/oDevEric"
						alt="Minecraft user head"
						className="w-10 h-10 rounded-md"
					/>
					<div
						className={`
              flex justify-between items-center
              overflow-hidden transition-all ${expanded ? 'w-52 ml-3' : 'w-0'}
          `}
					>
						<div className="leading-4">
							<h4 className="font-semibold">oDevEric</h4>
							<span className="text-xs text-gray-400">
								Microsoft Autheticated
							</span>
						</div>
						<MoreVertical size={20} />
					</div>
				</div>
			</nav>
		</aside>
	)
}

interface SidebarItemProps {
	icon?: React.ReactElement
	text: string
	active?: boolean
	alert?: boolean
	onClick?: (event: React.MouseEvent<HTMLLIElement, MouseEvent>) => void
}

export function SidebarItem({
	icon,
	text,
	active,
	alert,
	onClick,
}: SidebarItemProps) {
	const { expanded } = useContext(SidebarContext)

	return (
		<li
			onClick={onClick}
			onKeyUp={() => {}}
			className={`
			relative flex items-start py-2 px-3 my-1
			font-medium rounded-md cursor-pointer
			transition-colors group
			${
				active
					? 'bg-gradient-to-tr from-indigo-500 to-indigo-700 text-white'
					: 'hover:bg-indigo-300 hover:text-indigo-800'
			}
			`}
		>
			{icon}
			<span
				className={`overflow-hidden transition-all ${
					expanded ? 'w-52 ml-3' : 'w-0'
				}`}
			>
				{text}
			</span>
			{alert && (
				<div
					className={`absolute right-2 w-2 h-2 rounded bg-purple-700 ${
						expanded ? '' : 'top-2'
					}`}
				/>
			)}

			{!expanded && (
				<div
					className={`
          absolute left-full rounded-md px-2 py-1 ml-6
          bg-indigo-600 text-white text-sm
          invisible opacity-20 -translate-x-3 transition-all
          group-hover:visible group-hover:opacity-100 group-hover:translate-x-0
      `}
				>
					{text}
				</div>
			)}
		</li>
	)
}
