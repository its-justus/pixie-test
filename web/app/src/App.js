import './App.css';
import CreateCampaign from './components/CreateCampaign/CreateCampaign'
import Register from './components/Register/Register';
import MetaMaskAuth from './components/MetaMask/MetaMaskAuth/MetaMaskAuth';

function onAddressChanged(newAddress) {
	console.log(newAddress);
}

function App() {
	return (
		<div className="App">
			<Register/>
			<MetaMaskAuth onAddressChanged={onAddressChanged}/>
			<CreateCampaign/>
		</div>
	);
}

export default App;
