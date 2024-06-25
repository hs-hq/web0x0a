const puppeteer = require("puppeteer");
const jwt = require("jsonwebtoken");
const User = require("./model/User");
require("dotenv").config();

const secretKey = process.env.secretKey;
const FLAG = process.env.FLAG2;

const vote = async (goal, volunteer) => {
  const userQuery = User.findOne({ username: "admin" });
  var token = "";
  userQuery.exec().then((user) => {
    if (!user) {
      console.error("User not found");
    } else {
      token = jwt.sign(
        { userId: user._id, username: user.username, role: user.role },
        secretKey
      );
    }
  });

  const browser = await puppeteer.launch({
    headless: true,
    args: [
      "--disable-gpu",
      "--no-sandbox",
      "--disable-setuid-sandbox",
      "--js-flags=--noexpose_wasm,--jitless",
    ],
  });

  const page = await browser.newPage();

  await page.setCookie({
    name: "flag",
    value: FLAG,
    domain: "localhost",
    path: "/",
  });
  await page.goto(`http://localhost:3000/`);

  await new Promise((resolve) => setTimeout(resolve, 1000));

  await browser.close();
};

module.exports = vote;
