/* eslint-disable @typescript-eslint/no-explicit-any */
import devtools from "@vue/devtools";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
import { createApp } from "vue";
import router from "./router";
import App from "./App";

import "./assets/main.postcss";
import "element-plus/dist/index.css";
import "element-plus/theme-chalk/dark/css-vars.css";
if (process.env.NODE_ENV === "development") {
  devtools.connect("http://localhost", 8098);
}
const store = createPinia();
store.use(piniaPluginPersistedstate);
createApp(App).use(store).use(router).mount("#app");
// 禁止右键和检查
//禁止F12
document.onkeydown = function (event: any) {
  const winEvent: any = window.event;
  if (winEvent && winEvent.keyCode == 123) {
    event.keyCode = 0;
    event.returnValue = false;
  }
  if (winEvent && winEvent.keyCode == 13) {
    winEvent.keyCode = 505;
  }
};

//屏蔽右键菜单
document.oncontextmenu = function (event: any) {
  if (window.event) {
    event = window.event;
  }
  try {
    const the = event.srcElement;
    if (
      !(
        (the.tagName == "INPUT" && the.type.toLowerCase() == "text") ||
        the.tagName == "TEXTAREA"
      )
    ) {
      return false;
    }
    return true;
  } catch (e) {
    return false;
  }
};
