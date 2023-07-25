import { KeepAlive, Transition, createVNode, defineComponent } from "vue";
import { RouterView, useRoute } from "vue-router";
import {
  ElContainer,
  ElAside,
  ElMain,
  ElScrollbar,
  ElHeader,
  ElButton
} from "element-plus";
import { Icon } from "@iconify/vue";
import { exit } from "@tauri-apps/api/process";
import { appWindow } from "@tauri-apps/api/window";
import router from "./router";
export default defineComponent({
  name: "App",
  setup() {
    const activeIndex = ref("1");
    const menuList = ref([
      {
        name: "首页",
        icon: "mdi:home",
        index: "1",
        path: "/"
      }
      // {
      //   name: "选品列表",
      //   icon: "mdi:format-list-bulleted",
      //   index: "2",
      //   path: "/product"
      // },
      // {
      //   name: "文案设置",
      //   icon: "mdi:file-document-edit-outline",
      //   index: "3",
      //   path: "/doc-set"
      // }
    ]);
    const route = useRoute();
    const notMenuList = ref(["/log", "/live_state"]);
    return () =>
      notMenuList.value.includes(route.path) ? (
        <RouterView />
      ) : (
        <div class="h-screen w-screen bg-white">
          <ElContainer class="h-screen w-screen">
            <ElAside width="200px" class={"h-screen bg-neutral-50"}>
              <div class={"h-screen"}>
                <div
                  class={
                    "h-[62px] w-full flex justify-center items-center flex-nowrap"
                  }
                >
                  <div class={"text-gray-950 font-black text-2xl"}>
                    直播工具
                  </div>
                </div>
                <div>
                  <ElScrollbar
                    class={"mt-[40px]"}
                    viewClass={"h-[calc(100vh-102px)]"}
                  >
                    {/* 左边菜单栏 */}
                    <div class={"h-full flex flex-col px-8 text-zinc-900"}>
                      {menuList.value.map(item => {
                        return (
                          <div
                            onClick={() => {
                              activeIndex.value = item.index;
                              router.push(item.path);
                            }}
                            class={
                              "h-[30px] w-[160px] hover:bg-neutral-200 flex flex-nowrap items-center mb-4 p-2 rounded" +
                              (route.path == item.path ? " bg-blue-300" : "")
                            }
                          >
                            <Icon class="text-2xl mr-2" icon={item.icon}></Icon>
                            {item.name}
                          </div>
                        );
                      })}
                    </div>
                  </ElScrollbar>
                </div>
              </div>
            </ElAside>
            <ElMain
              class={"h-screen bg-white text-zinc-950 p-0"}
              style={{
                padding: 0
              }}
            >
              <div class="h-full w-full flex-nowrap flex flex-col">
                <ElHeader data-tauri-drag-region height="62px">
                  <div class="flex flex-nowrap items-center h-full ">
                    <div
                      data-tauri-drag-region
                      class="h-full w-full flex justify-center items-center flex-nowrap"
                    ></div>
                    <div>
                      {/* 窗口控制按钮 */}
                      <div class="h-full w-full flex justify-end items-center flex-nowrap">
                        <div class="h-full flex justify-center items-center flex-nowrap">
                          {/* 最小化 */}
                          <ElButton
                            text
                            onClick={() => {
                              appWindow.minimize();
                            }}
                          >
                            <Icon
                              class="text-neutral-900 text-2xl"
                              icon="mdi:window-minimize"
                            ></Icon>
                          </ElButton>
                          {/* 最大化 */}
                          <ElButton
                            text
                            onClick={() => {
                              appWindow.isMaximized().then(isMaximized => {
                                if (isMaximized) {
                                  appWindow.unmaximize();
                                } else {
                                  appWindow.maximize();
                                }
                              });
                            }}
                          >
                            <Icon
                              class="text-neutral-900 text-2xl"
                              icon="mdi:window-maximize"
                            ></Icon>
                          </ElButton>
                          {/* 关闭 */}
                          <ElButton
                            text
                            onClick={() => {
                              exit();
                            }}
                          >
                            <Icon
                              class="text-neutral-900 text-2xl"
                              icon="mdi:window-close"
                            ></Icon>
                          </ElButton>
                        </div>
                      </div>
                    </div>
                  </div>
                </ElHeader>
                <div class="h-full w-full px-4">
                  <RouterView
                    v-slots={{
                      default: ({ Component }: { Component: VNode }) => {
                        return Component ? (
                          <div key={route.path} style={{ height: "100%" }}>
                            <Transition mode="out-in">
                              <KeepAlive>{createVNode(Component)}</KeepAlive>
                            </Transition>
                          </div>
                        ) : null;
                      }
                    }}
                  />
                </div>
              </div>
            </ElMain>
          </ElContainer>
        </div>
      );
  }
});
