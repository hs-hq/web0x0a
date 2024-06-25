const express = require('express')
const router = express.Router();
const jwt = require('jsonwebtoken');
require('dotenv').config();


const secretKey = process.env.secretKey;

router.get('', (req, res) => {
    var loggedIn = false;
    var isAdmin = false;
    const token = req.cookies.token ? req.cookies.token : null;
    if (token) {
        jwt.verify(token, secretKey, (err, decoded) => {
            if (err) {
                return res.status(401).redirect('/');
            }
            role = decoded.role;
            if (role === 'admin') {
                isAdmin = true;
                return res.render('admin')
            }
            else {
                return res.status(401).redirect('/');
            }
        });
    }
    else{
        return res.status(401).redirect('/');
    }
});

router.get('/addentry', (req,res) => {
    var loggedIn = false;
    var isAdmin = false;
    var entry = {}
    const token = req.cookies.token ? req.cookies.token : null;
    if (token) {
        jwt.verify(token, secretKey, (err, decoded) => {
            if (err) {
                return res.status(401).redirect('/');
            }
            role = decoded.role;
            if (role === 'admin') {
                isAdmin = true;
                if (Object.keys(req.query).length === 0){
                    var volunteer = "volunteer";
                    var place = "place";
                    var price = "price";
                }else{
                    var {volunteer,place,price} = req.query
                }
                entry[volunteer]={}
                entry[volunteer][place]=price;
                return res.render('addentry',req.query)
            }
            else {
                return res.status(401).redirect('/');
            }
        });
    }
    else{
        return res.status(401).redirect('/');
    }    
});

module.exports = router