<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue'
import dayjs from 'dayjs'

export interface TimeSegment {
  startMinutes: number
  endMinutes: number
  isRunning?: boolean
}

export interface GanttTask {
  id: number
  title: string
  taskQuadrant: number
  taskStatus: string
  tags?: string[]
  segments: TimeSegment[]
}

const props = defineProps<{
  tasks: GanttTask[]
  workDayStart: number
  workDayEnd: number
}>()

const emit = defineEmits<{
  (e: 'edit-task', taskId: number): void
}>()

const now = ref(dayjs())
let timer: ReturnType<typeof setInterval> | null = null

onMounted(() => {
  timer = setInterval(() => {
    now.value = dayjs()
  }, 60000)
})

onUnmounted(() => {
  if (timer) clearInterval(timer)
})

// 工作开始时间（分钟，从0点开始）
const workStartMinutes = computed(() => props.workDayStart * 60)
// 工作结束时间（分钟，从0点开始）
const workEndMinutes = computed(() => props.workDayEnd * 60)
// 总工作时长（分钟）
const totalWorkMinutes = computed(() => workEndMinutes.value - workStartMinutes.value)

// 生成时间刻度（每小时的整点，但只显示在工作时段内的）
const timeTicks = computed(() => {
  const startHour = Math.floor(props.workDayStart)
  const endHour = Math.ceil(props.workDayEnd)
  const ticks: { hour: number; label: string; offsetPercent: number }[] = []

  for (let h = startHour; h <= endHour; h++) {
    const hourMinutes = h * 60
    // 计算该小时刻度相对于工作开始时间的偏移百分比
    const offsetMinutes = hourMinutes - workStartMinutes.value
    const offsetPercent = (offsetMinutes / totalWorkMinutes.value) * 100
    ticks.push({
      hour: h,
      label: `${String(h).padStart(2, '0')}:00`,
      offsetPercent
    })
  }
  return ticks
})



// 任务段样式
const getSegmentStyle = (segment: TimeSegment) => {
  const offsetMinutes = segment.startMinutes - workStartMinutes.value
  const leftPercent = (offsetMinutes / totalWorkMinutes.value) * 100
  const durationMinutes = segment.endMinutes - segment.startMinutes
  const widthPercent = (durationMinutes / totalWorkMinutes.value) * 100
  return {
    left: `${Math.max(leftPercent, 0)}%`,
    width: `${Math.max(widthPercent, 1)}%`
  }
}

const getQuadrantClass = (quadrant: number): string => {
  switch (quadrant) {
    case 1: return 'quadrant-1'
    case 2: return 'quadrant-2'
    case 3: return 'quadrant-3'
    case 4: return 'quadrant-4'
    default: return 'quadrant-1'
  }
}

const formatTime = (minutes: number): string => {
  const h = Math.floor(minutes / 60)
  const m = minutes % 60
  return `${String(h).padStart(2, '0')}:${String(m).padStart(2, '0')}`
}

const formatSegmentTime = (segment: TimeSegment): string => {
  const start = formatTime(segment.startMinutes)
  const end = segment.isRunning ? now.value.format('HH:mm') : formatTime(segment.endMinutes)
  return `${start} - ${end}`
}

const getTagClass = (tag: string): string => {
  const tagLower = tag.toLowerCase()
  if (tagLower.includes('urgent')) return 'tag-urgent'
  if (tagLower.includes('core')) return 'tag-core'
  if (tagLower.includes('strategy')) return 'tag-strategy'
  if (tagLower.includes('deepwork')) return 'tag-deepwork'
  if (tagLower.includes('external')) return 'tag-external'
  if (tagLower.includes('email')) return 'tag-email'
  return 'tag-default'
}

const isArchived = (status: string): boolean => status === 'archived'
</script>

