import React, { useState } from "react";

function Register() {
	const [username, setUsername] = useState("");

	function registerUser(){
		console.log("Registering User", username);
	}

	return (
		<div>
			Register
			<input onChange={(e) => setUsername(e.target.value)} type={"text"}/>
			<button onClick={() => registerUser()} type={"button"}>Register</button>
		</div>
	)
}

export default Register;