<template>
  <div class="holiday-settings-content">
    <!-- 主内容区 -->
    <div class="main-container">
      <!-- 左侧日历区域 -->
      <div class="calendar-section">
        <div class="section-header">
          <div class="title-area">
            <h2 class="section-title">工作节奏设定</h2>
            <p class="section-desc">选择日期来进行休假，补班和正常工作日的转换</p>
          </div>
          <div class="month-nav">
            <button class="nav-btn" @click="prevMonth">
              <el-icon><ArrowLeft /></el-icon>
            </button>
            <div class="current-month">
              {{ currentMonthName }}<br/>{{ currentYear }}
            </div>
            <button class="nav-btn" @click="nextMonth">
              <el-icon><ArrowRight /></el-icon>
            </button>
          </div>
        </div>

        <!-- 提示信息 -->
        <div class="info-tip">
          <el-icon><InfoFilled /></el-icon>
          <span>点击日历中的日期切换状态：工作日 → 节假日/补班 → 恢复默认</span>
        </div>

        <!-- 日历网格 -->
        <div class="calendar-grid">
          <!-- 星期标题 -->
          <div class="weekday-header" v-for="day in weekdays" :key="day">
            {{ day }}
          </div>
          
          <!-- 日期单元格 -->
          <div 
            v-for="(date, index) in calendarDays" 
            :key="index"
            :class="[
              'calendar-cell',
              {
                'other-month': !date.isCurrentMonth,
                'holiday': date.dateType === 'holiday',
                'makeup': date.dateType === 'makeup',
                'weekend': date.dateType === 'weekend',
                'workday': date.dateType === 'workday',
                'today': date.isToday,
                'loading': loadingDates.has(date.date),
                'has-custom-hours': date.hasCustomHours
              }
            ]"
            @click="handleDateClick(date)"
          >
            <div class="date-content">
              <span class="date-number">{{ date.day }}</span>
              <el-icon v-if="date.dateType === 'holiday'" class="type-icon holiday-icon"><Sunny /></el-icon>
              <el-icon v-if="date.dateType === 'makeup'" class="type-icon makeup-icon"><Briefcase /></el-icon>
              <el-icon v-if="date.dateType === 'weekend'" class="type-icon weekend-icon"><Moon /></el-icon>
            </div>
            <div v-if="date.dateType === 'holiday'" class="date-label">休假</div>
            <div v-if="date.dateType === 'makeup'" class="date-label">补班</div>
            <div v-if="date.dateType === 'weekend'" class="date-label">周末</div>
            <div v-if="date.hasCustomHours" class="hours-label">{{ date.workHours }}h</div>
            <div v-if="date.name" class="date-name">{{ date.name }}</div>
          </div>
        </div>

        <!-- 图例和操作按钮 -->
        <div class="calendar-footer">
          <div class="legend">
            <div class="legend-item">
              <span class="legend-dot workday-dot"></span>
              <span>工作日</span>
            </div>
            <div class="legend-item">
              <span class="legend-dot holiday-dot"></span>
              <span>休假</span>
            </div>
            <div class="legend-item">
              <span class="legend-dot makeup-dot"></span>
              <span>补班</span>
            </div>
            <div class="legend-item">
              <span class="legend-dot weekend-dot"></span>
              <span>周末</span>
            </div>
            <div class="legend-item">
              <span class="legend-dot custom-hours-dot"></span>
              <span>自定义工时</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧批量操作区域 -->
      <div class="bulk-actions-section">
        <div class="bulk-header">
          <div class="bulk-icon">
            <el-icon><Lightning /></el-icon>
          </div>
          <h3 class="bulk-title">批量操作</h3>
        </div>

        <div class="bulk-content">
          <!-- 日期范围选择 -->
          <div class="form-group">
            <label class="form-label">选择日期范围</label>
            <div class="date-range">
              <div class="date-input-wrapper">
                <el-icon class="date-icon"><Calendar /></el-icon>
                <input 
                  type="date" 
                  v-model="bulkStartDate" 
                  class="date-input"
                  @change="onDateRangeChange"
                />
              </div>
              <span class="date-separator">至</span>
              <div class="date-input-wrapper">
                <el-icon class="date-icon"><Calendar /></el-icon>
                <input 
                  type="date" 
                  v-model="bulkEndDate" 
                  class="date-input"
                  @change="onDateRangeChange"
                />
              </div>
            </div>
            <p v-if="dateError" class="error-text">{{ dateError }}</p>
          </div>

          <!-- 类型选择 -->
          <div class="form-group">
            <label class="form-label">设置为</label>
            <div class="type-options">
              <button 
                :class="['type-btn', 'holiday-btn', { active: bulkType === 'holiday', disabled: !canSetHoliday }]"
                @click="canSetHoliday && (bulkType = 'holiday')"
                :disabled="!canSetHoliday"
              >
                <el-icon><Sunny /></el-icon>
                <span>休假</span>
              </button>
              <button 
                :class="['type-btn', 'makeup-btn', { active: bulkType === 'makeup', disabled: !canSetMakeup }]"
                @click="canSetMakeup && (bulkType = 'makeup')"
                :disabled="!canSetMakeup"
              >
                <el-icon><Briefcase /></el-icon>
                <span>补班</span>
              </button>
            </div>
            <p v-if="typeHint" class="hint-text">{{ typeHint }}</p>
          </div>

          <!-- 节假日名称输入 -->
          <div class="form-group" v-if="bulkType === 'holiday'">
            <label class="form-label">节假日名称（可选）</label>
            <input 
              type="text" 
              v-model="bulkHolidayName" 
              placeholder="例如：国庆节"
              class="text-input"
            />
          </div>

          <!-- 预览信息 -->
          <div class="preview-section" v-if="batchPreview">
            <div class="preview-header">
              <el-icon><View /></el-icon>
              <span>操作预览</span>
            </div>
            <div class="preview-content">
              <div class="preview-item">
                <span class="preview-label">总天数：</span>
                <span class="preview-value">{{ batchPreview.total_days }} 天</span>
              </div>
              <div class="preview-item" v-if="batchPreview.holidays > 0">
                <span class="preview-label">将设为休假：</span>
                <span class="preview-value highlight-holiday">{{ batchPreview.holidays }} 天</span>
              </div>
              <div class="preview-item" v-if="batchPreview.makeups > 0">
                <span class="preview-label">将设为补班：</span>
                <span class="preview-value highlight-makeup">{{ batchPreview.makeups }} 天</span>
              </div>
              <div class="preview-item" v-if="batchPreview.invalid_dates.length > 0">
                <span class="preview-label">无效日期：</span>
                <span class="preview-value highlight-error">{{ batchPreview.invalid_dates.length }} 天</span>
              </div>
            </div>
          </div>

          <!-- 应用按钮 -->
          <button 
            class="apply-btn" 
            @click="applyBulkChanges"
            :disabled="!canApplyBulk"
          >
            <el-icon v-if="isApplying" class="loading-icon"><Loading /></el-icon>
            <span>{{ isApplying ? '应用中...' : '应用更改' }}</span>
          </button>
        </div>
      </div>
    </div>

    <!-- 默认工作时长配置区域 -->
    <div class="default-work-hours-section">
      <div class="section-header">
        <div class="title-area">
          <h3 class="section-title">
            <el-icon><Clock /></el-icon>
            默认工作时长
          </h3>
          <p class="section-desc">设置标准的工作时间段，将作为所有日期的默认配置</p>
        </div>
        <button class="edit-btn" @click="showDefaultWorkHoursEdit = true">
          <el-icon><Edit /></el-icon>
          修改全局默认
        </button>
      </div>

      <div class="work-hours-display">
        <div class="session-card">
          <div class="session-label">上午时段</div>
          <div class="session-time">
            <span class="time">{{ defaultWorkHours.morning_start_time }}</span>
            <span class="separator">—</span>
            <span class="time">{{ defaultWorkHours.morning_end_time }}</span>
          </div>
        </div>
        <div class="session-card">
          <div class="session-label">下午时段</div>
          <div class="session-time">
            <span class="time">{{ defaultWorkHours.afternoon_start_time }}</span>
            <span class="separator">—</span>
            <span class="time">{{ defaultWorkHours.afternoon_end_time }}</span>
          </div>
        </div>
        <div class="total-card">
          <div class="total-label">总工作时长</div>
          <div class="total-value">{{ formatHours(defaultWorkHours.total_work_minutes) }}h</div>
        </div>
      </div>
    </div>

    <!-- 确认对话框 -->
    <el-dialog
      v-model="confirmDialogVisible"
      title="确认更改"
      width="420px"
      :close-on-click-modal="false"
      class="apple-style-dialog confirm-dialog"
    >
      <div class="confirm-content-apple">
        <div class="confirm-icon">
          <el-icon><Warning /></el-icon>
        </div>
        <h3 class="confirm-title">即将应用更改</h3>
        <p class="confirm-desc">请确认以下操作信息</p>
        
        <div class="confirm-details">
          <div class="confirm-item">
            <span class="item-label">日期范围</span>
            <span class="item-value">{{ formatDate(bulkStartDate) }} 至 {{ formatDate(bulkEndDate) }}</span>
          </div>
          <div class="confirm-item">
            <span class="item-label">类型</span>
            <span :class="['item-value', 'type-badge', bulkType]">
              {{ bulkType === 'holiday' ? '休假' : '补班' }}
            </span>
          </div>
          <div class="confirm-item" v-if="bulkHolidayName">
            <span class="item-label">名称</span>
            <span class="item-value">{{ bulkHolidayName }}</span>
          </div>
          <div class="confirm-item">
            <span class="item-label">影响天数</span>
            <span class="item-value highlight">{{ batchPreview?.preview?.length || 0 }} 天</span>
          </div>
        </div>

        <div v-if="batchPreview && batchPreview.invalid_dates && batchPreview.invalid_dates.length > 0" class="confirm-warning-apple">
          <el-icon><InfoFilled /></el-icon>
          <span>有 {{ batchPreview.invalid_dates.length }} 个日期不符合规则将被跳过</span>
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer apple-footer">
          <button class="apple-btn secondary" @click="confirmDialogVisible = false">取消</button>
          <button class="apple-btn primary" @click="confirmApplyChanges" :disabled="isApplying">
            {{ isApplying ? '应用中...' : '确认' }}
          </button>
        </div>
      </template>
    </el-dialog>

    <!-- 日期详情对话框 -->
    <el-dialog
      v-model="dateDetailVisible"
      title="日期详情"
      width="420px"
      :close-on-click-modal="true"
      class="apple-style-dialog date-detail-dialog"
    >
      <div class="date-detail-content" v-if="selectedDateDetail">
        <div class="detail-header">
          <div class="detail-date">{{ selectedDateDetail.date }}</div>
          <div class="detail-weekday">{{ selectedDateDetail.day_of_week_name }}</div>
        </div>
        <div class="detail-status">
          <span class="status-label">当前状态</span>
          <span :class="['status-value', selectedDateDetail.date_type]">
            {{ getDateTypeLabel(selectedDateDetail.date_type) }}
          </span>
        </div>
        
        <!-- 工作时长配置 -->
        <div class="work-hours-section" v-if="selectedDateWorkHours">
          <div class="work-hours-header">
            <span class="work-hours-title">工作时长配置</span>
            <span v-if="selectedDateWorkHours.is_custom" class="custom-badge">自定义</span>
            <span v-else class="default-badge">默认</span>
          </div>
          <div class="work-hours-summary">
            <div class="work-hours-total">
              <span class="total-hours">{{ selectedDateWorkHours.total_hours }}h</span>
              <span class="total-label">总工作时长</span>
            </div>
          </div>
          <div class="work-sessions">
            <div class="work-session">
              <span class="session-name">上午</span>
              <span class="session-time">{{ selectedDateWorkHours.morning_session.start_time }} - {{ selectedDateWorkHours.morning_session.end_time }}</span>
              <span class="session-duration">{{ formatMinutes(selectedDateWorkHours.morning_session.duration_minutes) }}</span>
            </div>
            <div class="work-session">
              <span class="session-name">下午</span>
              <span class="session-time">{{ selectedDateWorkHours.afternoon_session.start_time }} - {{ selectedDateWorkHours.afternoon_session.end_time }}</span>
              <span class="session-duration">{{ formatMinutes(selectedDateWorkHours.afternoon_session.duration_minutes) }}</span>
            </div>
          </div>
        </div>

        <div class="detail-actions">
          <p class="action-hint">点击切换状态</p>
          <div class="action-buttons">
            <button 
              v-if="selectedDateDetail.date_type !== 'holiday' && !selectedDateDetail.is_weekend"
              class="action-btn holiday"
              @click="setDateType('holiday')"
            >
              <el-icon><Sunny /></el-icon>
              <span>设为休假</span>
            </button>
            <button 
              v-if="selectedDateDetail.date_type !== 'makeup' && selectedDateDetail.is_weekend"
              class="action-btn makeup"
              @click="setDateType('makeup')"
            >
              <el-icon><Briefcase /></el-icon>
              <span>设为补班</span>
            </button>
            <button 
              v-if="selectedDateDetail.date_type === 'holiday' || selectedDateDetail.date_type === 'makeup'"
              class="action-btn reset"
              @click="resetDateType()"
            >
              <el-icon><RefreshLeft /></el-icon>
              <span>恢复默认</span>
            </button>
            <button 
              class="action-btn work-hours"
              @click="openWorkHoursEdit()"
            >
              <el-icon><Clock /></el-icon>
              <span>调整工时</span>
            </button>
          </div>
        </div>
      </div>
    </el-dialog>

    <!-- 默认工作时长编辑对话框 -->
    <el-dialog
      v-model="showDefaultWorkHoursEdit"
      title="修改默认工作时长"
      width="460px"
      :close-on-click-modal="false"
      class="apple-style-dialog work-hours-dialog"
    >
      <div class="work-hours-edit-content">
        <div class="edit-section">
          <div class="section-header-with-icon">
            <div class="icon-wrapper morning">
              <el-icon><Sunny /></el-icon>
            </div>
            <h4>上午工作时段</h4>
          </div>
          <div class="time-inputs">
            <div class="time-field">
              <label>开始时间</label>
              <div class="time-input-wrapper">
                <input 
                  type="time" 
                  v-model="editingDefaultWorkHours.morning_start_time"
                  class="time-input"
                />
              </div>
            </div>
            <div class="time-field">
              <label>结束时间</label>
              <div class="time-input-wrapper">
                <input 
                  type="time" 
                  v-model="editingDefaultWorkHours.morning_end_time"
                  class="time-input"
                />
              </div>
            </div>
          </div>
        </div>
        
        <div class="edit-section">
          <div class="section-header-with-icon">
            <div class="icon-wrapper afternoon">
              <el-icon><Moon /></el-icon>
            </div>
            <h4>下午工作时段</h4>
          </div>
          <div class="time-inputs">
            <div class="time-field">
              <label>开始时间</label>
              <div class="time-input-wrapper">
                <input 
                  type="time" 
                  v-model="editingDefaultWorkHours.afternoon_start_time"
                  class="time-input"
                />
              </div>
            </div>
            <div class="time-field">
              <label>结束时间</label>
              <div class="time-input-wrapper">
                <input 
                  type="time" 
                  v-model="editingDefaultWorkHours.afternoon_end_time"
                  class="time-input"
                />
              </div>
            </div>
          </div>
        </div>

        <div class="validation-result" v-if="defaultWorkHoursValidation">
          <div v-if="!defaultWorkHoursValidation.is_valid" class="validation-error">
            <el-icon><Warning /></el-icon>
            <span>{{ defaultWorkHoursValidation.error_message }}</span>
          </div>
          <div v-else class="validation-success">
            <div class="success-content">
              <el-icon><CircleCheck /></el-icon>
              <div class="success-info">
                <span class="success-label">总工作时长</span>
                <span class="success-value">{{ formatMinutes(defaultWorkHoursValidation.total_minutes) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer apple-footer">
          <button class="apple-btn secondary" @click="showDefaultWorkHoursEdit = false">取消</button>
          <button 
            class="apple-btn primary" 
            @click="saveDefaultWorkHours" 
            :disabled="isSavingDefault || !defaultWorkHoursValidation?.is_valid"
          >
            {{ isSavingDefault ? '保存中...' : '保存' }}
          </button>
        </div>
      </template>
    </el-dialog>

    <!-- 单日工作时长编辑对话框 -->
    <el-dialog
      v-model="showDateWorkHoursEdit"
      title="调整工作时长"
      width="460px"
      :close-on-click-modal="false"
      class="apple-style-dialog work-hours-dialog"
    >
      <div class="work-hours-edit-content" v-if="editingDateWorkHours">
        <div class="edit-header-apple">
          <div class="date-info">
            <span class="edit-date">{{ editingDateWorkHours.date }}</span>
            <span v-if="editingDateWorkHours.is_custom" class="custom-indicator">自定义</span>
          </div>
          <button 
            class="reset-to-default-btn-apple"
            @click="resetDateWorkHours"
            v-if="editingDateWorkHours.is_custom"
          >
            <el-icon><RefreshLeft /></el-icon>
            恢复默认
          </button>
        </div>

        <div class="edit-section">
          <div class="section-header-with-icon">
            <div class="icon-wrapper morning">
              <el-icon><Sunny /></el-icon>
            </div>
            <h4>上午工作时段</h4>
          </div>
          <div class="time-inputs">
            <div class="time-field">
              <label>开始时间</label>
              <div class="time-input-wrapper">
                <input 
                  type="time" 
                  v-model="editingDateWorkHours.morning_session.start_time"
                  class="time-input"
                />
              </div>
            </div>
            <div class="time-field">
              <label>结束时间</label>
              <div class="time-input-wrapper">
                <input 
                  type="time" 
                  v-model="editingDateWorkHours.morning_session.end_time"
                  class="time-input"
                />
              </div>
            </div>
          </div>
        </div>
        
        <div class="edit-section">
          <div class="section-header-with-icon">
            <div class="icon-wrapper afternoon">
              <el-icon><Moon /></el-icon>
            </div>
            <h4>下午工作时段</h4>
          </div>
          <div class="time-inputs">
            <div class="time-field">
              <label>开始时间</label>
              <div class="time-input-wrapper">
                <input 
                  type="time" 
                  v-model="editingDateWorkHours.afternoon_session.start_time"
                  class="time-input"
                />
              </div>
            </div>
            <div class="time-field">
              <label>结束时间</label>
              <div class="time-input-wrapper">
                <input 
                  type="time" 
                  v-model="editingDateWorkHours.afternoon_session.end_time"
                  class="time-input"
                />
              </div>
            </div>
          </div>
        </div>

        <div class="edit-section">
          <div class="section-header-with-icon">
            <div class="icon-wrapper note">
              <el-icon><Edit /></el-icon>
            </div>
            <h4>备注（可选）</h4>
          </div>
          <div class="text-input-wrapper">
            <input 
              type="text" 
              v-model="editingDateWorkHours.description"
              placeholder="例如：提前下班、加班等"
              class="text-input-apple"
            />
          </div>
        </div>

        <div class="validation-result" v-if="dateWorkHoursValidation">
          <div v-if="!dateWorkHoursValidation.is_valid" class="validation-error">
            <el-icon><Warning /></el-icon>
            <span>{{ dateWorkHoursValidation.error_message }}</span>
          </div>
          <div v-else class="validation-success">
            <div class="success-content">
              <el-icon><CircleCheck /></el-icon>
              <div class="success-info">
                <span class="success-label">总工作时长</span>
                <span class="success-value">{{ formatMinutes(dateWorkHoursValidation.total_minutes) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer apple-footer">
          <button class="apple-btn secondary" @click="showDateWorkHoursEdit = false">取消</button>
          <button 
            class="apple-btn primary" 
            @click="saveDateWorkHours" 
            :disabled="isSavingDateWorkHours || !dateWorkHoursValidation?.is_valid"
          >
            {{ isSavingDateWorkHours ? '保存中...' : '保存' }}
          </button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import {
  ArrowLeft,
  ArrowRight,
  InfoFilled,
  Sunny,
  Briefcase,
  Lightning,
  Calendar,
  Moon,
  View,
  Warning,
  Loading,
  RefreshLeft,
  Clock,
  Edit,
  CircleCheck
} from '@element-plus/icons-vue'
import dayjs from 'dayjs'

// ==================== 类型定义 ====================

interface DateDetail {
  date: string
  year: number
  month: number
  day: number
  day_of_week: number
  day_of_week_name: string
  is_weekend: boolean
  date_type: 'workday' | 'holiday' | 'makeup' | 'weekend'
  name?: string
  description?: string
}

interface BatchPreviewResult {
  total_days: number
  workdays: number
  weekends: number
  holidays: number
  makeups: number
  invalid_dates: Array<{
    date: string
    is_valid: boolean
    reason?: string
  }>
  preview: DateDetail[]
}

interface CalendarDay {
  date: string
  year: number
  month: number
  day: number
  dayOfWeek: number
  dayOfWeekName: string
  isCurrentMonth: boolean
  isToday: boolean
  isWeekend: boolean
  dateType: 'workday' | 'holiday' | 'makeup' | 'weekend'
  name?: string
  description?: string
  hasCustomHours?: boolean
  workHours?: number
}

interface WorkSession {
  start_time: string
  end_time: string
  duration_minutes: number
}

interface WorkHoursDetail {
  date: string
  morning_session: WorkSession
  afternoon_session: WorkSession
  total_hours: number
  total_minutes: number
  is_custom: boolean
  description?: string
}

interface DefaultWorkHours {
  id: number
  morning_start_time: string
  morning_end_time: string
  afternoon_start_time: string
  afternoon_end_time: string
  total_work_minutes: number
}

interface WorkHoursValidationResult {
  is_valid: boolean
  error_message?: string
  total_minutes: number
}

// ==================== 响应式状态 ====================

const currentDate = ref(dayjs())
const calendarDays = ref<CalendarDay[]>([])
const loadingDates = ref<Set<string>>(new Set())
const isLoading = ref(false)

// 批量操作相关
const bulkStartDate = ref('')
const bulkEndDate = ref('')
const bulkType = ref<'holiday' | 'makeup' | null>(null)
const bulkHolidayName = ref('')
const dateError = ref('')
const batchPreview = ref<BatchPreviewResult | null>(null)
const confirmDialogVisible = ref(false)
const isApplying = ref(false)

// 日期详情对话框
const dateDetailVisible = ref(false)
const selectedDateDetail = ref<DateDetail | null>(null)
const selectedDateWorkHours = ref<WorkHoursDetail | null>(null)

// 默认工作时长
const defaultWorkHours = ref<DefaultWorkHours>({
  id: 1,
  morning_start_time: '08:30',
  morning_end_time: '12:00',
  afternoon_start_time: '13:00',
  afternoon_end_time: '17:30',
  total_work_minutes: 480
})
const showDefaultWorkHoursEdit = ref(false)
const editingDefaultWorkHours = ref({
  morning_start_time: '08:30',
  morning_end_time: '12:00',
  afternoon_start_time: '13:00',
  afternoon_end_time: '17:30'
})
const isSavingDefault = ref(false)

// 单日工作时长编辑
const showDateWorkHoursEdit = ref(false)
const editingDateWorkHours = ref<WorkHoursDetail | null>(null)
const isSavingDateWorkHours = ref(false)

// 月份工作时长配置缓存
const monthWorkHoursMap = ref<Map<string, WorkHoursDetail>>(new Map())

// ==================== 计算属性 ====================

const currentMonthName = computed(() => currentDate.value.format('M月'))
const currentYear = computed(() => currentDate.value.format('YYYY'))
const weekdays = ['周一', '周二', '周三', '周四', '周五', '周六', '周日']

const canApplyBulk = computed(() => {
  return bulkStartDate.value && 
         bulkEndDate.value && 
         bulkType.value && 
         !dateError.value &&
         batchPreview.value &&
         batchPreview.value.preview.length > 0
})

const canSetHoliday = computed(() => {
  if (!batchPreview.value) return true
  return batchPreview.value.workdays > 0
})

const canSetMakeup = computed(() => {
  if (!batchPreview.value) return true
  return batchPreview.value.weekends > 0
})

const typeHint = computed(() => {
  if (!batchPreview.value) return ''
  if (bulkType.value === 'holiday' && batchPreview.value.workdays === 0) {
    return '所选日期范围内没有工作日，无法设置休假'
  }
  if (bulkType.value === 'makeup' && batchPreview.value.weekends === 0) {
    return '所选日期范围内没有周末，无法设置补班'
  }
  return ''
})

const defaultWorkHoursValidation = computed<WorkHoursValidationResult | null>(() => {
  return validateWorkHours(
    editingDefaultWorkHours.value.morning_start_time,
    editingDefaultWorkHours.value.morning_end_time,
    editingDefaultWorkHours.value.afternoon_start_time,
    editingDefaultWorkHours.value.afternoon_end_time
  )
})

const dateWorkHoursValidation = computed<WorkHoursValidationResult | null>(() => {
  if (!editingDateWorkHours.value) return null
  return validateWorkHours(
    editingDateWorkHours.value.morning_session.start_time,
    editingDateWorkHours.value.morning_session.end_time,
    editingDateWorkHours.value.afternoon_session.start_time,
    editingDateWorkHours.value.afternoon_session.end_time
  )
})

// ==================== 方法 ====================

// 获取月份日期详情
async function fetchMonthDateDetails() {
  isLoading.value = true
  try {
    const year = currentDate.value.year()
    const month = currentDate.value.month() + 1
    
    const [details, workHoursConfig] = await Promise.all([
      invoke<DateDetail[]>('get_month_date_details', { year, month }),
      invoke<{ configs: WorkHoursDetail[] }>('get_month_work_hours', { year, month })
    ])
    
    // 更新工作时长配置缓存
    monthWorkHoursMap.value.clear()
    workHoursConfig.configs.forEach(config => {
      monthWorkHoursMap.value.set(config.date, config)
    })
    
    generateCalendarDays(details)
  } catch (error) {
    console.error('获取日期详情失败:', error)
    ElMessage.error('获取日历数据失败')
  } finally {
    isLoading.value = false
  }
}

// 获取默认工作时长
async function fetchDefaultWorkHours() {
  try {
    const result = await invoke<DefaultWorkHours>('get_default_work_hours')
    defaultWorkHours.value = result
  } catch (error) {
    console.error('获取默认工作时长失败:', error)
  }
}

// 生成日历天数
function generateCalendarDays(monthDetails: DateDetail[]) {
  const year = currentDate.value.year()
  const month = currentDate.value.month()
  
  const firstDay = dayjs(new Date(year, month, 1))
  const lastDay = dayjs(new Date(year, month + 1, 0))
  
  const days: CalendarDay[] = []
  const today = dayjs().format('YYYY-MM-DD')
  
  // 上个月的日期
  const firstDayOfWeek = firstDay.day() || 7
  for (let i = firstDayOfWeek - 1; i > 0; i--) {
    const date = firstDay.subtract(i, 'day')
    days.push(createCalendarDay(date, false, today, monthDetails))
  }
  
  // 当前月的日期
  for (let i = 1; i <= lastDay.date(); i++) {
    const date = dayjs(new Date(year, month, i))
    days.push(createCalendarDay(date, true, today, monthDetails))
  }
  
  // 下个月的日期
  const remainingDays = 42 - days.length
  for (let i = 1; i <= remainingDays; i++) {
    const date = lastDay.add(i, 'day')
    days.push(createCalendarDay(date, false, today, monthDetails))
  }
  
  calendarDays.value = days
}

// 创建日历天对象
function createCalendarDay(
  date: dayjs.Dayjs, 
  isCurrentMonth: boolean, 
  today: string,
  monthDetails: DateDetail[]
): CalendarDay {
  const dateStr = date.format('YYYY-MM-DD')
  const detail = monthDetails.find(d => d.date === dateStr)
  const dayOfWeek = (date.day() || 7) - 1
  
  // 获取工作时长信息
  const workHoursConfig = monthWorkHoursMap.value.get(dateStr)
  const hasCustomHours = workHoursConfig?.is_custom || false
  const workHours = workHoursConfig?.total_hours
  
  return {
    date: dateStr,
    year: date.year(),
    month: date.month() + 1,
    day: date.date(),
    dayOfWeek,
    dayOfWeekName: weekdays[dayOfWeek],
    isCurrentMonth,
    isToday: dateStr === today,
    isWeekend: dayOfWeek >= 5,
    dateType: detail?.date_type || (dayOfWeek >= 5 ? 'weekend' : 'workday'),
    name: detail?.name,
    description: detail?.description,
    hasCustomHours,
    workHours
  }
}

// 处理日期点击
async function handleDateClick(date: CalendarDay) {
  if (!date.isCurrentMonth || loadingDates.value.has(date.date)) return
  
  // 获取日期详情
  try {
    const [detail, workHours] = await Promise.all([
      invoke<DateDetail>('get_holiday_config_by_date', { date: date.date }),
      invoke<WorkHoursDetail>('get_work_hours_by_date', { date: date.date })
    ])
    
    selectedDateDetail.value = detail || {
      date: date.date,
      year: date.year,
      month: date.month,
      day: date.day,
      day_of_week: date.dayOfWeek,
      day_of_week_name: date.dayOfWeekName,
      is_weekend: date.isWeekend,
      date_type: date.dateType
    }
    selectedDateWorkHours.value = workHours
    dateDetailVisible.value = true
  } catch (error) {
    console.error('获取日期详情失败:', error)
  }
}

// 获取日期类型标签
function getDateTypeLabel(type: string): string {
  const labels: Record<string, string> = {
    workday: '工作日',
    holiday: '休假',
    makeup: '补班',
    weekend: '周末'
  }
  return labels[type] || type
}

// 设置日期类型
async function setDateType(type: 'holiday' | 'makeup') {
  if (!selectedDateDetail.value) return
  
  try {
    await invoke('update_holiday_config', {
      request: {
        year: selectedDateDetail.value.year.toString(),
        date: selectedDateDetail.value.date,
        type,
        name: type === 'holiday' ? '节假日' : undefined,
        description: undefined
      }
    })
    
    ElMessage.success(`已设为${type === 'holiday' ? '休假' : '补班'}`)
    dateDetailVisible.value = false
    await fetchMonthDateDetails()
  } catch (error) {
    ElMessage.error('操作失败: ' + String(error))
  }
}

// 重置日期类型
async function resetDateType() {
  if (!selectedDateDetail.value) return
  
  try {
    await invoke('delete_holiday_config', {
      date: selectedDateDetail.value.date
    })
    
    ElMessage.success('已恢复默认状态')
    dateDetailVisible.value = false
    await fetchMonthDateDetails()
  } catch (error) {
    ElMessage.error('操作失败: ' + String(error))
  }
}

// 打开工作时长编辑
function openWorkHoursEdit() {
  if (!selectedDateWorkHours.value) return
  
  editingDateWorkHours.value = JSON.parse(JSON.stringify(selectedDateWorkHours.value))
  showDateWorkHoursEdit.value = true
}

// 重置单日工作时长
async function resetDateWorkHours() {
  if (!editingDateWorkHours.value) return
  
  try {
    await invoke('delete_work_hours', {
      date: editingDateWorkHours.value.date
    })
    
    ElMessage.success('已恢复默认工作时长')
    showDateWorkHoursEdit.value = false
    dateDetailVisible.value = false
    await fetchMonthDateDetails()
  } catch (error) {
    ElMessage.error('操作失败: ' + String(error))
  }
}

// 保存单日工作时长
async function saveDateWorkHours() {
  if (!editingDateWorkHours.value || !dateWorkHoursValidation.value?.is_valid) return
  
  isSavingDateWorkHours.value = true
  
  try {
    await invoke('update_work_hours', {
      request: {
        date: editingDateWorkHours.value.date,
        morning_start_time: editingDateWorkHours.value.morning_session.start_time,
        morning_end_time: editingDateWorkHours.value.morning_session.end_time,
        afternoon_start_time: editingDateWorkHours.value.afternoon_session.start_time,
        afternoon_end_time: editingDateWorkHours.value.afternoon_session.end_time,
        description: editingDateWorkHours.value.description
      }
    })
    
    ElMessage.success('工作时长已保存')
    showDateWorkHoursEdit.value = false
    dateDetailVisible.value = false
    await fetchMonthDateDetails()
  } catch (error) {
    ElMessage.error('保存失败: ' + String(error))
  } finally {
    isSavingDateWorkHours.value = false
  }
}

// 月份导航
function prevMonth() {
  currentDate.value = currentDate.value.subtract(1, 'month')
  fetchMonthDateDetails()
}

function nextMonth() {
  currentDate.value = currentDate.value.add(1, 'month')
  fetchMonthDateDetails()
}

// 日期范围变化处理
async function onDateRangeChange() {
  dateError.value = ''
  batchPreview.value = null
  bulkType.value = null
  
  if (!bulkStartDate.value || !bulkEndDate.value) return
  
  const start = dayjs(bulkStartDate.value)
  const end = dayjs(bulkEndDate.value)
  
  if (!start.isValid() || !end.isValid()) {
    dateError.value = '日期格式无效'
    return
  }
  
  if (end.isBefore(start)) {
    dateError.value = '结束日期不能早于开始日期'
    return
  }
  
  // 获取预览数据
  try {
    const preview = await invoke<BatchPreviewResult>('preview_batch_operation', {
      startDate: bulkStartDate.value,
      endDate: bulkEndDate.value,
      dateType: 'holiday'
    })
    batchPreview.value = preview
  } catch (error) {
    console.error('获取预览失败:', error)
  }
}

// 监听类型变化，更新预览
watch(bulkType, async (newType) => {
  if (!newType || !bulkStartDate.value || !bulkEndDate.value) return
  
  try {
    const preview = await invoke<BatchPreviewResult>('preview_batch_operation', {
      startDate: bulkStartDate.value,
      endDate: bulkEndDate.value,
      dateType: newType
    })
    batchPreview.value = preview
  } catch (error) {
    console.error('获取预览失败:', error)
  }
})

// 应用批量更改
function applyBulkChanges() {
  if (!canApplyBulk.value) return
  confirmDialogVisible.value = true
}

// 确认应用批量更改
async function confirmApplyChanges() {
  if (!bulkType.value || !batchPreview.value) return
  
  isApplying.value = true
  
  try {
    const dates = batchPreview.value.preview.map(p => p.date)
    
    await invoke('batch_update_holiday_configs', {
      request: {
        dates,
        type: bulkType.value,
        name: bulkHolidayName.value || undefined
      }
    })
    
    ElMessage.success(`成功应用设置到 ${dates.length} 个日期`)
    
    // 重置表单
    bulkStartDate.value = ''
    bulkEndDate.value = ''
    bulkType.value = null
    bulkHolidayName.value = ''
    batchPreview.value = null
    confirmDialogVisible.value = false
    
    // 刷新日历
    await fetchMonthDateDetails()
  } catch (error) {
    console.error('批量应用失败:', error)
    ElMessage.error('批量应用失败: ' + String(error))
  } finally {
    isApplying.value = false
  }
}

// 格式化日期显示
function formatDate(dateStr: string): string {
  if (!dateStr) return ''
  return dayjs(dateStr).format('YYYY年M月D日')
}

// 格式化小时
function formatHours(minutes: number): string {
  return (minutes / 60).toFixed(1)
}

// 格式化分钟为小时和分钟
function formatMinutes(minutes: number): string {
  const hours = Math.floor(minutes / 60)
  const mins = minutes % 60
  if (hours > 0 && mins > 0) {
    return `${hours}小时${mins}分钟`
  } else if (hours > 0) {
    return `${hours}小时`
  } else {
    return `${mins}分钟`
  }
}

// 验证工作时长
function validateWorkHours(
  morningStart: string,
  morningEnd: string,
  afternoonStart: string,
  afternoonEnd: string
): WorkHoursValidationResult {
  // 解析时间
  const parseTime = (timeStr: string): number => {
    const [hours, minutes] = timeStr.split(':').map(Number)
    return hours * 60 + minutes
  }
  
  const morningStartMinutes = parseTime(morningStart)
  const morningEndMinutes = parseTime(morningEnd)
  const afternoonStartMinutes = parseTime(afternoonStart)
  const afternoonEndMinutes = parseTime(afternoonEnd)
  
  // 验证时间顺序
  if (morningStartMinutes >= morningEndMinutes) {
    return {
      is_valid: false,
      error_message: '上午开始时间必须早于结束时间',
      total_minutes: 0
    }
  }
  
  if (afternoonStartMinutes >= afternoonEndMinutes) {
    return {
      is_valid: false,
      error_message: '下午开始时间必须早于结束时间',
      total_minutes: 0
    }
  }
  
  if (morningEndMinutes >= afternoonStartMinutes) {
    return {
      is_valid: false,
      error_message: '上午结束时间必须早于下午开始时间',
      total_minutes: 0
    }
  }
  
  const totalMinutes = (morningEndMinutes - morningStartMinutes) + 
                       (afternoonEndMinutes - afternoonStartMinutes)
  
  return {
    is_valid: true,
    total_minutes: totalMinutes
  }
}

// 保存默认工作时长
async function saveDefaultWorkHours() {
  if (!defaultWorkHoursValidation.value?.is_valid) return
  
  isSavingDefault.value = true
  
  try {
    const result = await invoke<DefaultWorkHours>('update_default_work_hours', {
      request: {
        morning_start_time: editingDefaultWorkHours.value.morning_start_time,
        morning_end_time: editingDefaultWorkHours.value.morning_end_time,
        afternoon_start_time: editingDefaultWorkHours.value.afternoon_start_time,
        afternoon_end_time: editingDefaultWorkHours.value.afternoon_end_time
      }
    })
    
    defaultWorkHours.value = result
    ElMessage.success('默认工作时长已更新')
    showDefaultWorkHoursEdit.value = false
    await fetchMonthDateDetails()
  } catch (error) {
    ElMessage.error('保存失败: ' + String(error))
  } finally {
    isSavingDefault.value = false
  }
}

// 监听默认工作时长编辑对话框
watch(showDefaultWorkHoursEdit, (show) => {
  if (show) {
    editingDefaultWorkHours.value = {
      morning_start_time: defaultWorkHours.value.morning_start_time,
      morning_end_time: defaultWorkHours.value.morning_end_time,
      afternoon_start_time: defaultWorkHours.value.afternoon_start_time,
      afternoon_end_time: defaultWorkHours.value.afternoon_end_time
    }
  }
})

// ==================== 生命周期 ====================

onMounted(() => {
  fetchMonthDateDetails()
  fetchDefaultWorkHours()
})
</script>

<style scoped lang="scss">
.holiday-settings-content {
  width: 100%;
  height: 100%;
  overflow: auto;
}

// 主内容区
.main-container {
  display: flex;
  gap: 24px;
  margin-bottom: 24px;
  min-height: 600px;
}

// 日历区域
.calendar-section {
  flex: 1;
  background: var(--bg-card);
  border-radius: 48px;
  padding: 32px;
  box-shadow: 0 1px 2px var(--shadow-sm);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  margin-bottom: 16px;
}

.title-area {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.section-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: 8px;
}

.section-desc {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}

.month-nav {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--bg-input);
  padding: 4px;
  border-radius: 48px;
}

.nav-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 32px;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
  transition: all 0.2s;

  &:hover {
    background: var(--bg-card);
  }
}

.current-month {
  text-align: center;
  font-size: 16px;
  font-weight: 500;
  color: #1e293b;
  line-height: 1.4;
  padding: 0 16px;
  min-width: 80px;
}

.info-tip {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-secondary);
  font-size: 13px;
  margin-bottom: 16px;
  padding: 8px 12px;
  background: var(--bg-card);
  border-radius: 8px;
}

// 日历网格
.calendar-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 1px;
  background: var(--border-color);
  border-radius: 32px;
  overflow: hidden;
  border: 1px solid var(--border-color);
}

