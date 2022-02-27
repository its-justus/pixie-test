import axios from "axios";
import React, { useState } from "react";

function Register() {
	const [username, setUsername] = useState("");

	function registerUser(){
		console.log("Registering User", username);
		const config = {
			headers: { "Content-Type": "application/json" },
		  };
		axios.post(
			"http://localhost:9001/register", 
			{username: username},
			config)
			.then((res) => {
				console.log(res);
			});
	}

	return (
		<div>
			Register
			<input onChange={e => setUsername(e.target.value)} type={"text"}/>
			<button onClick={() => registerUser()} type={"button"}>Register</button>
		</div>
	)
}

export default Register;