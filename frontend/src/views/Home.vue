<template>
  <div class="home-container">
    <!-- 统计卡片区域 - 现代块级布局 -->
    <div class="stats-grid">
      <el-card class="stat-card glass-card stat-card-primary">
        <template #header>
          <div class="card-header">
            <div class="card-icon-wrapper">
              <el-icon class="card-icon"><clock /></el-icon>
            </div>
            <h3 class="card-title">时间最长的5个任务</h3>
          </div>
        </template>
        <div class="stat-content">
          <div v-for="task in longestTasks" :key="task.id" class="task-item">
            <div class="task-info">
              <span class="task-title">{{ task.title }}</span>
              <span class="task-duration">{{ task.totalHours || '0' }}分钟</span>
            </div>
            <el-progress 
              :percentage="0" 
              :show-text="false" 
              :stroke-width="4"
            />
          </div>
        </div>
      </el-card>

      <el-card class="stat-card glass-card stat-card-secondary">
        <template #header>
          <div class="card-header">
            <div class="card-icon-wrapper">
              <el-icon class="card-icon"><trend-charts /></el-icon>
            </div>
            <h3 class="card-title">每日处理任务数</h3>
          </div>
        </template>
        <div class="stat-content">
          <v-chart class="chart" :option="dailyTasksOption" />
        </div>
      </el-card>

      <el-card class="stat-card glass-card stat-card-accent">
        <template #header>
          <div class="card-header">
            <div class="card-icon-wrapper">
              <el-icon class="card-icon"><document-add /></el-icon>
            </div>
            <h3 class="card-title">每日创建任务数</h3>
          </div>
        </template>
        <div class="stat-content">
          <v-chart class="chart" :option="dailyCreatedOption" />
        </div>
      </el-card>
    </div>

    <!-- 甘特图区域 - 大胆块级设计 -->
    <el-card class="gantt-card glass-card">
      <template #header>
        <div class="gantt-header">
          <div class="card-icon-wrapper">
            <el-icon class="card-icon"><data-analysis /></el-icon>
          </div>
          <h2 class="gantt-title">任务处理记录甘特图</h2>
        </div>
      </template>
      <div class="gantt-content">
        <v-chart class="gantt-chart" :option="ganttOption" />
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { use } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { LineChart, BarChart, CustomChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent,
  DataZoomComponent
} from 'echarts/components'
import VChart from 'vue-echarts'
import { Clock, TrendCharts, DocumentAdd, DataAnalysis } from '@element-plus/icons-vue'
import { 
  getTopTimeConsumingTasks, 
  getDailyProcessedTasks, 
  getDailyCreatedTasks, 
  getGanttChartByTags 
} from '@/api/stats'

use([
  CanvasRenderer,
  LineChart,
  BarChart,
  CustomChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent,
  DataZoomComponent
])

interface Task {
  id: number
  title: string
  totalHours?: string
  totalMinutes: number
  status: string
  activitiesByType?: Record<string, string>
}

interface DailyStats {
  dates: string[]
  processedCounts: number[]
  totalProcessed: number
}

interface CreatedStats {
  dates: string[]
  createdCounts: number[]
  totalCreated: number
}

interface GanttData {
  tagGroups: Array<{
    tagId: number | null
    tagName: string
    tagColor: string
    tasks: Array<{
      id: number
      title: string
      status: string
      progress: number
      priority: string
      segments: Array<{
        type: string
        typeDescription: string
        startTime: string
        endTime: string
        durationMinutes: number
        description: string
      }>
    }>
  }>
}

const longestTasks = ref<Task[]>([])
const dailyProcessedStats = ref<DailyStats>({ dates: [], processedCounts: [], totalProcessed: 0 })
const dailyCreatedStats = ref<CreatedStats>({ dates: [], createdCounts: [], totalCreated: 0 })
const ganttData = ref<GanttData>({ tagGroups: [] })

// 计算选项函数
const dailyTasksOption = computed(() => ({
  xAxis: {
    type: 'category',
    data: dailyProcessedStats.value.dates,
    axisLabel: {
      formatter: (value: string) => {
        const date = new Date(value)
        return `${date.getMonth() + 1}/${date.getDate()}`
      }
    }
  },
  yAxis: {
    type: 'value',
    name: '任务数'
  },
  tooltip: {
    trigger: 'axis',
    formatter: function(params: any) {
      const data = params[0]
      return `${data.axisValueLabel}<br/>处理任务数: ${data.value}`
    }
  },
  series: [{
    data: dailyProcessedStats.value.processedCounts,
    type: 'line',
    smooth: true,
    areaStyle: {
      opacity: 0.3
    },
    itemStyle: {
      color: '#409eff'
    },
    lineStyle: {
      width: 2
    }
  }],
  grid: {
    left: '3%',
    right: '4%',
    bottom: '3%',
    containLabel: true
  }
}))

