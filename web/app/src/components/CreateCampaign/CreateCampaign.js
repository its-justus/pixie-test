import React, { useState } from "react";
import axios from "axios";


function CreateCampaign() {
	const {name, setName} = useState("");
	const {testers, setTesters} = useState(0);

	function submitCampaign() {
		axios.post("localhost:3030", {name: name, testers: testers})
	}
	
	return (
		<div>
			<input onChange={e => setName(e.target.value)} type={"text"}/>
			<input onChange={e => setTesters(e.target.value)} type={"number"}/>
			<button type="button" onClick={() => submitCampaign()}>Create Campaign</button>
		</div>
	)
}