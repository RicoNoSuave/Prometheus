//index.js
const express = require("express");
const cors = require("cors");
const app = express();
const PORT = 4000;

app.use(express.urlencoded({ extended: true }));
app.use(express.json());
app.use(cors());

app.get("/api", (req, res) => {
    res.json({
        message: "Hello world",
    });
});

app.get("/api/about", (req, res) => {
    var fs = require("fs");
    var text = fs.readFileSync("./about.md").toString('utf-8');
    res.json({
        about: text,
    });
});

app.listen(PORT, () => {
    console.log(`Server listening on ${PORT}`);
});