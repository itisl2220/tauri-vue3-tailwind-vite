"use strict";
exports.__esModule = true;
var vue_1 = require("vue");
var element_plus_1 = require("element-plus");
exports["default"] = vue_1.defineComponent({
    name: "App",
    setup: function () {
        return function () { return (React.createElement("div", { "class": "h-screen w-screen" },
            React.createElement(element_plus_1.ElContainer, { "class": "h-screen w-screen" },
                React.createElement(element_plus_1.ElAside, { width: "120px" },
                    React.createElement(element_plus_1.ElScrollbar, null,
                        React.createElement(element_plus_1.ElMenu, { "class": "h-screen" },
                            React.createElement(element_plus_1.ElMenuItem, null)))),
                React.createElement(element_plus_1.ElMain, null, "Main")))); };
    }
});
