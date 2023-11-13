import React from "react";
import Nav from "./Navbar";

const Download = () => {
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
			</center>
		</>
	)
}

export default Download;