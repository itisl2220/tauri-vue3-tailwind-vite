/* eslint-disable @typescript-eslint/no-explicit-any */
import { invoke } from "@tauri-apps/api";
import { ElNotification, MessageProps, NotifyTypedFn } from "element-plus";
import moment from "moment";
interface InvokeRustFunctionOptions {
  errorMessage?: boolean | undefined | MessageProps;
  errorNotification?: boolean | undefined | NotifyTypedFn;
  message?: boolean | undefined | MessageProps;
  notification?: boolean | undefined | NotifyTypedFn;
}

export function invokeRustFunction(
  method: string,
  params: any,
  options: InvokeRustFunctionOptions | undefined
) {
  return new Promise((resolve, reject) => {
    invoke(method, params)
      .then((res: any) => {
        if (options?.message) {
          if (typeof options.message === "boolean") {
            ElMessage({
              message: res,
              type: "success"
            });
          } else if (typeof options.message === "object") {
            ElMessage(options.message);
          }
        }
        if (options?.notification) {
          if (typeof options.notification === "boolean") {
            ElNotification({
              title: "Success",
              message: res,
              type: "success"
            });
          } else if (typeof options.notification === "object") {
            ElNotification(options.notification);
          }
        }
        resolve(res);
      })
      .catch(err => {
        if (options?.errorMessage) {
          if (typeof options.errorMessage === "boolean") {
            ElMessage({
              message: err,
              type: "error"
            });
          } else if (typeof options.errorMessage === "object") {
            ElMessage(options.errorMessage);
          }
        }
        if (options?.errorNotification) {
          if (typeof options.errorNotification === "boolean") {
            ElNotification({
              title: "Error",
              message: err,
              type: "error"
            });
          } else if (typeof options.errorNotification === "object") {
            ElNotification(options.errorNotification);
          }
        }
        reject(err);
      });
  });
}

/**
 * 时间戳转换为时间格式
 * @param timestamp 时间戳
 */
export const dateToStr = (timestamp: string) =>
  moment(new Date(parseInt(timestamp))).format("YYYY-MM-DD HH:mm");