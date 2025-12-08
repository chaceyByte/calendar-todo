// 测试Ctrl+Z快捷键功能
console.log('Ctrl+Z快捷键测试')

// 模拟键盘事件
function simulateKeyDown(key, ctrlKey = false) {
  const event = new KeyboardEvent('keydown', {
    key: key,
    ctrlKey: ctrlKey,
    bubbles: true,
    cancelable: true
  })

  console.log(`模拟按键: key=${key}, ctrlKey=${ctrlKey}`)
  document.dispatchEvent(event)
}

// 测试Ctrl+Z
console.log('测试Ctrl+Z快捷键...')
simulateKeyDown('z', true)

// 测试普通Z键（不应触发撤销）
console.log('测试普通Z键（不应触发撤销）...')
simulateKeyDown('z', false)

// 测试Ctrl+A（不应触发撤销）
console.log('测试Ctrl+A（不应触发撤销）...')
simulateKeyDown('a', true)

console.log('测试完成！请查看控制台日志确认快捷键功能是否正常。')