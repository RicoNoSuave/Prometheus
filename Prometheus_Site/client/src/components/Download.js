import React, { useEffect, useState } from "react";
import ReactMarkdown from "react-markdown";
import Nav from "./Navbar";

const Download = () => {
	const [instructions, setInstructions] = useState("");

	useEffect(() => {
		fetch("./instructions.md")
			.then((res) => res.text())
			.then((text) => setInstructions(text));
	}, []);

	
	return (
		<>
			<Nav />
			<center>
				<h1>
					<p className="Header">
						Download
					</p>
				</h1>
				<h3>
					<p className="SubHeader">
						Download for your Operating System:
					</p>
				</h3>
				<p className="Text">
					<img alt="Windows" width="30px" src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/windows8/windows8-original.svg" />
					: click&nbsp;
						<a href='/Prometheus Windows Release 1.0.exe' download>
							here
						</a>
				</p>
				<p className="Text">
					<img alt="Linux" width="30px"  src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/linux/linux-original.svg" />
					: click&nbsp;
					<a href='/Prometheus Linux Release 1.0.tgz' download>
						here
					</a>
				</p>
				<div className="about">
					<table>
						<thead>
							<tr>
								<th align="left">
									<b><h3>Windows Instructions</h3></b>
								</th>
								<th>
									<b><h3>Linux Instructions</h3></b>
								</th>
							</tr>
						</thead>
					</table>
				</div>
			</center>
		</>
	)
}

export default Download;