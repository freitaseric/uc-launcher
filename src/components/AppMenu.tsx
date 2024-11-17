import { ResizableBox } from 'react-resizable'

interface AppMenuProps {
	width: number
	setWidth: React.Dispatch<React.SetStateAction<number>>
}

export function AppMenu({ width, setWidth }: AppMenuProps) {
	return (
		<ResizableBox
			width={width}
			height={Number.POSITIVE_INFINITY}
			axis="x"
			resizeHandles={['e']}
			minConstraints={[100, Number.POSITIVE_INFINITY]}
			maxConstraints={[500, Number.POSITIVE_INFINITY]}
			onResizeStop={(_, data) => setWidth(data.size.width)}
			className="border-r-2 border-r-zinc-800 dark:border-r-zinc-400"
		>
			<h2>Menu</h2>
		</ResizableBox>
	)
}
