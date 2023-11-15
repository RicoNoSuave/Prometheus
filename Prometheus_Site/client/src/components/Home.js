import React from "react";
import Nav from "./Navbar";

const Home = () => {
	return (
		<>
			<Nav />
			<center>
				<table className="about">
					<th>
						<h3>Welcome to Prometheus!</h3>
					</th>
					<tr>
						<td align="left">
							<p>
								&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Prometheus is a memory-secure newsreader application built in Rust. All news is sourced from<br />
								<a href="https://www.newsapi.org/">NewsAPI.org</a>, a continuation of the Google News API. Using this application, you can view top news<br />
								stories in the country of your choice. You can also view news by category, including: Business,<br />
								entertainment, health, science, sports, and technology. If you need news on a particular topic, you are<br />
								also able to search using your own custom terms. Lastly, you can enhance your readability experience<br />
								by adjusting the font size of the application and toggling night mode.<br /><br />
								Thank you for taking the time to use our product! If you wish to see more work by the developer, you<br />
								can find their portfolio <a href="https://riconosuave.github.io/portfolio">here</a>, or contact them <a href="mailto:ricardo.e.harris@gmail.com">here</a>!
							</p>
						</td>
					</tr>
					<th>
						<h3>How to Use</h3>
					</th>
				</table>
				<div className="about">
				</div>
				<div>
				</div>
			</center>
		</>
	)
}

export default Home;