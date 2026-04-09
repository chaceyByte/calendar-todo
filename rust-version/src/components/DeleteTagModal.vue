<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="isVisible" class="modal-overlay" @click="handleCancel">
        <div class="modal-container" @click.stop>
          <div class="modal-content">
            <!-- Header -->
            <div class="modal-header">
              <div class="icon-wrapper">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"/>
                  <line x1="12" y1="8" x2="12" y2="12"/>
                  <line x1="12" y1="16" x2="12.01" y2="16"/>
                </svg>
              </div>
              <div class="header-text">
                <h3 class="modal-title">无法删除标签</h3>
                <p class="modal-subtitle">该标签当前正在被使用中</p>
              </div>
            </div>

            <!-- Body -->
            <div class="modal-body">
              <p class="body-text">
                <span class="text-normal">标签 </span>
                <span class="text-highlight">"{{ tagName }}"</span>
                <span class="text-normal"> 已关联到 </span>
                <span class="text-number">{{ taskCount }}</span>
                <span class="text-normal"> 个进行中的任务。在删除此标签前，请先移除或更换这些任务的标签属性。</span>
              </p>
            </div>

            <!-- Footer -->
            <div class="modal-footer">
              <button class="btn-cancel" @click="handleCancel">
                取消
              </button>
              <button class="btn-primary" @click="handleViewTasks">
                查看相关任务
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
interface Props {
  isVisible: boolean
  tagName?: string
  taskCount?: number
}

interface Emits {
  (e: 'cancel'): void
  (e: 'view-tasks'): void
  (e: 'update:isVisible', value: boolean): void
}

const props = withDefaults(defineProps<Props>(), {
  tagName: '高优先级',
  taskCount: 24
})

const emit = defineEmits<Emits>()

const handleCancel = () => {
  emit('update:isVisible', false)
  emit('cancel')
}

const handleViewTasks = () => {
  emit('view-tasks')
  emit('update:isVisible', false)
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(45, 49, 51, 0.2);
  backdrop-filter: blur(2px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.modal-container {
  background: var(--bg-card);
  border-radius: 48px;
  box-shadow: 0px 25px 50px -12px rgba(0, 0, 0, 0.25);
  max-width: 480px;
  width: 90%;
  animation: modalSlideIn 0.3s ease;
}

@keyframes modalSlideIn {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-content {
  padding: 32px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.modal-header {
  display: flex;
  align-items: center;
  gap: 16px;
}

.icon-wrapper {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: #ffdad6;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.icon-wrapper svg {
  width: 24px;
  height: 24px;
  color: var(--color-error);
}

.header-text {
  flex: 1;
}

.modal-title {
  font-size: 18px;
  font-weight: 600;
  color: #191c1e;
  margin: 0 0 4px 0;
}

.modal-subtitle {
  font-size: 14px;
  font-weight: 500;
  color: #516072;
  margin: 0;
}

.modal-body {
  background: var(--bg-input);
  border-radius: 32px;
  padding: 16px;
}

.body-text {
  font-size: 12px;
  line-height: 20px;
  margin: 0;
}

.text-normal {
  color: #516072;
  font-weight: 500;
}

.text-highlight {
  color: #191c1e;
  font-weight: 600;
}

.text-number {
  color: #191c1e;
  font-weight: 700;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 8px;
}

.btn-cancel {
  padding: 10px 24px;
  background: transparent;
  border: none;
  border-radius: 9999px;
  font-size: 14px;
  font-weight: 500;
  color: #516072;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-cancel:hover {
  background: var(--bg-input);
}

.btn-primary {
  padding: 10px 24px;
  background: var(--color-primary);
  border: none;
  border-radius: 9999px;
  font-size: 14px;
  font-weight: 500;
  color: var(--bg-card);
  cursor: pointer;
  box-shadow: 0px 4px 6px -1px rgba(0, 0, 0, 0.1), 0px 2px 4px -2px rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
}

.btn-primary:hover {
  background: #004a9a;
  transform: translateY(-1px);
}

/* Transition */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .modal-container,
.modal-leave-active .modal-container {
  transition: transform 0.3s ease;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  transform: translateY(-20px);
}
</style>