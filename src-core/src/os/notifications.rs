pub mod os {
    use dbus::arg::Variant;
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
    use dbus::blocking::{Connection, Proxy};
    use serde_repr::Serialize_repr;
    use std::collections::HashMap;
    use std::time::Duration;

    use crate::os::{DBUS_CONNECTION, connect_dbus};
    #[inline]
    pub fn with_proxy<'a>(conn: &'a Connection) -> Proxy<'a, &'a Connection> {
        conn.with_proxy(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            Duration::from_millis(100),
        )
    }
    #[derive(Debug, Clone, Copy, Serialize_repr)]
    #[repr(u8)]
    pub enum IsInhibited {
        NO = 0,
        ByOyasumi = 1,
        ByOther = 2,
        Unknown = 3,
    }
    static mut INHIBIT_TOKEN: u32 = u32::MAX;
    pub async fn is_inibited() -> IsInhibited {
        if connect_dbus().await
            && let Some(conn) = DBUS_CONNECTION.lock().await.as_ref()
        {
            match with_proxy(&conn).get::<bool>("org.freedesktop.Notifications", "Inhibited") {
                Ok(v) => unsafe {
                    if !v {
                        INHIBIT_TOKEN = u32::MAX;
                        IsInhibited::NO
                    } else if INHIBIT_TOKEN == u32::MAX {
                        IsInhibited::ByOther
                    } else {
                        IsInhibited::ByOyasumi
                    }
                },
                Err(err) => {
                    unsafe { INHIBIT_TOKEN = u32::MAX };
                    log::error!("failed to get Inhibited: {:?}", err);
                    IsInhibited::Unknown
                }
            }
        } else {
            log::error!(
                "[core] failed to check org.freedesktop.Notifications.Inhibited not connected to dbus"
            );
            return IsInhibited::Unknown;
        }
    }
    pub async fn inhibit(reason: String) -> bool {
        if connect_dbus().await
            && let Some(conn) = DBUS_CONNECTION.lock().await.as_ref()
        {
            let mut sucess = true;
            let mut args = HashMap::new();
            args.insert("", Variant(""));
            let handle: (u32,) = with_proxy(&conn)
                .method_call(
                    "org.freedesktop.Notifications",
                    "Inhibit",
                    ("OyasumiVR", reason, args),
                )
                .unwrap_or_else(|err| {
                    log::error!("failed to Inhibit notifications: {:?}", err);
                    sucess = false;
                    (u32::MAX,)
                });
            unsafe { INHIBIT_TOKEN = handle.0 };
            sucess
        } else {
            log::error!("[core] failed to Inhibit notifications not connected to dbus");
            false
        }
    }
    pub async fn un_inhibit() -> bool {
        if connect_dbus().await
            && let Some(conn) = DBUS_CONNECTION.lock().await.as_ref()
        {
            let mut sucess = true;
            let _: () = with_proxy(&conn)
                .method_call(
                    "org.freedesktop.Notifications",
                    "UnInhibit",
                    ((unsafe { INHIBIT_TOKEN },),),
                )
                .unwrap_or_else(|err| {
                    log::error!("failed to UnInhibit notifications: {:?}", err);
                    sucess = false;
                });
            sucess
        } else {
            log::error!("[core] failed to UnInhibit notifications not connected to dbus");
            false
        }
    }
}