const dailyCreatedOption = computed(() => ({
  xAxis: {
    type: 'category',
    data: dailyCreatedStats.value.dates,
    axisLabel: {
      formatter: (value: string) => {
        const date = new Date(value)
        return `${date.getMonth() + 1}/${date.getDate()}`
      }
    }
  },
  yAxis: {
    type: 'value',
    name: '任务数'
  },
  tooltip: {
    trigger: 'axis',
    formatter: function(params: any) {
      const data = params[0]
      return `${data.axisValueLabel}<br/>创建任务数: ${data.value}`
    }
  },
  series: [{
    data: dailyCreatedStats.value.createdCounts,
    type: 'bar',
    itemStyle: {
      color: '#67c23a'
    }
  }],
  grid: {
    left: '3%',
    right: '4%',
    bottom: '3%',
    containLabel: true
  }
}))

// 甘特图数据处理
const ganttOption = computed(() => {
  if (!ganttData.value.tagGroups || ganttData.value.tagGroups.length === 0) {
    return { title: { text: '暂无数据' } }
  }
  
  // 准备所有任务和标签
  const tasks: any[] = []
  const categories: string[] = []
  const tagColors: Record<string, string> = {}
  
  // 遍历每个标签分组
  ganttData.value.tagGroups.forEach(tagGroup => {
    tagGroup.tasks.forEach(task => {
      const taskName = `[${tagGroup.tagName}] ${task.title}`
      categories.push(taskName)
      tagColors[taskName] = tagGroup.tagColor || '#409eff'
      
      // 遍历每个任务的分段
      if (task.segments && task.segments.length > 0) {
        task.segments.forEach(segment => {
          const startTime = new Date(segment.startTime).getTime()
          const endTime = segment.endTime ? new Date(segment.endTime).getTime() : Date.now()
          const duration = segment.durationMinutes || 0
          
          tasks.push([
            taskName,                   // 任务名称
            startTime,                   // 开始时间
            endTime,                     // 结束时间
            startTime,                   // 自定义渲染需要的开始时间
            endTime,                     // 自定义渲染需要的结束时间
            segment.typeDescription,      // 活动类型描述
            duration,                    // 持续时间（分钟）
            tagColors[taskName],         // 标签颜色
            segment.type                 // 活动类型
          ])
        })
      }
    })
  })
  
  return {
    tooltip: {
      formatter: function(params: any) {
        const data = params.data
        return `
          <div>
            <div>${data[0]}</div>
            <div>活动类型: ${data[5]}</div>
            <div>开始时间: ${new Date(data[1]).toLocaleString()}</div>
            <div>结束时间: ${new Date(data[2]).toLocaleString()}</div>
            <div>持续时间: ${Math.floor(data[6] / 60)}小时${data[6] % 60}分钟</div>
          </div>
        `
      }
    },
    title: {
      text: '任务处理记录'
    },
    legend: {
      data: ['工作', '会议', '学习', '其他']
    },
    grid: {
      height: categories.length * 40 + 60,
      top: 80,
      left: 100,
      right: 100
    },
    xAxis: {
      type: 'time'
    },
    yAxis: {
      type: 'category',
      data: categories,
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: {
        margin: 12,
        formatter: function(value: string) {
          // 截断过长的任务名称
          if (value.length > 25) {
            return value.substring(0, 25) + '...'
          }
          return value
        }
      }
    },
    dataZoom: [{
      type: 'slider',
      xAxisIndex: 0,
      filterMode: 'filter'
    }, {
      type: 'inside',
      xAxisIndex: 0,
      filterMode: 'filter'
    }],
    series: [{
      type: 'custom',
      renderItem: function(_params: any, api: any) {
        const taskName = api.value(0)
        const start = api.coord([api.value(1), taskName])
        const end = api.coord([api.value(2), taskName])
        const height = api.size([0, 1])[1] * 0.6
        
        // 根据活动类型设置不同颜色
        const activityType = api.value(8)
        let itemColor = '#409eff' // 默认蓝色
        
        switch(activityType) {
          case 'WORK':
            itemColor = '#409eff'
            break
          case 'MEETING':
            itemColor = '#e6a23c'
            break
          case 'STUDY':
            itemColor = '#67c23a'
            break
          case 'CREATED':
            itemColor = '#909399'
            break
          case 'STARTED':
            itemColor = '#409eff'
            break
          case 'PAUSED':
            itemColor = '#f56c6c'
            break
          case 'RESUMED':
            itemColor = '#409eff'
            break
          case 'COMPLETED':
            itemColor = '#67c23a'
            break
          default:
            itemColor = '#909399'
        }
        
        return {
          type: 'rect',
          shape: {
            x: start[0],
            y: start[1] - height / 2,
            width: end[0] - start[0],
            height: height,
            r: [4, 4, 0, 0]
          },
          style: {
            fill: itemColor,
            opacity: 0.8
          }
        }
      },
      data: tasks,
      z: 100
    }]
  }
})

