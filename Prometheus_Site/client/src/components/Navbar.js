import React from "react";
import logo from '../PrometheusLogo.png';

var URL = "https://github.com/RHarris87345/CSU-Senior-Project/blob/master/docs/Prometheus%20Project%20Requirements.md";

const Nav = () => {
    return (
        <nav className='navbar'>
            <img className='logo' alt = "logo" src = { logo }/>
            <form>
                <ul className='navbutton'>
                    <li><h2>Prometheus</h2></li>
                    <li><h3><a href="/">Home</a></h3></li>
                    <li><h3><a href={ URL }>Documentation</a></h3></li>
                    <li><h3><a href="/download">Download</a></h3></li>
                </ul>
            </form>
        </nav>
    )
}

export default Nav;