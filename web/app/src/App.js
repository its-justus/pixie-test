import logo from './logo.svg';
import './App.css';
import CreateCampaign from './components/CreateCampaign/CreateCampaign'
import Register from './components/Register/Register';

function App() {
	return (
		<div className="App">
			<Register/>
			<CreateCampaign/>
		</div>
	);
}

export default App;