// 加载数据函数
const loadHomeData = async () => {
  try {
    // 并行加载所有数据
    const [topTasksResponse, processedTasksResponse, createdTasksResponse, ganttResponse] = await Promise.all([
      getTopTimeConsumingTasks(),
      getDailyProcessedTasks(),
      getDailyCreatedTasks(),
      getGanttChartByTags()
    ])
    
    // 从响应中提取实际数据
    const topTasks = topTasksResponse.data || topTasksResponse
    const processedTasks = processedTasksResponse.data || processedTasksResponse
    const createdTasks = createdTasksResponse.data || createdTasksResponse
    const ganttDataResponse = ganttResponse.data || ganttResponse
    
    // 设置最长时长任务
    if (Array.isArray(topTasks)) {
      longestTasks.value = topTasks.map(task => ({
        id: task.taskId,
        title: task.title,
        totalHours: task.totalHours,
        totalMinutes: task.totalMinutes,
        status: task.status,
        activitiesByType: task.activitiesByType
      }))
    } else {
      console.warn('topTasks 不是数组:', topTasks)
      longestTasks.value = []
    }
    
    // 设置每日处理任务数据
    dailyProcessedStats.value = processedTasks
    
    // 设置每日创建任务数据
    dailyCreatedStats.value = createdTasks
    
    // 设置甘特图数据
    ganttData.value = ganttDataResponse
  } catch (error) {
    console.error('加载首页数据失败:', error)
  }
}

onMounted(() => {
  loadHomeData()
})
</script>

<style scoped>
.home-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 32px;
  padding: 24px 0;
}

/* 统计卡片网格 - 现代块级布局 */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 32px;
}

/* Glassmorphism卡片 - 创意前卫 */
.stat-card {
  height: 320px;
  border: none;
  overflow: hidden;
}

.stat-card-primary {
  --card-accent: var(--color-primary);
}

.stat-card-secondary {
  --card-accent: var(--color-accent);
}

.stat-card-accent {
  --card-accent: var(--color-warning);
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--color-border-light);
}

.card-icon-wrapper {
  width: 40px;
  height: 40px;
  border-radius: var(--radius);
  background: var(--card-accent);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 20px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.card-icon {
  font-size: 20px;
  font-weight: 600;
}

.card-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text);
  margin: 0;
  letter-spacing: -0.01em;
}

.stat-content {
  height: 240px;
  display: flex;
  flex-direction: column;
  padding-top: 8px;
}

.task-item {
  margin-bottom: 16px;
  padding: 12px;
  background: rgba(255, 255, 255, 0.6);
  border-radius: var(--radius);
  border-left: 4px solid var(--card-accent);
  transition: var(--transition);
}

.task-item:hover {
  background: rgba(255, 255, 255, 0.9);
  transform: translateX(4px);
}

.task-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.task-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text);
}

.task-duration {
  font-size: 12px;
  font-weight: 600;
  color: var(--card-accent);
  background: rgba(255, 255, 255, 0.8);
  padding: 2px 8px;
  border-radius: var(--radius);
}

.chart {
  width: 100%;
  height: 100%;
}

/* 甘特图卡片 - 大胆设计 */
.gantt-card {
  flex: 1;
  border: none;
  overflow: hidden;
}

.gantt-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--color-border-light);
}

.gantt-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--color-text);
  margin: 0;
  letter-spacing: -0.01em;
}

.gantt-content {
  height: 420px;
  padding-top: 8px;
}

.gantt-chart {
  width: 100%;
  height: 100%;
}
</style>