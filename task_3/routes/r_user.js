const express = require('express')
const router = express.Router();
const User = require("../model/User");

const {
    login,
    register
} = require('../middlewares/m_auth.js');


router.get('/login', (req, res) => {
    try {
        const token = req.cookies.session;
        if (token) {
            return res.redirect('/');
        }
        else {
            return res.render('login', { error: "" });
        }
    } catch (err) {
        res.render('login', { error: "" });
    }
});

router.get('/logout', (req, res) => {
    res.clearCookie('token');
    res.redirect('/');
});
router.post('/login', login)
router.get('/register',(req,res) => {
    try {
        const token = req.cookies.session;
        if (token) {
            return res.redirect('/');
        }
        else {
            return res.render('register', { error: "" });
        }
    } catch (err) {
        res.render('register', { error: "" });
    }
})
router.post('/register', register)

module.exports = router