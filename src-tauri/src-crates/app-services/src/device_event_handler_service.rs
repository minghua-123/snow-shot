use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use device_query::{
    CallbackGuard, DeviceEvents, DeviceEventsHandlerInnerThread, Keycode, MouseButton,
    MousePosition,
};

const DEVICE_EVENT_HANDLER_FPS: u64 = 100;

pub struct DeviceEventHandlerService {
    fps: u64,
    /* 设备事件处理 */
    device_event_handler: Option<DeviceEventsHandlerInnerThread>,
    is_resetting: AtomicBool,
}

impl DeviceEventHandlerService {
    pub fn new() -> Self {
        Self {
            fps: DEVICE_EVENT_HANDLER_FPS,
            device_event_handler: None,
            is_resetting: AtomicBool::new(false),
        }
    }

    pub fn set_fps(&mut self, fps: u64) {
        self.fps = fps;
    }

    pub fn get_device_event_handler(&mut self) -> Result<&DeviceEventsHandlerInnerThread, String> {
        if let Some(ref handler) = self.device_event_handler {
            return Ok(handler);
        }

        // 防止在重置过程中重复创建
        if self.is_resetting.load(Ordering::SeqCst) {
            return Err("[DeviceEventHandlerService] Handler is being reset".to_string());
        }

        let handler = DeviceEventsHandlerInnerThread::new(Duration::from_millis(1000 / self.fps));

        self.device_event_handler = Some(handler);
        Ok(self.device_event_handler.as_ref().unwrap())
    }

    /// 重置设备事件处理器，用于解决后台挂起后无响应的问题
    pub fn reset(&mut self) {
        self.is_resetting.store(true, Ordering::SeqCst);
        self.device_event_handler.take();
        self.is_resetting.store(false, Ordering::SeqCst);
        log::info!("[DeviceEventHandlerService] Handler reset completed");
    }

    pub fn on_mouse_move<Callback: Fn(&MousePosition) + Sync + Send + 'static>(
        &mut self,
        callback: Callback,
    ) -> Result<CallbackGuard<Callback>, String> {
        Ok(self.get_device_event_handler()?.on_mouse_move(callback))
    }

    pub fn on_mouse_down<Callback: Fn(&MouseButton) + Sync + Send + 'static>(
        &mut self,
        callback: Callback,
    ) -> Result<CallbackGuard<Callback>, String> {
        Ok(self.get_device_event_handler()?.on_mouse_down(callback))
    }

    pub fn on_mouse_up<Callback: Fn(&MouseButton) + Sync + Send + 'static>(
        &mut self,
        callback: Callback,
    ) -> Result<CallbackGuard<Callback>, String> {
        Ok(self.get_device_event_handler()?.on_mouse_up(callback))
    }

    pub fn on_key_down<Callback: Fn(&Keycode) + Sync + Send + 'static>(
        &mut self,
        callback: Callback,
    ) -> Result<CallbackGuard<Callback>, String> {
        Ok(self.get_device_event_handler()?.on_key_down(callback))
    }

    pub fn on_key_up<Callback: Fn(&Keycode) + Sync + Send + 'static>(
        &mut self,
        callback: Callback,
    ) -> Result<CallbackGuard<Callback>, String> {
        Ok(self.get_device_event_handler()?.on_key_up(callback))
    }

    pub fn release(&mut self) {
        self.reset();
    }
}
