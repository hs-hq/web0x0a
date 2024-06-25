const jwt = require('jsonwebtoken');
const bcrypt = require('bcrypt');
const User = require("../model/User");
require('dotenv').config();
const parsetrace = require('parsetrace');

const secretKey = process.env.secretKey;

const login = async (req, res) => {
    const { username, password } = req.body;
    try {
        const user = await User.findOne({ username });
        if (!user) {
            return res.status(401).render('login', { error: 'Incorrect Information' });
        }

        const passwordMatch = await bcrypt.compare(password, user.password);
        if (!passwordMatch) {
            return res.status(401).render('login', { error: 'Incorrect Information' });
        }

        const token = jwt.sign({ userId: user._id, username: user.username, role: user.role }, secretKey);
        res.cookie('token', token);
        return res.redirect('/');
    } catch (error) {
        console.error('Error during login:', error);
        return res.status(500).json({ error: 'Failed to login' });
    }

};


const register = async (req, res, next) => {
    const { username, password, refcode } = req.body;

    try {
        const existingUser = await User.findOne({ username });
        if (existingUser) {
            return res.status(401).reder('register', { error: 'Username already exists' });
        }
        const referal_code = "not-the-real-referal-code"; // just a placeholder in the dev environment
        if (refcode.toString() !== referal_code) {
            return res.status(401).render('register', { error: 'Incorrect referal code' })
        }
        const hashedPassword = await bcrypt.hash(password, 10);
        const user = new User({
            username,
            password: hashedPassword,
        });
        await user.save();
        return res.redirect('/user/login');
    } catch (err) {
        console.error('Error during registration:', err);
        return res.status(402).json({ success: JSON.parse(parsetrace(err, { sources: true }).json()) });

    }
};

module.exports = {
    login,
    register,
};