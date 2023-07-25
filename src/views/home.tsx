/* eslint-disable @typescript-eslint/no-empty-function */
import { useSettingStore } from "../store";

export default defineComponent({
  name: "HomePage",
  setup() {
    const state = useSettingStore();
    onMounted(() => {});

    return () => <div class="h-full w-full flex flex-col">主页</div>;
  }
});
