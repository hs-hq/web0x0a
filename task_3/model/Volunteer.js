const Mongoose = require("mongoose");

const VolunteerSchema = new Mongoose.Schema({
    volunteerID: {
        type: Number,
        unique:true
    },
    firstName: {
        type: String,
        minlength: 3,
        required: true,
    },
    lastName: {
        type: String,
        minlength: 3,
        required: true,
    },
    email: {
        type: String,
        required: true,
    },
});




const Volunteer = Mongoose.model("volunteer", VolunteerSchema);

module.exports = Volunteer;