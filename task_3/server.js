// Essential imports
const express = require("express");
const connectDB = require("./db");
const cookieParser = require("cookie-parser");
const mongoSanitize = require("express-mongo-sanitize");
const jwt = require("jsonwebtoken");
var cors = require("cors");
require("dotenv").config();
var path = require("path");
const app = express();
const port = 3000;

// Setting up my view Engine
app.set("views", path.join(__dirname, "views"));
app.set("view engine", "ejs");
app.use("/static", express.static(path.join(__dirname, "static")));

// Custom imports
const user_router = require("./routes/r_user.js");
const admin_router = require("./routes/r_admin.js");
const voting = require("./middlewares/voting");
const Volunteer = require("./model/Volunteer");
const vote = require("./bot");

// Connecting to DB
connectDB();

//Setting up my middlewares
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(cookieParser());
app.use(
  mongoSanitize({
    onSanitize: ({ req, key }) => {
      // Throw an error
      // Catch with express error handler
    },
  })
);

app.use((req, res, next) => {
  res.setHeader(
    "Content-Security-Policy",
    "base-uri 'self';frame-ancestors 'none';img-src 'self';object-src 'none';script-src 'self' 'sha256-txnYJbxuXC7iPfO+oOUS8LAK3u9eNhYYE68MxzrusLc=' 'unsafe-eval' https://*.google.com/ https://kit.fontawesome.com/; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com/"
  );
  next();
});

const secretKey = process.env.secretKey;
const FLAG1 = process.env.FLAG1;
// Routes
app.get("", (req, res) => {
  var loggedIn = false;
  var isAdmin = false;
  const flag = "";
  const token = req.cookies.token ? req.cookies.token : null;
  if (token) {
    jwt.verify(token, secretKey, (err, decoded) => {
      if (err) {
        return res.status(401).render("home", { loggedIn, isAdmin, flag });
      }
      role = decoded.role;
      if (role === "admin") {
        isAdmin = true;
      }
      loggedIn = true;
      const flag = FLAG1;
      return res.status(200).render("home", { loggedIn, isAdmin, flag });
    });
  }
  return res.status(200).render("home", { loggedIn, isAdmin, flag });
});

app.get("/about", (req, res) => {
  var loggedIn = false;
  var isAdmin = false;
  const token = req.cookies.token ? req.cookies.token : null;
  if (token) {
    jwt.verify(token, secretKey, (err, decoded) => {
      if (err) {
        return res.status(401).render("aboutus", { loggedIn, isAdmin });
      }
      role = decoded.role;
      if (role === "admin") {
        isAdmin = true;
      }
      loggedIn = true;
    });
  }
  return res.status(200).render("aboutus", { loggedIn, isAdmin });
});

app.get("/volunteers", async (req, res) => {
  try {
    var loggedIn = false;
    var isAdmin = false;
    const token = req.cookies.token ? req.cookies.token : null;

    if (token) {
      jwt.verify(token, secretKey, (err, decoded) => {
        if (err) {
          return res.status(401).render("volunteers", { loggedIn, isAdmin });
        }
        role = decoded.role;
        if (role === "admin") {
          isAdmin = true;
        }
        loggedIn = true;
      });
    }
    const volunteers = await Volunteer.find({});
    res.render("volunteers", { volunteers, loggedIn, isAdmin });
  } catch (err) {
    console.error("Error retrieving volunteers:", err);
    res.status(500).json({ error: "Failed to retrieve volunteers" });
  }
});

app.get("/volunteers/:volunteerID", voting, async (req, res) => {
  try {
    var loggedIn = false;
    var isAdmin = false;
    const token = req.cookies.token ? req.cookies.token : null;
    if (token) {
      jwt.verify(token, secretKey, (err, decoded) => {
        if (err) {
          return res.status(401).redirect("/");
        }
        role = decoded.role;
        if (role === "admin") {
          isAdmin = true;
        }
        loggedIn = true;
      });
    } else {
      return res.status(401).redirect("/");
    }
    const volunteerID = parseInt(req.params.volunteerID);
    const volunteer = await Volunteer.findOne({ volunteerID });

    if (volunteer) {
      res.render("volunteerDetails", {
        volunteer,
        ...res.user,
        loggedIn,
        isAdmin,
      });
    } else {
      res.status(404).json({ error: "Volunteer not found" });
    }
  } catch (error) {
    console.error("error while loading the page: ", error);
    res.status(500).json({ error: "Internal server error" });
  }
});

app.post("/vote", async (req, res) => {
  try {
    var loggedIn = false;
    var isAdmin = false;
    const token = req.cookies.token ? req.cookies.token : null;
    if (token) {
      jwt.verify(token, secretKey, (err, decoded) => {
        if (err) {
          return res.status(401).render("home", { loggedIn, isAdmin });
        }
        role = decoded.role;
        if (role === "admin") {
          isAdmin = true;
        }
        loggedIn = true;
      });
    } else {
      return res.status(401).render("home", { loggedIn, isAdmin });
    }
    if (!req.body.goal) {
      throw new Error("Invalid input.");
    }
    if (typeof req.body.goal !== "string") {
      throw new Error("Invalid input.");
    }

    await vote(req.body.goal, req.body.volunteer);
    return res.json({ success: true });
  } catch (err) {
    console.log(err);
    return res
      .status(500)
      .json({ error: "error while handling the request", err: err });
  }
});

// user routes
app.use("/user", user_router);
app.use("/admin", admin_router);

// 404 page handling
app.all("*", (req, res) => {
  res.status(404).render("error404");
});

// Error handling
app.use(function (err, req, res, next) {
  console.log(err);
  res.status(500).send("Internal Server Error");
});

// Starting the app
app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});
