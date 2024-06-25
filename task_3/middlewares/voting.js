const voting = (req, res, next) => {

    res.user = {
      text: "Voting System",
      purpose: {
        goal: "vote",
      },
    };
    if (req.query.goal) {
      res.user = { ...res.user, ...req.query };
    }
    next();
  }

module.exports = voting