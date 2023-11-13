import React from "react";
import logo from '../PrometheusLogo.png';

const Nav = () => {
	return (
		<nav className='navbar'>
			<img className='logo' alt="logo" src={ logo } />
			<form>
				<ul className='navbutton'>
					<li><h2>Prometheus</h2></li>
					<li><h3><a href="/">Home</a></h3></li>
					<li><h3><a href="/documentation">Documentation</a></h3></li>
					<li><h3><a href="/download">Download</a></h3></li>
				</ul>
			</form>
		</nav>
	)
}

export default Nav;