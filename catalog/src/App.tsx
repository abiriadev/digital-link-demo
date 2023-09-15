import './App.css'
import data from '../../data-collector/products.json'
import QRCode from 'react-qr-code'

function App() {
	return (
		<>
			<h1>GS1 Digital Link Demo Products QRcode</h1>
			<ul>
				{data.map(({ url, name, image }) => (
					<li>
						<h2>{name}</h2>
						<img
							src={image}
							width="300px"
						></img>
						<QRCode value={url} />
					</li>
				))}
			</ul>
		</>
	)
}

export default App
