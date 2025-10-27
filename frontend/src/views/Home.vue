<template>
  <div class="home-container">
    <!-- 统计卡片区域 -->
    <div class="stats-grid">
      <el-card class="stat-card">
        <template #header>
          <div class="card-header">
            <el-icon><clock /></el-icon>
            <span>时间最长的5个任务</span>
          </div>
        </template>
        <div class="stat-content">
          <div v-for="task in longestTasks" :key="task.id" class="task-item">
            <div class="task-info">
              <span class="task-title">{{ task.title }}</span>
              <span class="task-duration">{{ task.duration }}天</span>
            </div>
            <el-progress 
              :percentage="task.progress" 
              :show-text="false" 
              :stroke-width="4"
            />
          </div>
        </div>
      </el-card>

      <el-card class="stat-card">
        <template #header>
          <div class="card-header">
            <el-icon><trend-charts /></el-icon>
            <span>每日处理任务数</span>
          </div>
        </template>
        <div class="stat-content">
          <v-chart class="chart" :option="dailyTasksOption" />
        </div>
      </el-card>

      <el-card class="stat-card">
        <template #header>
          <div class="card-header">
            <el-icon><document-add /></el-icon>
            <span>每日创建任务数</span>
          </div>
        </template>
        <div class="stat-content">
          <v-chart class="chart" :option="dailyCreatedOption" />
        </div>
      </el-card>
    </div>

    <!-- 甘特图区域 -->
    <el-card class="gantt-card">
      <template #header>
        <div class="card-header">
          <el-icon><data-analysis /></el-icon>
          <span>任务处理记录甘特图</span>
        </div>
      </template>
      <div class="gantt-content">
        <v-chart class="gantt-chart" :option="ganttOption" />
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { use } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { LineChart, BarChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent
} from 'echarts/components'
import VChart from 'vue-echarts'
import { Clock, TrendCharts, DocumentAdd, DataAnalysis } from '@element-plus/icons-vue'

use([
  CanvasRenderer,
  LineChart,
  BarChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent
])

interface Task {
  id: number
  title: string
  duration: number
  progress: number
}

const longestTasks = ref<Task[]>([
  { id: 1, title: '项目架构设计', duration: 15, progress: 80 },
  { id: 2, title: '前端页面开发', duration: 12, progress: 60 },
  { id: 3, title: '后端API开发', duration: 10, progress: 40 },
  { id: 4, title: '数据库设计', duration: 8, progress: 100 },
  { id: 5, title: '测试与部署', duration: 6, progress: 20 }
])

const dailyTasksOption = ref({
  xAxis: {
    type: 'category',
    data: ['周一', '周二', '周三', '周四', '周五', '周六', '周日']
  },
  yAxis: {
    type: 'value'
  },
  series: [{
    data: [12, 8, 15, 10, 18, 5, 9],
    type: 'line',
    smooth: true,
    areaStyle: {}
  }],
  grid: {
    left: '3%',
    right: '4%',
    bottom: '3%',
    containLabel: true
  }
})

const dailyCreatedOption = ref({
  xAxis: {
    type: 'category',
    data: ['周一', '周二', '周三', '周四', '周五', '周六', '周日']
  },
  yAxis: {
    type: 'value'
  },
  series: [{
    data: [5, 3, 8, 6, 10, 2, 4],
    type: 'bar',
    itemStyle: {
      color: '#409eff'
    }
  }],
  grid: {
    left: '3%',
    right: '4%',
    bottom: '3%',
    containLabel: true
  }
})

const ganttOption = ref({
  tooltip: {
    formatter: function(params: any) {
      return `${params.marker} ${params.name}<br/>
              开始: ${params.data[3]}<br/>
              结束: ${params.data[4]}<br/>
              进度: ${params.data[5]}%`
    }
  },
  xAxis: {
    type: 'time'
  },
  yAxis: {
    type: 'category',
    data: ['项目架构', '前端开发', '后端开发', '数据库', '测试部署']
  },
  series: [{
    type: 'custom',
    renderItem: function(params: any, api: any) {
      const categoryIndex = api.value(0)
      const start = api.coord([api.value(1), categoryIndex])
      const end = api.coord([api.value(2), categoryIndex])
      const height = api.size([0, 1])[1] * 0.6
      
      return {
        type: 'rect',
        shape: {
          x: start[0],
          y: start[1] - height / 2,
          width: end[0] - start[0],
          height: height
        },
        style: api.style()
      }
    },
    data: [
      ['项目架构', '2024-01-01', '2024-01-15', '2024-01-01', '2024-01-15', 80],
      ['前端开发', '2024-01-10', '2024-01-22', '2024-01-10', '2024-01-22', 60],
      ['后端开发', '2024-01-12', '2024-01-25', '2024-01-12', '2024-01-25', 40],
      ['数据库', '2024-01-05', '2024-01-12', '2024-01-05', '2024-01-12', 100],
      ['测试部署', '2024-01-20', '2024-01-26', '2024-01-20', '2024-01-26', 20]
    ]
  }]
})

onMounted(() => {
  // 初始化数据
  console.log('Home component mounted')
})
</script>

<style scoped>
.home-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 24px;
}

.stat-card {
  height: 300px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
}

.stat-content {
  height: 240px;
  display: flex;
  flex-direction: column;
}

.task-item {
  margin-bottom: 12px;
}

.task-info {
  display: flex;
  justify-content: space-between;
  margin-bottom: 4px;
}

.task-title {
  font-size: 14px;
  color: #606266;
}

.task-duration {
  font-size: 12px;
  color: #909399;
}

.chart {
  width: 100%;
  height: 100%;
}

.gantt-card {
  flex: 1;
}

.gantt-content {
  height: 400px;
}

.gantt-chart {
  width: 100%;
  height: 100%;
}
</style>