<template>
  <div class="gantt-wrapper">
    <!-- Glass Header -->
    <div class="gantt-header-glass">
      <div class="gantt-label-header">
        <span class="header-title">Tasks</span>
        <span class="header-count">{{ tasks.length }}</span>
      </div>
      <div class="gantt-time-header">
        <div
          v-for="tick in timeTicks"
          :key="tick.hour"
          class="time-tick"
          :style="{ left: `${tick.offsetPercent}%` }"
        >
          <span class="time-label">{{ tick.label }}</span>
        </div>
      </div>
    </div>

    <!-- Body with Glass Effect -->
    <div class="gantt-body">
      <!-- Vertical Grid Lines -->
      <div class="grid-lines">
        <div
          v-for="tick in timeTicks"
          :key="tick.hour"
          class="grid-line"
          :style="{ left: `${tick.offsetPercent}%` }"
        ></div>
      </div>

      <!-- Task Rows -->
      <div v-if="tasks.length > 0" class="task-rows">
        <div
          v-for="(task, rowIndex) in tasks"
          :key="task.id"
          class="task-row"
          :class="{ 'row-alt': rowIndex % 2 === 1 }"
        >
          <!-- Task Label -->
          <div class="task-label-glass" @click="emit('edit-task', task.id)">
            <div class="quadrant-indicator" :class="getQuadrantClass(task.taskQuadrant)">
              <div class="indicator-glow"></div>
            </div>
            <div class="label-content">
              <span class="label-title" :class="{ 'archived': isArchived(task.taskStatus) }">
                {{ task.title }}
              </span>
              <div v-if="task.tags && task.tags.length > 0" class="label-tags">
                <span
                  v-for="(tag, idx) in task.tags.slice(0, 2)"
                  :key="idx"
                  class="label-tag"
                  :class="getTagClass(tag)"
                >{{ tag.replace('#', '') }}</span>
              </div>
            </div>
          </div>

          <!-- Task Bar Area -->
          <div class="task-bar-area">
            <div
              v-for="(segment, segIndex) in task.segments"
              :key="segIndex"
              class="task-segment"
              :class="[
                getQuadrantClass(task.taskQuadrant),
                { 'is-running': segment.isRunning, 'is-archived': isArchived(task.taskStatus) }
              ]"
              :style="getSegmentStyle(segment)"
              @click="emit('edit-task', task.id)"
            >
              <div class="segment-glass"></div>
              <div class="segment-content">
                <span class="segment-time">{{ formatSegmentTime(segment) }}</span>
                <span v-if="segment.isRunning" class="running-dot"></span>
              </div>
              <div v-if="segment.isRunning" class="segment-shimmer"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="empty-state">
        <div class="empty-glass">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
            <line x1="16" y1="2" x2="16" y2="6"/>
            <line x1="8" y1="2" x2="8" y2="6"/>
            <line x1="3" y1="10" x2="21" y2="10"/>
          </svg>
          <p>No tasks scheduled</p>
          <span class="empty-hint">Your day is clear — time to focus</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.gantt-wrapper {
  display: flex;
  flex-direction: column;
  background: var(--glass-bg);
  backdrop-filter: blur(20px) saturate(180%);
  -webkit-backdrop-filter: blur(20px) saturate(180%);
  border-radius: 16px;
  border: 1px solid var(--glass-border);
  overflow: hidden;
  min-height: 400px;
  box-shadow:
    0 4px 24px rgba(0, 0, 0, 0.04),
    0 1px 2px rgba(0, 0, 0, 0.02),
    inset 0 1px 0 rgba(255, 255, 255, 0.6);
}

/* Glass Header */
.gantt-header-glass {
  display: flex;
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--border-light);
  flex-shrink: 0;
}

.gantt-label-header {
  flex: 0 0 180px;
  min-width: 160px;
  max-width: 220px;
  padding: 16px 20px;
  border-right: 1px solid var(--border-light);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-title {
  font-size: 11px;
  font-weight: 700;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.8px;
}

.header-count {
  font-size: 11px;
  font-weight: 700;
  color: var(--text-tertiary);
  background: var(--bg-input);
  padding: 2px 8px;
  border-radius: 10px;
  min-width: 20px;
  text-align: center;
}

.gantt-time-header {
  flex: 1;
  position: relative;
  height: 52px;
  overflow: hidden;
}

.time-tick {
  position: absolute;
  top: 0;
  bottom: 0;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  justify-content: center;
}

.time-tick:first-child {
  transform: translateX(0);
}

.time-tick:last-child {
  transform: translateX(-100%);
}

.time-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 0.3px;
  white-space: nowrap;
}

/* Gantt Body */
.gantt-body {
  flex: 1;
  position: relative;
  overflow-y: auto;
  overflow-x: hidden;
  min-height: 350px;
}

/* Grid Lines */
.grid-lines {
  position: absolute;
  top: 0;
  left: 180px;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 1;
}

