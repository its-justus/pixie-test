import React, { useState } from "react";
import axios from "axios";


function CreateCampaign() {
	const [name, setName] = useState("");
	const [testers, setTesters] = useState(0);

	function submitCampaign() {
		const config = {
			headers: { "Content-Type": "application/json" },
		};
		axios.post(
			"http://localhost:9001/campaign", 
			{name: name, testers: Number(testers)},
			config)
			.then((res) => {
				console.log(res);
			});
	}
	
	return (
		<div>
			<input onChange={e => setName(e.target.value)} type={"text"}/>
			<input onChange={e => setTesters(e.target.value)} type={"number"}/>
			<button type="button" onClick={() => submitCampaign()}>Create Campaign</button>
		</div>
	)
}

export default CreateCampaign;