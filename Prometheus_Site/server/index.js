//index.js
const express = require("express");
const cors = require("cors");
const app = express();
const PORT = 4000;

app.use(express.urlencoded({ extended: true }));
app.use(express.json());
app.use(cors());

var MongoClient = require('mongodb').MongoClient;

MongoClient.connect("mongodb://localhost:27017/Prometheus", function (err, db) {
    if (err) throw err;
});

app.get("/api", (req, res) => {
    res.json({
        message: "Hello world",
    });
});

app.get("/api/requirements", (req, res) => {
    
})

app.get("/api/test", (req, res) => {

})

app.listen(PORT, () => {
    console.log(`Server listening on ${PORT}`);
});
