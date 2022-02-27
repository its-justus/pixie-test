import React, { useEffect, useState } from "react";
import ConnectMetaMask from "../ConnectMetaMask/ConnectMetaMask";

async function isMetaMaskConnected(onConnected) {
	// uses supplied setter (onConnected) to set the account state if
	// metamask is present and connected in the browser
	const accounts = window.ethereum?.request({
		method: "eth_requestAccounts",
	});
	if (accounts) onConnected(accounts[0]);
}

function MetaMaskAuth({onAddressChanged}) {
	// TODO: Move address state into a redux store
	const [address, setAddress] = useState("");

	useEffect(() => {
		isMetaMaskConnected(setAddress);
	}, []);

	useEffect(() => {
		// calls hook from parent to pass address change up
		// to parent. This will be removed later.
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