.weekday-header {
  background: var(--bg-card);
  padding: 12px;
  text-align: center;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
}

.calendar-cell {
  background: var(--bg-card);
  padding: 12px 8px;
  min-height: 90px;
  cursor: pointer;
  transition: all 0.2s;
  position: relative;

  &:hover {
    background: var(--bg-input);
    transform: scale(1.02);
    z-index: 1;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  &.other-month {
    background: var(--bg-card);
    color: var(--text-tertiary);
    cursor: default;

    &:hover {
      background: var(--bg-card);
      transform: none;
      box-shadow: none;
    }
  }

  &.today {
    .date-number {
      background: #3b82f6;
      color: var(--bg-card);
      width: 28px;
      height: 28px;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 50%;
    }
  }

  &.holiday {
    background: #ecfdf5;
    border: 2px solid var(--color-success);

    .date-number {
      color: #047857;
    }
  }

  &.makeup {
    background: var(--bg-card)7ed;
    border: 2px solid #fb923c;

    .date-number {
      color: #c2410c;
    }
  }

  &.weekend {
    background: var(--bg-input);

    .date-number {
      color: var(--text-secondary);
    }
  }

  &.workday {
    .date-number {
      color: #1e293b;
    }
  }

  &.has-custom-hours {
    &::after {
      content: '';
      position: absolute;
      top: 4px;
      right: 4px;
      width: 6px;
      height: 6px;
      background: #3b82f6;
      border-radius: 50%;
    }
  }

  &.loading {
    opacity: 0.6;
    pointer-events: none;
  }
}

.date-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.date-number {
  font-size: 15px;
  font-weight: 500;
  color: #1e293b;
}

.type-icon {
  font-size: 14px;

  &.holiday-icon {
    color: var(--color-success);
  }

  &.makeup-icon {
    color: #ea580c;
  }

  &.weekend-icon {
    color: var(--text-secondary);
  }
}

.date-label {
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: -0.5px;
  margin-top: 4px;

  .holiday & {
    color: var(--color-success);
  }

  .makeup & {
    color: #ea580c;
  }

  .weekend & {
    color: var(--text-secondary);
  }
}

.hours-label {
  font-size: 10px;
  font-weight: 600;
  color: #3b82f6;
  margin-top: 2px;
}

.date-name {
  font-size: 10px;
  color: var(--text-secondary);
  margin-top: 2px;
  line-height: 1.3;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

// 日历底部
.calendar-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 16px;
}

.legend {
  display: flex;
  gap: 20px;
  flex-wrap: wrap;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: #475569;
}

.legend-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;

  &.workday-dot {
    background: var(--border-color);
    border: 2px solid var(--border-color);
  }

  &.holiday-dot {
    background: var(--color-success);
  }

  &.makeup-dot {
    background: #fb923c;
  }

  &.weekend-dot {
    background: var(--text-tertiary);
  }

  &.custom-hours-dot {
    background: #3b82f6;
  }
}

