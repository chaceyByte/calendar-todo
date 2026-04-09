<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="isVisible" class="modal-overlay" @click="handleCancel">
        <div class="modal-container" @click.stop>
          <!-- Header -->
          <div class="modal-header">
            <h3 class="modal-title">{{ isEditMode ? '编辑标签' : '新建标签' }}</h3>
            <button class="close-btn" @click="handleCancel">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>

          <!-- Body -->
          <div class="modal-body">
            <!-- Tag Name Input -->
            <div class="input-field">
              <label class="field-label">标签名称</label>
              <div class="input-wrapper">
                <input
                  v-model="tagName"
                  type="text"
                  placeholder="新标签项目"
                  class="text-input"
                  @keyup.enter="handleSave"
                />
              </div>
            </div>

            <!-- Color Picker -->
            <div class="color-picker">
              <label class="field-label">选择识别色</label>
              <div class="color-options">
                <div class="color-column">
                  <button
                    class="color-btn"
                    :class="{ active: selectedColor === 'var(--color-primary)' }"
                    style="background: var(--color-primary)"
                    @click="selectedColor = 'var(--color-primary)'"
                  >
                    <span v-if="selectedColor === 'var(--color-primary)'" class="check-icon">
                      <svg viewBox="0 0 24 24" fill="none" stroke="var(--bg-card)" stroke-width="3">
                        <polyline points="20 6 9 17 4 12"/>
                      </svg>
                    </span>
                  </button>
                  <button class="remove-color-btn" @click="selectedColor = ''">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <line x1="18" y1="6" x2="6" y2="18"/>
                      <line x1="6" y1="6" x2="18" y2="18"/>
                    </svg>
                  </button>
                </div>
                <button
                  v-for="color in colorOptions"
                  :key="color"
                  class="color-btn"
                  :class="{ active: selectedColor === color }"
                  :style="{ background: color }"
                  @click="selectedColor = color"
                >
                  <span v-if="selectedColor === color" class="check-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="var(--bg-card)" stroke-width="3">
                      <polyline points="20 6 9 17 4 12"/>
                    </svg>
                  </span>
                </button>
              </div>
            </div>

            <!-- Footer Actions -->
            <div class="modal-footer">
              <button class="btn-cancel" @click="handleCancel">
                取消
              </button>
              <button class="btn-save" @click="handleSave">
                保存标签
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

interface Props {
  isVisible: boolean
  isEditMode?: boolean
  initialName?: string
  initialColor?: string
}

interface Emits {
  (e: 'cancel'): void
  (e: 'save', data: { name: string; color: string }): void
  (e: 'update:isVisible', value: boolean): void
}

const props = withDefaults(defineProps<Props>(), {
  isEditMode: false,
  initialName: '',
  initialColor: 'var(--color-primary)'
})

const emit = defineEmits<Emits>()

const tagName = ref('')
const selectedColor = ref('var(--color-primary)')

const colorOptions = [
  'var(--color-error)',
  '#f97316',
  'var(--color-success)',
  '#8b5cf6',
  '#ec4899',
  'var(--text-secondary)'
]

watch(() => props.isVisible, (visible) => {
  if (visible) {
    tagName.value = props.initialName
    selectedColor.value = props.initialColor || 'var(--color-primary)'
  }
})

const handleCancel = () => {
  emit('update:isVisible', false)
  emit('cancel')
}

const handleSave = () => {
  if (!tagName.value.trim()) {
    return
  }
  emit('save', {
    name: tagName.value.trim(),
    color: selectedColor.value
  })
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
  backdrop-filter: blur(6px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.modal-container {
  background: var(--bg-card);
  border-radius: 48px;
  box-shadow: 0px 25px 50px -12px rgba(0, 0, 0, 0.25);
  width: 480px;
  padding: 32px;
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

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 32px;
}

.modal-title {
  font-size: 20px;
  font-weight: 600;
  color: #191c1e;
  margin: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: var(--bg-input);
}

.close-btn svg {
  width: 14px;
  height: 14px;
  color: var(--text-secondary);
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.input-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.field-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.6px;
  padding-left: 4px;
}

.input-wrapper {
  background: var(--bg-input);
  border-radius: 48px;
  padding: 16px 20px;
}

.text-input {
  width: 100%;
  background: transparent;
  border: none;
  outline: none;
  font-size: 18px;
  font-weight: 500;
  color: #191c1e;
}

.text-input::placeholder {
  color: var(--text-tertiary);
}

.color-picker {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.color-options {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 0 4px;
}

.color-column {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.color-btn {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  border: none;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.color-btn.active {
  box-shadow: 0 0 0 4px var(--bg-card), 0 0 0 6px currentColor;
}

.color-btn:hover {
  transform: scale(1.1);
}

.check-icon {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.check-icon svg {
  width: 16px;
  height: 16px;
}

.remove-color-btn {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  border: 1px solid var(--border-color);
  background: var(--scrollbar-thumb);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.remove-color-btn:hover {
  background: #d1d5db;
}

.remove-color-btn svg {
  width: 12px;
  height: 12px;
  color: var(--text-secondary);
}

.modal-footer {
  display: flex;
  gap: 16px;
  padding-top: 16px;
}

.btn-cancel {
  flex: 1;
  padding: 16px;
  background: transparent;
  border: none;
  border-radius: 48px;
  font-size: 16px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-cancel:hover {
  background: var(--bg-input);
}

.btn-save {
  flex: 1;
  padding: 16px;
  background: var(--color-primary);
  border: none;
  border-radius: 48px;
  font-size: 16px;
  font-weight: 500;
  color: var(--bg-card);
  cursor: pointer;
  box-shadow: 0px 10px 15px -3px rgba(0, 88, 190, 0.2), 0px 4px 6px -4px rgba(0, 88, 190, 0.2);
  transition: all 0.2s ease;
}

.btn-save:hover {
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