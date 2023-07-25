use crate::error::Result;
use tauri::Window;
use webview2_com::{
    take_pwstr, GetCookiesCompletedHandler,
    Microsoft::Web::WebView2::Win32::{ICoreWebView2Cookie, ICoreWebView2_2},
};
use windows::core::{HSTRING, PWSTR, Interface};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
}
/**
 * 运行获取cookie
 */
pub async fn get_win_cookie(
    win: &Window,
    // done_tx: oneshot::Sender<Vec<Cookie>>,
    url: &'static str,
) -> Result<Vec<Cookie>> {
    let (done_tx, done_rx) = oneshot::channel::<Vec<Cookie>>();
    win.with_webview(move |webview| unsafe {
        let core = webview.controller().CoreWebView2().unwrap();
        let core2 = Interface::cast::<ICoreWebView2_2>(&core).unwrap();
        let uri = HSTRING::from(url);
        let manager = core2.CookieManager().unwrap();
        GetCookiesCompletedHandler::wait_for_async_operation(
            Box::new(move |handler| {
                manager.GetCookies(&uri, &handler)?;
                Ok(())
            }),
            Box::new(move |hresult, list| {
                hresult?;
                match list {
                    Some(list) => {
                        let mut count: u32 = 0;
                        list.Count(&mut count)?;
                        // tracing::info!("count: {}", count);
                        let mut cookie_str = vec![];
                        for i in 0..count {
                            let cookie: ICoreWebView2Cookie = list.GetValueAtIndex(i)?;
                            let mut name = PWSTR::null();
                            let mut value = PWSTR::null();
                            let mut domain = PWSTR::null();
                            let mut path = PWSTR::null();
                            cookie.Name(&mut name)?;
                            cookie.Value(&mut value)?;
                            cookie.Domain(&mut domain)?;
                            cookie.Path(&mut path)?;
                            cookie_str.push(Cookie {
                                name: take_pwstr(name),
                                value: take_pwstr(value),
                                domain: take_pwstr(domain),
                                path: take_pwstr(path),
                            });
                        }
                        done_tx.send(cookie_str).unwrap();
                    }
                    None => {
                        tracing::info!("list is none");
                    }
                };
                Ok(())
            }),
        )
        .unwrap();
    })
    .unwrap();
    let cookie_str = done_rx.await.unwrap();
    // tracing::info!("cookie_str: {:#?}", cookie_str);
    Ok(cookie_str)
}
