import { useState } from 'react'
import './App.css'

function App() {
	const [blob, setBlob] = useState<Blob | null>(null)

	return (
		<div
			className="container"
			onPaste={async () => {
				try {
					const a = (
						await navigator.clipboard.read()
					)[0]

					const mime = a.types.find(type =>
						type.startsWith('image/'),
					)

					if (!mime) return

					const blob1 = await a.getType(mime)

					setBlob(blob1)
				} catch {}
			}}
		>
			<h1>Welcome to Tauri!</h1>

			<img
				src={
					blob
						? window.URL.createObjectURL(blob)
						: undefined
				}
			></img>
		</div>
	)
}

export default App
