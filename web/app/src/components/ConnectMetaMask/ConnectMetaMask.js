import React from "react";

async function connect(onConnected){
	if (!window.ethereum) {
		alert("Get MetaMask!");
		return;
	}

	const accounts = await window.ethereum.request({
		method: "eth_requestAccounts",
	});

	onConnected(accounts[0]);
}

function ConnectMetaMask({setAddress}){
	return (
		<button onClick={() => connect(setAddress)}>
			Connect to MetaMask
		</button>
	)
}

export default ConnectMetaMask;