// 批量操作区域
.bulk-actions-section {
  width: 320px;
  background: var(--bg-card);
  border-radius: 48px;
  padding: 32px;
  box-shadow: 0 1px 2px var(--shadow-sm);
  flex-shrink: 0;
}

.bulk-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 24px;
}

.bulk-icon {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: rgba(0, 88, 190, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-primary);
  font-size: 20px;
}

.bulk-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.bulk-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.date-range {
  display: flex;
  align-items: center;
  gap: 8px;
}

.date-input-wrapper {
  position: relative;
  flex: 1;
}

.date-icon {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-tertiary);
  font-size: 14px;
}

.date-input {
  width: 100%;
  padding: 10px 10px 10px 32px;
  border: 1px solid var(--border-color);
  border-radius: 12px;
  background: var(--bg-card);
  font-size: 13px;
  color: #1e293b;
  outline: none;
  transition: all 0.2s;

  &:focus {
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }
}

.date-separator {
  color: var(--text-tertiary);
  font-size: 13px;
}

.error-text {
  font-size: 12px;
  color: var(--color-error);
  margin: 0;
}

.hint-text {
  font-size: 12px;
  color: var(--color-warning);
  margin: 0;
}

.type-options {
  display: flex;
  gap: 12px;
}

.type-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px;
  border: 2px solid var(--border-color);
  border-radius: 16px;
  background: var(--bg-card);
  cursor: pointer;
  transition: all 0.2s;

  span {
    font-size: 12px;
    font-weight: 600;
  }

  &.holiday-btn {
    color: #047857;

    &:hover:not(.disabled) {
      border-color: var(--color-success);
      background: #f0fdf4;
    }

    &.active {
      background: #ecfdf5;
      border-color: var(--color-success);
    }

    &.disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }
  }

  &.makeup-btn {
    color: #c2410c;

    &:hover:not(.disabled) {
      border-color: #fb923c;
      background: var(--bg-card)7ed;
    }

    &.active {
      background: var(--bg-card)7ed;
      border-color: #fb923c;
    }

    &.disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }
  }
}

