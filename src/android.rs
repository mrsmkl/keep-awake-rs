
use jni::JavaVM;
use jni::objects::{ JValue, JObject, GlobalRef };

pub struct Holder {
    wake_lock: GlobalRef
}

// const FLAG_KEEP_SCREEN_ON: jni::sys::jint = 128;
const PARTIAL_WAKE_LOCK: jni::sys::jint = 1;
const FULL_WAKE_LOCK: jni::sys::jint = 26;

fn get_jvm() -> JavaVM {
    unsafe { JavaVM::from_raw(ndk_context::android_context().vm().cast()) }.unwrap()
}

pub fn inhibit_system(name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    let ctx = ndk_context::android_context();
    if ctx.context().is_null() {
        return Err("Android context not set! Use ndk_context::initialize_android_context or ndk_glue".into());
    }

    let vm = get_jvm();
    let mut env = vm.attach_current_thread().unwrap();

    let result = (|| -> jni::errors::Result<Holder> {
        let power = env.new_string("power")?;
        let name = env.new_string(&format!("{name}:{reason}"))?;

        let pm = env.call_method(unsafe { JObject::from_raw(ctx.context().cast()) }, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::from(&power)])?.l()?;

        let wake_lock = env.call_method(&pm, "newWakeLock", "(ILjava/lang/String;)Landroid/os/PowerManager$WakeLock;", &[JValue::from(PARTIAL_WAKE_LOCK), JValue::from(&name)])?;
        let wake_lock = env.new_global_ref(wake_lock.l()?)?;

        env.call_method(&wake_lock, "acquire", "()V", &[])?;

        Ok(Holder { wake_lock })
    })();
    match result {
        Ok(x) => Ok(x),
        Err(e @ jni::errors::Error::JavaException) => {
            let _ = env.exception_describe();
            let _ = env.exception_clear();
            Err(Box::new(e))
        },
        Err(e) => Err(Box::new(e))
    }
}

pub fn inhibit_display(name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    let ctx = ndk_context::android_context();
    if ctx.context().is_null() {
        return Err("Android context not set! Use ndk_context::initialize_android_context or ndk_glue".into());
    }

    let vm = get_jvm();
    let mut env = vm.attach_current_thread().unwrap();

    let result = (|| -> jni::errors::Result<Holder> {
        let power = env.new_string("power")?;
        let name = env.new_string(&format!("{name}:{reason}"))?;

        let pm = env.call_method(unsafe { JObject::from_raw(ctx.context().cast()) }, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::from(&power)])?.l()?;

        let wake_lock = env.call_method(&pm, "newWakeLock", "(ILjava/lang/String;)Landroid/os/PowerManager$WakeLock;", &[JValue::from(FULL_WAKE_LOCK), JValue::from(&name)])?;
        let wake_lock = env.new_global_ref(wake_lock.l()?)?;

        env.call_method(&wake_lock, "acquire", "()V", &[])?;

        Ok(Holder { wake_lock })
    })();
    match result {
        Ok(x) => Ok(x),
        Err(e @ jni::errors::Error::JavaException) => {
            let _ = env.exception_describe();
            let _ = env.exception_clear();
            Err(Box::new(e))
        },
        Err(e) => Err(Box::new(e))
    }

    // FLAG_KEEP_SCREEN_ON way, but it requires to be called on the UI thread, which is tricky with pure JNI calls and this crate's API

    /*let vm = unsafe { jni::JavaVM::from_raw(ndk_context::android_context().vm().cast()) }?;
    let context = unsafe { jni::objects::JObject::from_raw(ndk_context::android_context().context().cast()) };
    let mut env = vm.attach_current_thread()?;

    let window = env.call_method(context, "getWindow", "()Landroid/view/Window;", &[])?.l()?;

    env.call_method(window, "addFlags", "(I)V", &[FLAG_KEEP_SCREEN_ON.into()])?;

    Ok(Holder { })*/
}

impl Drop for Holder {
    fn drop(&mut self) {
        let vm = get_jvm();
        let mut env = vm.attach_current_thread().unwrap();

        match env.call_method(&self.wake_lock, "release", "()V", &[]) {
            Ok(_) => { },
            Err(e @ jni::errors::Error::JavaException) => {
                let _ = env.exception_describe();
                let _ = env.exception_clear();
                log::error!("Failed to release the wake lock: {e:?}");
            },
            Err(e) => { log::error!("Failed to release the wake lock: {e:?}"); }
        }

        // FLAG_KEEP_SCREEN_ON way
        /*if let Err(e) = (|| -> Result<(), Box<dyn std::error::Error>> {
            let vm = unsafe { jni::JavaVM::from_raw(ndk_context::android_context().vm().cast()) }?;
            let context = unsafe { jni::objects::JObject::from_raw(ndk_context::android_context().context().cast()) };
            let mut env = vm.attach_current_thread()?;

            let window = env.call_method(context, "getWindow", "()Landroid/view/Window;", &[])?.l()?;

            env.call_method(window, "clearFlags", "(I)V", &[FLAG_KEEP_SCREEN_ON.into()])?;
            Ok(())
        })() {
            log::error!("Failed to clear the KEEP_SCREEN_ON flag: {e:?}");
        }*/
    }
}