.grid-line {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 1px;
  background: linear-gradient(to bottom,
    var(--border-light) 0%,
    var(--border-light) 70%,
    transparent 100%
  );
}

/* Task Rows */
.task-rows {
  position: relative;
  z-index: 2;
}

.task-row {
  display: flex;
  min-height: 76px;
  border-bottom: 1px solid var(--border-light);
  transition: background-color 0.25s ease;
}

.task-row:hover {
  background: rgba(0, 113, 227, 0.03);
}

.task-row.row-alt {
  background: rgba(0, 0, 0, 0.01);
}

.task-row.row-alt:hover {
  background: rgba(0, 113, 227, 0.04);
}

/* Task Label - Glass Style */
.task-label-glass {
  flex: 0 0 180px;
  min-width: 160px;
  max-width: 220px;
  flex-shrink: 0;
  padding: 14px 18px;
  display: flex;
  align-items: center;
  gap: 12px;
  border-right: 1px solid var(--border-light);
  cursor: pointer;
  transition: all 0.2s ease;
}

.task-label-glass:hover {
  background: rgba(0, 113, 227, 0.04);
}

.quadrant-indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
  position: relative;
}

.indicator-glow {
  position: absolute;
  inset: -4px;
  border-radius: 50%;
  opacity: 0.3;
  filter: blur(4px);
}

.quadrant-indicator.quadrant-1 { background: var(--color-primary); }
.quadrant-indicator.quadrant-1 .indicator-glow { background: var(--color-primary); }
.quadrant-indicator.quadrant-2 { background: var(--color-success); }
.quadrant-indicator.quadrant-2 .indicator-glow { background: var(--color-success); }
.quadrant-indicator.quadrant-3 { background: var(--color-warning); }
.quadrant-indicator.quadrant-3 .indicator-glow { background: var(--color-warning); }
.quadrant-indicator.quadrant-4 { background: #af52de; }
.quadrant-indicator.quadrant-4 .indicator-glow { background: #af52de; }

.label-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
  flex: 1;
}

.label-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  letter-spacing: -0.1px;
}

.label-title.archived {
  color: var(--text-tertiary);
  text-decoration: line-through;
}

.label-tags {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}

.label-tag {
  font-size: 9px;
  font-weight: 700;
  padding: 1px 5px;
  border-radius: 4px;
  text-transform: uppercase;
  letter-spacing: 0.4px;
}

.label-tag.tag-urgent { background: rgba(255, 59, 48, 0.12); color: #e53e3e; }
.label-tag.tag-core { background: rgba(0, 113, 227, 0.12); color: #2563eb; }
.label-tag.tag-strategy { background: rgba(175, 82, 222, 0.12); color: #7c3aed; }
.label-tag.tag-deepwork { background: rgba(48, 209, 88, 0.12); color: #059669; }
.label-tag.tag-external { background: rgba(255, 149, 0, 0.12); color: #d97706; }
.label-tag.tag-email { background: rgba(120, 120, 128, 0.12); color: #6b7280; }

/* Task Bar Area */
.task-bar-area {
  flex: 1;
  position: relative;
  padding: 14px 0;
  min-width: 0;
}

/* Task Segment - Glass Bar */
.task-segment {
  position: absolute;
  top: 14px;
  height: 48px;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  min-width: 60px;
  box-sizing: border-box;
  overflow: hidden;
  border: 1px solid transparent;
}

.task-segment:hover {
  transform: translateY(-2px) scale(1.01);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  z-index: 10;
}

.segment-glass {
  position: absolute;
  inset: 0;
  border-radius: 10px;
  backdrop-filter: blur(12px) saturate(150%);
  -webkit-backdrop-filter: blur(12px) saturate(150%);
}

.segment-content {
  position: relative;
  z-index: 2;
  display: flex;
  align-items: center;
  gap: 8px;
  height: 100%;
  padding: 0 12px;
}

.segment-time {
  font-size: 10px;
  font-weight: 700;
  color: var(--text-secondary);
  letter-spacing: 0.2px;
  white-space: nowrap;
}

.running-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--color-success);
  box-shadow: 0 0 0 3px rgba(48, 209, 88, 0.3);
  animation: running-blink 1.5s ease-in-out infinite;
}

@keyframes running-blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

/* Segment Shimmer for Running Tasks */
.segment-shimmer {
  position: absolute;
  top: 0;
  left: -100%;
  width: 50%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  animation: shimmer 2.5s ease-in-out infinite;
  z-index: 3;
  pointer-events: none;
}

@keyframes shimmer {
  0% { left: -100%; }
  100% { left: 200%; }
}

/* Quadrant Colors - Glass Style */
.task-segment.quadrant-1 {
  background: linear-gradient(135deg, rgba(0, 113, 227, 0.15) 0%, rgba(0, 113, 227, 0.04) 100%);
  border-color: rgba(0, 113, 227, 0.18);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.4),
    0 2px 8px rgba(0, 113, 227, 0.08),
    0 0 0 1px rgba(0, 113, 227, 0.05);
}

.task-segment.quadrant-2 {
  background: linear-gradient(135deg, rgba(48, 209, 88, 0.15) 0%, rgba(48, 209, 88, 0.04) 100%);
  border-color: rgba(48, 209, 88, 0.18);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.4),
    0 2px 8px rgba(48, 209, 88, 0.08),
    0 0 0 1px rgba(48, 209, 88, 0.05);
}

.task-segment.quadrant-3 {
  background: linear-gradient(135deg, rgba(255, 149, 0, 0.15) 0%, rgba(255, 149, 0, 0.04) 100%);
  border-color: rgba(255, 149, 0, 0.18);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.4),
    0 2px 8px rgba(255, 149, 0, 0.08),
    0 0 0 1px rgba(255, 149, 0, 0.05);
}

.task-segment.quadrant-4 {
  background: linear-gradient(135deg, rgba(175, 82, 222, 0.15) 0%, rgba(175, 82, 222, 0.04) 100%);
  border-color: rgba(175, 82, 222, 0.18);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.4),
    0 2px 8px rgba(175, 82, 222, 0.08),
    0 0 0 1px rgba(175, 82, 222, 0.05);
}

