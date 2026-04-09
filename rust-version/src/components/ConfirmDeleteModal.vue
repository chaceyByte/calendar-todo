<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="isVisible" class="modal-overlay" @click="handleCancel">
        <div class="modal-container" @click.stop>
          <div class="modal-content">
            <!-- Header -->
            <div class="modal-header">
              <div class="icon-wrapper warning">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
                  <line x1="12" y1="9" x2="12" y2="13"/>
                  <line x1="12" y1="17" x2="12.01" y2="17"/>
                </svg>
              </div>
              <div class="header-text">
                <h3 class="modal-title">确认删除</h3>
                <p class="modal-subtitle">此操作不可撤销</p>
              </div>
            </div>

            <!-- Body -->
            <div class="modal-body">
              <p class="body-text">
                确定要删除标签 <span class="text-highlight">"{{ tagName }}"</span> 吗？
              </p>
              <p class="body-hint" v-if="taskCount === 0">
                该标签当前没有关联的任务，可以安全删除。
              </p>
            </div>

            <!-- Footer -->
            <div class="modal-footer">
              <button class="btn-cancel" @click="handleCancel">
                取消
              </button>
              <button class="btn-delete" @click="handleConfirm">
                确认删除
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
  (e: 'confirm'): void
  (e: 'update:isVisible', value: boolean): void
}

const props = withDefaults(defineProps<Props>(), {
  tagName: '',
  taskCount: 0
})

const emit = defineEmits<Emits>()

const handleCancel = () => {
  emit('update:isVisible', false)
  emit('cancel')
}

const handleConfirm = () => {
  emit('update:isVisible', false)
  emit('confirm')
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
  backdrop-filter: blur(6px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.modal-container {
  background: var(--bg-card);
  border-radius: 24px;
  box-shadow: 0px 25px 50px -12px rgba(0, 0, 0, 0.25);
  max-width: 420px;
  width: 90%;
  animation: modalSlideIn 0.3s ease;
}

@keyframes modalSlideIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.modal-content {
  padding: 28px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.modal-header {
  display: flex;
  align-items: flex-start;
  gap: 16px;
}

.icon-wrapper {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.icon-wrapper.warning {
  background: #fef3cd;
}

.icon-wrapper svg {
  width: 24px;
  height: 24px;
  color: #d97706;
}

.header-text {
  flex: 1;
  padding-top: 4px;
}

.modal-title {
  font-size: 18px;
  font-weight: 600;
  color: #191c1e;
  margin: 0 0 4px 0;
}

.modal-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.modal-body {
  background: #f8f9fa;
  border-radius: 16px;
  padding: 16px;
}

.body-text {
  font-size: 14px;
  line-height: 1.6;
  color: #374151;
  margin: 0 0 8px 0;
}

.body-hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0;
}

.text-highlight {
  color: #191c1e;
  font-weight: 600;
}

.modal-footer {
  display: flex;
  gap: 12px;
  padding-top: 4px;
}

.btn-cancel {
  flex: 1;
  padding: 12px 20px;
  background: var(--bg-input);
  border: none;
  border-radius: 12px;
  font-size: 14px;
  font-weight: 500;
  color: #374151;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-cancel:hover {
  background: #e5e7eb;
}

.btn-delete {
  flex: 1;
  padding: 12px 20px;
  background: linear-gradient(135deg, var(--color-error) 0%, #dc2626 100%);
  border: none;
  border-radius: 12px;
  font-size: 14px;
  font-weight: 500;
  color: var(--bg-card);
  cursor: pointer;
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
  transition: all 0.2s ease;
}

.btn-delete:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px rgba(239, 68, 68, 0.4);
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
  transform: scale(0.95);
}
</style>
