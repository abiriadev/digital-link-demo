import './App.css'
import data from '../../data-collector/products.json'
import QRCode from 'react-qr-code'
import { useState } from 'react'

const lh = 'http://localhost'

function App() {
	const [baseUrl, setBaseUrl] = useState(lh)

	return (
		<>
			<h1>GS1 Digital Link Demo Products QRcode</h1>
			<input
				value={baseUrl}
				onChange={e => setBaseUrl(e.target.value)}
			/>
			<button onClick={() => setBaseUrl(lh)}>
				Reset
			</button>
			<ul>
				{data.map(({ gtin, name, image }) => (
					<li>
						<h2>{name}</h2>
						<img
							src={image}
							width="300px"
						></img>
						<QRCode
							value={`${baseUrl}/01/${gtin}`}
						/>
					</li>
				))}
			</ul>
		</>
	)
}

export default App
