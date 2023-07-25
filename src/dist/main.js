"use strict";
exports.__esModule = true;
var devtools_1 = require("@vue/devtools");
var vue_1 = require("vue");
require("element-plus/dist/index.css");
require("element-plus/theme-chalk/dark/css-vars.css");
var App_1 = require("./App");
require("./assets/main.postcss");
if (process.env.NODE_ENV === "development") {
    devtools_1["default"].connect("http://localhost", 8098);
}
vue_1.createApp(App_1["default"]).mount("#app");