.text-input {
  padding: 10px 14px;
  border: 1px solid var(--border-color);
  border-radius: 12px;
  font-size: 14px;
  outline: none;
  transition: all 0.2s;

  &:focus {
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  &::placeholder {
    color: var(--text-tertiary);
  }
}

// 预览区域
.preview-section {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 16px;
  border: 1px solid var(--border-color);
}

.preview-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 600;
  color: #475569;
  margin-bottom: 12px;
}

.preview-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.preview-item {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
}

.preview-label {
  color: var(--text-secondary);
}

.preview-value {
  font-weight: 600;
  color: #1e293b;

  &.highlight-holiday {
    color: var(--color-success);
  }

  &.highlight-makeup {
    color: #ea580c;
  }

  &.highlight-error {
    color: var(--color-error);
  }
}

.apply-btn {
  width: 100%;
  padding: 14px;
  border: none;
  border-radius: 48px;
  background: var(--color-primary);
  color: var(--bg-card);
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: 0 10px 15px -3px rgba(0, 88, 190, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;

  &:hover:not(:disabled) {
    background: #004a9e;
    transform: translateY(-1px);
  }

  &:disabled {
    background: var(--border-color);
    cursor: not-allowed;
    box-shadow: none;
  }
}

.loading-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

// 默认工作时长区域
.default-work-hours-section {
  background: var(--bg-card);
  border-radius: 48px;
  padding: 32px;
  box-shadow: 0 1px 2px var(--shadow-sm);
  margin-bottom: 24px;

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }

  .edit-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 16px;
    border: 1px solid var(--border-color);
    border-radius: 24px;
    background: var(--bg-card);
    color: var(--color-primary);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      background: var(--bg-input);
      border-color: var(--color-primary);
    }
  }
}

