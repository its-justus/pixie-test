import React, { useEffect, useState } from "react";
import ConnectMetaMask from "../ConnectMetaMask/ConnectMetaMask";

async function isMetaMaskConnected(onConnected) {
	const accounts = window.ethereum?.request({
		method: "eth_requestAccounts",
	});
	if (accounts) onConnected(accounts[0]);
}

function MetaMaskAuth({onAddressChanged}) {
	const [address, setAddress] = useState("");

	useEffect(() => {
		isMetaMaskConnected(setAddress);
	}, []);

	useEffect(() => {
		onAddressChanged(address);
	}, [address, onAddressChanged]);

	return (
		<div>
			Login
			{address 
				? <div>Connected with {address}</div>
				: <ConnectMetaMask setAddress={setAddress}/> }
		</div>
	)
}

export default MetaMaskAuth;