function sendErrorTip(errorInfo) {
  // 暂时仅在控制台输出错误信息，后续可接入错误上报接口
  console.error('[ErrorHandler]', errorInfo.type, errorInfo.message, errorInfo.stack)
}

window.addEventListener('unhandledrejection', (event) => {
  sendErrorTip({
    type: '前端',
    message: `错误信息: ${event.reason}`,
    stack: `调用栈: ${event.reason?.stack || '没有调用栈信息'}`,
  });
});
