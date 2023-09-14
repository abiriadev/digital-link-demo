import { useEffect, useRef, useState } from 'react'
import './App.css'
import { Html5Qrcode } from 'html5-qrcode'

function App() {
	const [blob, setBlob] = useState<Blob | null>(null)
	const [link, setLink] = useState<string | null>(null)

	const qr = useRef<Html5Qrcode | null>(null)

	useEffect(() => {
		qr.current = new Html5Qrcode('qr')
	}, [])

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

					const res = await qr.current?.scanFile(
						new File([blob1], 'qr'),
						false,
					)

					if (!res) return

					setLink(res)

					console.log(res)
				} catch {}
			}}
		>
			<h1>GS1 Digital Link Smart Client Demo</h1>

			<img
				src={
					blob === null
						? '#'
						: window.URL.createObjectURL(blob)
				}
				width={300}
				style={{
					margin: '0 auto',
				}}
			></img>

			<button
				onClick={() => {
					setBlob(null)
					setLink(null)
				}}
			>
				reset image
			</button>

			{link && <h1>Decoded Link: {link}</h1>}

			<div id="qr"></div>
		</div>
	)
}

export default App