.work-hours-display {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

.session-card {
  flex: 1;
  min-width: 180px;
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.session-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 12px;
}

.session-time {
  display: flex;
  align-items: center;
  gap: 8px;

  .time {
    font-size: 24px;
    font-weight: 600;
    color: var(--color-primary);
  }

  .separator {
    color: var(--text-tertiary);
    font-size: 18px;
  }
}

.total-card {
  width: 140px;
  background: linear-gradient(135deg, var(--color-primary) 0%, #3b82f6 100%);
  border-radius: 16px;
  padding: 20px;
  color: var(--bg-card);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.total-label {
  font-size: 12px;
  font-weight: 500;
  opacity: 0.9;
  margin-bottom: 8px;
}

.total-value {
  font-size: 32px;
  font-weight: 700;
}

// 确认对话框 - 苹果风格
.confirm-dialog {
  .confirm-content-apple {
    padding: 8px 8px 16px;
    text-align: center;
  }

  .confirm-icon {
    width: 56px;
    height: 56px;
    margin: 0 auto 16px;
    border-radius: 50%;
    background: var(--bg-card)4e0;
    display: flex;
    align-items: center;
    justify-content: center;

    .el-icon {
      font-size: 28px;
      color: #f5a623;
    }
  }

  .confirm-title {
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 6px 0;
    letter-spacing: -0.01em;
  }

  .confirm-desc {
    font-size: 15px;
    color: var(--text-secondary);
    margin: 0 0 24px 0;
  }

  .confirm-details {
    background: var(--bg-card-hover);
    border-radius: 16px;
    padding: 20px;
    margin-bottom: 20px;
    text-align: left;
  }

  .confirm-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 0;
    border-bottom: 1px solid var(--bg-hover);

    &:last-child {
      border-bottom: none;
      padding-bottom: 0;
    }

    &:first-child {
      padding-top: 0;
    }
  }

  .item-label {
    font-size: 14px;
    color: var(--text-secondary);
    font-weight: 500;
  }

  .item-value {
    font-size: 14px;
    color: var(--text-primary);
    font-weight: 600;

    &.highlight {
      color: var(--color-primary);
    }

    &.type-badge {
      padding: 4px 12px;
      border-radius: 20px;
      font-size: 13px;

      &.holiday {
        background: #d4f5e0;
        color: #1d7f3e;
      }

      &.makeup {
        background: #ffe8d4;
        color: #b45c1a;
      }
    }
  }

  .confirm-warning-apple {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    color: var(--color-warning);
    font-size: 13px;
    font-weight: 500;
    padding: 12px 16px;
    background: var(--bg-card)8e7;
    border-radius: 12px;

    .el-icon {
      font-size: 16px;
    }
  }
}

// 通用对话框底部
.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.apple-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.apple-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 10px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);

  &:active {
    transform: scale(0.96);
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  &.secondary {
    background: var(--border-light);
    color: var(--text-primary);

    &:hover:not(:disabled) {
      background: #d8d8dd;
    }
  }

  &.primary {
    background: var(--color-primary);
    color: var(--bg-card);

    &:hover:not(:disabled) {
      background: #0077ed;
    }
  }
}

// 苹果风格对话框全局样式
:deep(.apple-style-dialog) {
  .el-dialog {
    border-radius: 20px;
    box-shadow: 
      0 25px 50px -12px rgba(0, 0, 0, 0.25),
      0 0 0 1px var(--shadow-sm);
    overflow: hidden;
  }

  .el-dialog__header {
    padding: 20px 24px 16px;
    margin: 0;
    border-bottom: 1px solid var(--divider-color);
  }

  .el-dialog__title {
    font-size: 17px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .el-dialog__headerbtn {
    top: 20px;
    right: 20px;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: var(--shadow-sm);
    transition: all 0.2s ease;

    &:hover {
      background: rgba(0, 0, 0, 0.1);
    }

    .el-dialog__close {
      color: var(--text-secondary);
      font-size: 14px;
      font-weight: 500;
    }
  }

  .el-dialog__body {
    padding: 0;
  }

  .el-dialog__footer {
    padding: 16px 24px 24px;
    border-top: 1px solid var(--divider-color);
  }
}

// 日期详情对话框
.date-detail-dialog {
  .date-detail-content {
    padding: 0;
  }

  .detail-header {
    text-align: center;
    padding: 24px 24px 20px;
    background: linear-gradient(180deg, var(--bg-card-hover) 0%, var(--bg-card) 100%);
    border-bottom: 1px solid var(--bg-hover);
  }

  .detail-date {
    font-size: 28px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .detail-weekday {
    font-size: 15px;
    color: var(--text-secondary);
    margin-top: 6px;
    font-weight: 500;
  }

  .detail-status {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin: 20px 24px;
    padding: 14px 18px;
    background: var(--bg-card-hover);
    border-radius: 14px;
  }

  .status-label {
    font-size: 15px;
    color: var(--text-secondary);
    font-weight: 500;
  }

  .status-value {
    font-size: 14px;
    font-weight: 600;
    padding: 6px 14px;
    border-radius: 20px;

    &.workday {
      background: var(--border-light);
      color: #5e5e64;
    }

    &.holiday {
      background: #d4f5e0;
      color: #1d7f3e;
    }

    &.makeup {
      background: #ffe8d4;
      color: #b45c1a;
    }

    &.weekend {
      background: var(--border-light);
      color: var(--text-secondary);
    }
  }
}

// 工作时长显示区域 - 苹果风格
.work-hours-section {
  background: var(--bg-card-hover);
  border-radius: 16px;
  padding: 20px;
  margin: 0 24px 20px;
}

.work-hours-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 16px;
}

.work-hours-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.custom-badge {
  font-size: 11px;
  font-weight: 600;
  color: var(--color-primary);
  background: rgba(0, 113, 227, 0.1);
  padding: 3px 10px;
  border-radius: 12px;
  letter-spacing: 0.02em;
}

.default-badge {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  background: var(--border-light);
  padding: 3px 10px;
  border-radius: 12px;
  letter-spacing: 0.02em;
}

.work-hours-summary {
  display: flex;
  justify-content: center;
  margin-bottom: 16px;
}

.work-hours-total {
  text-align: center;
  padding: 16px 32px;
  background: var(--bg-card);
  border-radius: 16px;
  box-shadow: 0 2px 8px var(--bg-hover);

  .total-hours {
    font-size: 40px;
    font-weight: 700;
    color: var(--color-primary);
    display: block;
    letter-spacing: -0.02em;
  }

  .total-label {
    font-size: 13px;
    color: var(--text-secondary);
    margin-top: 4px;
    font-weight: 500;
  }
}

.work-sessions {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.work-session {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px;
  background: var(--bg-card);
  border-radius: 12px;
  font-size: 14px;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.02);

  .session-name {
    font-weight: 600;
    color: var(--text-primary);
    width: 40px;
  }

  .session-time {
    color: #424245;
    flex: 1;
    text-align: center;
    font-weight: 500;
  }

  .session-duration {
    color: var(--text-secondary);
    font-size: 13px;
    width: 70px;
    text-align: right;
    font-weight: 500;
  }
}

.detail-actions {
  padding: 0 24px 24px;

  .action-hint {
    font-size: 13px;
    color: var(--text-secondary);
    margin: 0 0 12px 0;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
}

.action-buttons {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 14px;
  border: none;
  border-radius: 12px;
  background: var(--bg-card-hover);
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  font-size: 15px;
  font-weight: 600;

  &:hover {
    transform: scale(1.02);
  }

  &:active {
    transform: scale(0.98);
  }

  &.holiday {
    color: #1d7f3e;
    background: #d4f5e0;

    &:hover {
      background: #c2f0d3;
    }
  }

  &.makeup {
    color: #b45c1a;
    background: #ffe8d4;

    &:hover {
      background: #ffdfc2;
    }
  }

  &.reset {
    color: #424245;
    background: var(--border-light);

    &:hover {
      background: #d8d8dd;
    }
  }

  &.work-hours {
    color: var(--color-primary);
    background: rgba(0, 113, 227, 0.1);

    &:hover {
      background: rgba(0, 113, 227, 0.15);
    }
  }
}

// 工作时长编辑对话框 - 苹果风格
.work-hours-dialog {
  .work-hours-edit-content {
    padding: 20px 24px;
  }

  .edit-header-apple {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--divider-color);
  }

  .date-info {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .edit-date {
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .custom-indicator {
    font-size: 11px;
    font-weight: 600;
    color: var(--color-primary);
    background: rgba(0, 113, 227, 0.1);
    padding: 3px 10px;
    border-radius: 12px;
  }

  .reset-to-default-btn-apple {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 14px;
    border: none;
    border-radius: 20px;
    background: var(--bg-card-hover);
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;

    &:hover {
      background: var(--border-light);
      color: #424245;
    }

    &:active {
      transform: scale(0.96);
    }
  }

  .section-header-with-icon {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 12px;

    .icon-wrapper {
      width: 28px;
      height: 28px;
      border-radius: 8px;
      display: flex;
      align-items: center;
      justify-content: center;

      &.morning {
        background: var(--bg-card)4e0;
        color: #f5a623;
      }

      &.afternoon {
        background: #e8f4fd;
        color: #5ac8fa;
      }

      &.note {
        background: #f0f0f5;
        color: var(--text-secondary);
      }

      .el-icon {
        font-size: 16px;
      }
    }

    h4 {
      font-size: 15px;
      font-weight: 600;
      color: var(--text-primary);
      margin: 0;
    }
  }

  .edit-section {
    margin-bottom: 24px;
  }

  .time-inputs {
    display: flex;
    gap: 12px;
  }

  .time-field {
    flex: 1;

    label {
      display: block;
      font-size: 12px;
      color: var(--text-secondary);
      margin-bottom: 8px;
      font-weight: 500;
      text-transform: uppercase;
      letter-spacing: 0.03em;
    }
  }

  .time-input-wrapper {
    position: relative;
    background: var(--bg-card-hover);
    border-radius: 12px;
    padding: 2px;
    transition: all 0.2s ease;

    &:focus-within {
      background: rgba(0, 113, 227, 0.1);
    }
  }

  .time-input {
    width: 100%;
    padding: 12px 14px;
    border: none;
    border-radius: 10px;
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    background: var(--bg-card);
    outline: none;
    text-align: center;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', sans-serif;
    letter-spacing: 0.02em;

    &::-webkit-calendar-picker-indicator {
      display: none;
    }
  }

  .text-input-wrapper {
    background: var(--bg-card-hover);
    border-radius: 12px;
    padding: 2px;
    transition: all 0.2s ease;

    &:focus-within {
      background: rgba(0, 113, 227, 0.1);
    }
  }

  .text-input-apple {
    width: 100%;
    padding: 14px 16px;
    border: none;
    border-radius: 10px;
    font-size: 15px;
    color: var(--text-primary);
    background: var(--bg-card);
    outline: none;

    &::placeholder {
      color: #c4c4c7;
    }
  }

  .validation-result {
    margin-top: 20px;
  }

  .validation-error {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--color-error);
    font-size: 14px;
    font-weight: 500;
    background: #ffebeb;
    padding: 14px 16px;
    border-radius: 12px;

    .el-icon {
      font-size: 18px;
    }
  }

  .validation-success {
    background: #f0fff4;
    padding: 14px 16px;
    border-radius: 12px;

    .success-content {
      display: flex;
      align-items: center;
      gap: 12px;
    }

    .el-icon {
      font-size: 22px;
      color: #34c759;
    }

    .success-info {
      display: flex;
      flex-direction: column;
      gap: 2px;
    }

    .success-label {
      font-size: 12px;
      color: var(--text-secondary);
      font-weight: 500;
    }

    .success-value {
      font-size: 16px;
      color: var(--text-primary);
      font-weight: 700;
    }
  }

  // 苹果风格按钮
  .apple-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
  }
}

// 响应式适配
@media (max-width: 1200px) {
  .main-container {
    flex-direction: column;
  }

  .bulk-actions-section {
    width: 100%;
  }

  .work-hours-display {
    flex-direction: column;
  }

  .total-card {
    width: 100%;
  }
}

@media (max-width: 768px) {
  .calendar-section {
    padding: 16px;
    border-radius: 24px;
  }

  .calendar-cell {
    min-height: 60px;
    padding: 8px 4px;
  }

  .date-number {
    font-size: 13px;
  }

  .date-label {
    font-size: 9px;
  }

  .legend {
    flex-wrap: wrap;
    gap: 12px;
  }

  .section-title {
    font-size: 20px;
  }

  .bulk-actions-section {
    padding: 20px;
    border-radius: 24px;
  }

  .default-work-hours-section {
    padding: 20px;
    border-radius: 24px;
  }

  .time-inputs {
    flex-direction: column;
    gap: 12px;
  }
}
</style>
