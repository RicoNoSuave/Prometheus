# Prometheus
	Free Community-Led Object-Based Terminal

# Author
	Ricardo E. C. Harris

# Contents
	1)	Full-Stack website
		a)	Describes what Prometheus is
		b)	Contains updates as to Prometheus's development
		c)	Contains build plan / roadmap for Prometheus
			i)		includes PERT/GANTT chart
		d)	Contains mockup prototypes of Prometheus to communicate vision
		e)	Allows account creation for email updates
			i)		Account also allows community submission for library of commands
			ii)		Admin account allows review of submissions for implemenentation
				x)		Possible update to library for complilation?
		f) Stack:
			i)		MySQL
			ii)		Node
				x)		Sequelize
				xx)		Sessions?
				xxx)	Log sites for training (from main comp)
			iii)	Express
			iv)		Angular

	2) Prometheus
		a) Linux Terminal
			i)	Windows and Mac - FUTURE IMPLEMENTATION
		b) Vertically tabulated terminal
			i)		Allows commands to impact other tabs by reference
			ii)		Allows naming of tabs
			iii)	Tabs fully compartmentalized 
				x)		Securely closed environment (Docker?)
				xx)		Return only output data, not commands
				xxx)	Once closed, unreferenceable (All data scrubbed)
			iv)		Tabs can be minimized and/or closed for organization
			v)		Drag and drop - FUTURE IMPLEMENTATION
		c)	Also includes horizontal tabulation
			i)		No tab communication horizontally, only vertically
		d)	Terminals allow variable grammar to generate commands
			i)		Example:
				x)		grep "^[0-9]\{5\}$" Number
				xx)		Show from Number all strings with only exactly 5 numbers
				xxx)	Reduces mystifying commands, making the command line more accessible
			ii)		Variable grammar library pulled from community submissions
				x)		Future growth