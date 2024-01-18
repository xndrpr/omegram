
export let cachedDialogs = JSON.parse(localStorage.getItem("dialogs"));
export let cachedAuth = JSON.parse(localStorage.getItem("auth")) == "true" ? true : false;
export let cachedUser = {};
