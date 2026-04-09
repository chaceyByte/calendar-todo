import { ref } from 'vue'

// 简单的全局事件总线
export const newTaskEvent = ref(false)

export function triggerNewTask() {
  newTaskEvent.value = !newTaskEvent.value
}