/* Archived State */
.task-segment.is-archived {
  opacity: 0.5;
  background: linear-gradient(135deg, rgba(120, 120, 128, 0.08) 0%, rgba(120, 120, 128, 0.03) 100%);
  border-color: rgba(120, 120, 128, 0.12);
}

/* Empty State */
.empty-state {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.empty-glass {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 40px 48px;
  background: var(--glass-bg);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border-radius: 20px;
  border: 1px solid var(--glass-border);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.06);
  color: var(--text-tertiary);
}

.empty-glass svg {
  width: 48px;
  height: 48px;
  opacity: 0.5;
}

.empty-glass p {
  margin: 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-secondary);
}

.empty-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  font-weight: 500;
}

/* Scrollbar Styling */
.gantt-body::-webkit-scrollbar {
  width: 6px;
}

.gantt-body::-webkit-scrollbar-track {
  background: transparent;
}

.gantt-body::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 3px;
}

.gantt-body::-webkit-scrollbar-thumb:hover {
  background: var(--scrollbar-thumb-hover);
}

/* Dark Mode Adjustments */
[data-theme="dark"] .gantt-wrapper {
  box-shadow:
    0 4px 24px rgba(0, 0, 0, 0.2),
    0 1px 2px rgba(0, 0, 0, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.05);
}

[data-theme="dark"] .gantt-header-glass {
  background: rgba(28, 28, 30, 0.6);
}

[data-theme="dark"] .segment-shimmer {
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.08), transparent);
}

[data-theme="dark"] .task-segment.quadrant-1 {
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.05), 0 1px 3px rgba(0, 0, 0, 0.2);
}

[data-theme="dark"] .task-segment.quadrant-2 {
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.05), 0 1px 3px rgba(0, 0, 0, 0.2);
}

[data-theme="dark"] .task-segment.quadrant-3 {
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.05), 0 1px 3px rgba(0, 0, 0, 0.2);
}

[data-theme="dark"] .task-segment.quadrant-4 {
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.05), 0 1px 3px rgba(0, 0, 0, 0.2);
}

/* Responsive */
@media (max-width: 768px) {
  .gantt-label-header,
  .task-label-glass {
    flex: 0 0 140px;
    min-width: 120px;
    padding: 12px 14px;
  }

  .grid-lines {
    left: 140px;
  }

  .current-time-indicator {
    left: calc(140px + var(--offset, 0%)) !important;
  }
}
</style>
