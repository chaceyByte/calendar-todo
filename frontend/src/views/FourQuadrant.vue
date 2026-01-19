<script setup lang="ts">
import { ref, onMounted, nextTick, onUnmounted } from 'vue'
import { useTaskStore } from '@/stores/task'
import { useTagStore } from '@/stores/tag'
import type { Task } from '@/api/task'
import * as echarts from 'echarts'
import * as THREE from 'three'

// 任务状态类型
type PriorityLevel = '-high' | '-middle' | '-low' | 'low' | 'middle' | 'high'
type UrgencyLevel = '-high' | '-middle' | '-low' | 'low' | 'middle' | 'high'
type TaskStatus = 'planning' | 'in-progress' | 'completed' | 'cancelled'

interface QuadrantTask extends Task {
  x: number
  y: number
  z: number
  radius: number
  sphereSize: number
  isDragging: boolean
  quadrant: number
  sphere?: THREE.Mesh
  sprite?: THREE.Sprite
}

interface Tag {
  id: number
  name: string
  color: string
}

// 四象限配置（仅用于参考颜色，实际定位基于紧急/重要字段的正负值）
const QUADRANTS = [
  { name: '重要', color: '#ff4d4f' },
  { name: '不紧急', color: '#faad14' },
  { name: '紧急', color: '#52c41a' },
  { name: '不重要', color: '#1890ff' }
]

// 商店
const taskStore = useTaskStore()
const tagStore = useTagStore()

// 响应式数据
const chartRef = ref<HTMLElement>()
const threeContainerRef = ref<HTMLElement>()
const tasks = ref<QuadrantTask[]>([])
const selectedTask = ref<QuadrantTask | null>(null)
const tags = ref<Tag[]>([])
const draggingTag = ref<Tag | null>(null)
const scale = ref(1)
const isArchivedView = ref(false)

// Three.js相关变量
let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let raycaster: THREE.Raycaster
let mouse: THREE.Vector2
let isDragging = false
let draggedObject: THREE.Group | null = null
let dragPlane: THREE.Plane
let dragOffset = new THREE.Vector3()

// ECharts相关变量
let chart: any

// 缩放控制
const zoomIn = () => {
  scale.value = Math.min(scale.value + 0.1, 2)
}

const zoomOut = () => {
  scale.value = Math.max(scale.value - 0.1, 0.5)
}

// 初始化Three.js场景
const initThreeScene = () => {
  if (!threeContainerRef.value) return

  // 创建场景
  scene = new THREE.Scene()
  scene.background = new THREE.Color(0xf8f9fa)

  // 创建正交相机以避免透视变形
  const aspect = window.innerWidth / (window.innerHeight - 64)
  const viewHeight = 20 // 垂直可见范围（增大以容纳所有气球）
  const viewWidth = viewHeight * aspect
  camera = new THREE.OrthographicCamera(
    -viewWidth / 2, viewWidth / 2, // left, right
    viewHeight / 2, -viewHeight / 2, // top, bottom
    0.1, 1000
  )
  camera.position.set(0, 0, 10)
  camera.lookAt(0, 0, 0)

  // 创建渲染器
  renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true })
  renderer.setSize(window.innerWidth, window.innerHeight - 64)
  renderer.setPixelRatio(window.devicePixelRatio)
  threeContainerRef.value.appendChild(renderer.domElement)

  // 添加环境光
  const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
  scene.add(ambientLight)

  // 添加方向光
  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8)
  directionalLight.position.set(5, 5, 5)
  scene.add(directionalLight)

  // 初始化射线投射器和鼠标位置
  raycaster = new THREE.Raycaster()
  mouse = new THREE.Vector2()

  // 创建拖动平面（XY平面，z=0）
  dragPlane = new THREE.Plane(new THREE.Vector3(0, 0, 1), 0)

  // 动画循环
  const animate = () => {
    requestAnimationFrame(animate)
    
    // 让气球缓慢浮动（轻微上下运动）
    tasks.value.forEach(task => {
      if (task.sphere && !task.sphere.userData.isDragging) {
        // 轻微上下浮动效果
        task.sphere.position.y += Math.sin(Date.now() * 0.001 + task.id!) * 0.002
        
        // 不再旋转气球组，保持文字标签静止
      }
    })
    
    renderer.render(scene, camera)
  }
  animate()

  // 窗口大小调整（正交相机）
  const handleResize = () => {
    const aspect = window.innerWidth / (window.innerHeight - 64)
    const viewHeight = 20 // 与初始化时的值保持一致
    const viewWidth = viewHeight * aspect
    camera.left = -viewWidth / 2
    camera.right = viewWidth / 2
    camera.top = viewHeight / 2
    camera.bottom = -viewHeight / 2
    camera.updateProjectionMatrix()
    renderer.setSize(window.innerWidth, window.innerHeight - 64)

    // 调整echarts大小
    if (chart) {
      chart.resize()
    }
  }
  window.addEventListener('resize', handleResize)

  // 鼠标事件监听
  const handleMouseMove = (event: MouseEvent) => {
    mouse.x = (event.clientX / window.innerWidth) * 2 - 1
    mouse.y = -(event.clientY / (window.innerHeight - 64)) * 2 + 1
    
    // 拖动处理
    if (isDragging && draggedObject) {
      raycaster.setFromCamera(mouse, camera)
      const intersectionPoint = new THREE.Vector3()
      raycaster.ray.intersectPlane(dragPlane, intersectionPoint)
      
      if (intersectionPoint) {
        draggedObject.position.copy(intersectionPoint.add(dragOffset))
        
        // 限制拖动范围（避免飞出视图）
        const maxX = 10, maxY = 10
        draggedObject.position.x = Math.max(-maxX, Math.min(maxX, draggedObject.position.x))
        draggedObject.position.y = Math.max(-maxY, Math.min(maxY, draggedObject.position.y))
      }
      return
    }
    
    // 悬停检测（非拖动状态）
    const interactableObjects = getInteractableObjects()
    if (interactableObjects.length > 0) {
      raycaster.setFromCamera(mouse, camera)
      const intersects = raycaster.intersectObjects(interactableObjects, true)
      
      if (intersects.length > 0) {
        document.body.style.cursor = 'grab'
      } else {
        document.body.style.cursor = 'default'
      }
    }
  }
  window.addEventListener('mousemove', handleMouseMove)

  // 获取所有可交互的气球对象
  const getInteractableObjects = (): THREE.Object3D[] => {
    const interactableObjects: THREE.Object3D[] = []
    scene.children.forEach(child => {
      if (child.userData.isBalloonGroup) {
        // 添加气球组的所有子对象
        interactableObjects.push(...child.children)
      }
    })
    return interactableObjects
  }

  // 查找气球组（从任意子对象向上查找）
  const findBalloonGroup = (object: THREE.Object3D): THREE.Group | null => {
    let current = object
    while (current) {
      if (current.userData.isBalloonGroup) {
        return current as THREE.Group
      }
      current = current.parent
    }
    return null
  }

  const handleMouseDown = (event: MouseEvent) => {
    mouse.x = (event.clientX / window.innerWidth) * 2 - 1
    mouse.y = -(event.clientY / (window.innerHeight - 64)) * 2 + 1
    
    raycaster.setFromCamera(mouse, camera)
    
    // 只检测可交互的对象（气球组的子对象）
    const interactableObjects = getInteractableObjects()
    if (interactableObjects.length === 0) return
    
    const intersects = raycaster.intersectObjects(interactableObjects, true)
    
    if (intersects.length > 0) {
      const clickedObject = intersects[0].object
      
      // 向上查找对应的气球组
      const balloonGroup = findBalloonGroup(clickedObject)
      
      if (balloonGroup && balloonGroup.userData.taskId) {
        // 开始拖动
        isDragging = true
        draggedObject = balloonGroup
        draggedObject.userData.isDragging = true
        document.body.style.cursor = 'grabbing' // 拖动时光标
        
        // 计算拖动偏移量
        raycaster.setFromCamera(mouse, camera)
        const intersectionPoint = new THREE.Vector3()
        raycaster.ray.intersectPlane(dragPlane, intersectionPoint)
        
        if (intersectionPoint) {
          dragOffset.copy(draggedObject.position).sub(intersectionPoint)
        }
        
        // 查找对应的任务
        const task = tasks.value.find(t => t.id === balloonGroup.userData.taskId)
        if (task) {
          selectedTask.value = task
        }
      }
    }
  }

  const handleMouseUp = () => {
    if (isDragging && draggedObject) {
      draggedObject.userData.isDragging = false
      
      // 更新任务的紧急重要程度
      updateTaskPriorityFromPosition(draggedObject)
      document.body.style.cursor = 'grab' // 恢复为抓取光标
    }
    
    isDragging = false
    draggedObject = null
  }

  window.addEventListener('mousedown', handleMouseDown)
  window.addEventListener('mouseup', handleMouseUp)

  // 防止默认拖动行为
  window.addEventListener('dragstart', (e) => e.preventDefault())
}

// 创建气球组（气球+标签+绳子）
const createBalloonGroup = (task: QuadrantTask, position: THREE.Vector3) => {
  // 根据紧急程度和重要程度计算气球大小
  const priorityScore = getPriorityScore(task.priority)
  const urgencyScore = getUrgencyScore(task.urgency)
  
  // 基础大小 + 重要程度影响 + 紧急程度影响
  const baseSize = 0.4
  const priorityFactor = Math.abs(priorityScore) * 0.15
  const urgencyFactor = Math.abs(urgencyScore) * 0.1
  const balloonSize = baseSize + priorityFactor + urgencyFactor
  
  task.sphereSize = balloonSize

  // 创建气球组
  const balloonGroup = new THREE.Group()
  balloonGroup.position.copy(position)
  balloonGroup.userData = { 
    taskId: task.id,
    isDragging: false,
    dragOffset: new THREE.Vector3(),
    isBalloonGroup: true // 标记这是一个气球组
  }

  // 根据象限获取颜色
  const quadrantIndex = getTaskQuadrant(task)
  const quadrantColor = QUADRANTS[quadrantIndex].color
  const color = new THREE.Color(quadrantColor)
  
  // 创建气球（垂直椭圆体，直立气球形状）
  const balloonGeometry = new THREE.SphereGeometry(balloonSize, 32, 32)
  // 拉伸球体成为垂直椭圆形气球（y轴拉伸）
  balloonGeometry.scale(0.8, 0.9, 0.8)
  const balloonMaterial = new THREE.MeshPhongMaterial({ 
    color: color,
    transparent: true,
    opacity: 0.9,
    shininess: 60
  })
  const balloon = new THREE.Mesh(balloonGeometry, balloonMaterial)
  balloon.position.y = balloonSize * 2 // 气球在绳子顶部
  
  // 气球保持直立状态
  // balloon.rotation.z = 0 // 移除倾斜，保持直立
  
  // 添加气球高光效果（垂直椭圆形高光）
  const highlightGeometry = new THREE.SphereGeometry(balloonSize * 0.25, 16, 16)
  highlightGeometry.scale(0.8, 1.1, 0.8)
  const highlightMaterial = new THREE.MeshBasicMaterial({ 
    color: 0xffffff, 
    transparent: true,
    opacity: 0.4
  })
  const highlight = new THREE.Mesh(highlightGeometry, highlightMaterial)
  highlight.position.set(balloonSize * 0.4, balloonSize * 0.3, balloonSize * 0.4)
  balloon.add(highlight)
  
  // 添加气球底部结（气球打结处）
  const knotGeometry = new THREE.SphereGeometry(balloonSize * 0.1, 8, 8)
  const knotMaterial = new THREE.MeshBasicMaterial({ color: 0x333333 })
  const knot = new THREE.Mesh(knotGeometry, knotMaterial)
  knot.position.y = -balloonSize * 0.9 // 在气球底部
  balloon.add(knot)
  
  // 创建绳子
  const ropeGeometry = new THREE.CylinderGeometry(0.008, 0.008, balloonSize * 3, 8)
  const ropeMaterial = new THREE.MeshBasicMaterial({ color: 0x666666 })
  const rope = new THREE.Mesh(ropeGeometry, ropeMaterial)
  rope.position.y = balloonSize * 0.5 // 绳子从气球底部延伸
  
  // 创建标签板（垂直平面，适合竖向文字）
  const labelGeometry = new THREE.PlaneGeometry(balloonSize * 1.5, balloonSize * 1.5) // 增大以匹配更大的文字
  const labelMaterial = new THREE.MeshBasicMaterial({
    color: 0xffffff,
    transparent: true,
    opacity: 0.1,
    side: THREE.DoubleSide
  })
  const label = new THREE.Mesh(labelGeometry, labelMaterial)
  label.position.y = -balloonSize * 1.5 // 标签在绳子底部

  // 标签板保持垂直平面，与绳子平行
  // label.rotation.x = 0 // 不旋转，保持垂直

  // 创建标签边框
  const borderGeometry = new THREE.PlaneGeometry(balloonSize * 1.15, balloonSize * 4.55) // 增大以匹配标签板
  const borderMaterial = new THREE.MeshBasicMaterial({ 
    color: 0xcccccc,
    transparent: true,
    opacity: 0.8,
    side: THREE.DoubleSide
  })
  const border = new THREE.Mesh(borderGeometry, borderMaterial)
  border.position.z = -0.01 // 边框在标签后面
  label.add(border)
  
  // 创建文字精灵显示任务标题
  createTextSprite(task, label, balloonSize)
  
  // 将气球、绳子、标签添加到组中
  balloonGroup.add(balloon)
  balloonGroup.add(rope)
  balloonGroup.add(label)
  
  // 添加透明点击检测球体（扩大点击区域，但不可见）
  const hitboxGeometry = new THREE.SphereGeometry(balloonSize * 1.5, 16, 16)
  const hitboxMaterial = new THREE.MeshBasicMaterial({ 
    color: color,
    transparent: true,
    opacity: 0, // 完全透明，只用于点击检测
    side: THREE.DoubleSide
  })
  const hitbox = new THREE.Mesh(hitboxGeometry, hitboxMaterial)
  hitbox.position.y = balloonSize * 1.5 // 在气球位置
  hitbox.userData.isHitbox = true // 标记为点击检测框
  balloonGroup.add(hitbox)
  
  scene.add(balloonGroup)
  task.sphere = balloonGroup
  
  return balloonGroup
}

// 创建文字精灵（竖向排列）
const createTextSprite = (task: QuadrantTask, label: THREE.Mesh, balloonSize: number) => {
  const canvas = document.createElement('canvas')
  const context = canvas.getContext('2d')!
  
  // 设置画布大小（竖向，高度大于宽度）- 增大尺寸以容纳更大文字
  const canvasWidth = 160 // 较窄，增大以容纳更大文字
  const canvasHeight = 480 // 较高，增大以容纳更大文字
  canvas.width = canvasWidth
  canvas.height = canvasHeight
  
  // 填充透明背景
  context.clearRect(0, 0, canvasWidth, canvasHeight)
  
  // 绘制浅色半透明背景（矩形）
  const padding = 10
  const bgWidth = canvasWidth - padding * 2
  const bgHeight = canvasHeight - padding * 2
  context.fillStyle = 'rgba(255, 255, 255, 0.9)'
  context.fillRect(padding, padding, bgWidth, bgHeight)
  
  // 可选：添加边框
  context.strokeStyle = 'rgba(200, 200, 200, 0.7)'
  context.lineWidth = 1
  context.strokeRect(padding, padding, bgWidth, bgHeight)
  
  // 设置文字样式 - 增大字体，加深颜色，添加阴影
  context.font = 'bold 36px Arial' // 从26px增大到36px
  context.fillStyle = '#000000'
  context.textAlign = 'center'
  context.textBaseline = 'middle'
  context.shadowColor = 'rgba(255, 255, 255, 0.8)'
  context.shadowBlur = 4
  
  // 处理长标题 - 字体变大后可以显示更多字符
  let displayText = task.title
  if (task.title.length > 10) {
    displayText = task.title.substring(0, 10) + '...'
  }

  // 文字竖向显示，从上到下自然排列（逐个字符绘制）
  context.save()
  const fontSize = 36
  const lineHeight = fontSize * 1.2
  const startY = (canvasHeight - displayText.length * lineHeight) / 2 + lineHeight / 2

  for (let i = 0; i < displayText.length; i++) {
    const char = displayText[i]
    context.fillText(char, canvasWidth / 2, startY + i * lineHeight)
  }
  context.restore()
  
  // 创建纹理
  const texture = new THREE.CanvasTexture(canvas)
  
  // 创建精灵材质
  const spriteMaterial = new THREE.SpriteMaterial({ 
    map: texture,
    transparent: true
  })
  
  // 创建精灵
  const sprite = new THREE.Sprite(spriteMaterial)
  sprite.scale.set(balloonSize * 1.5, balloonSize * 4.5, 1) // 增大精灵尺寸以匹配更大的文字
  sprite.position.z = 0.01 // 文字在标签前面
  
  label.add(sprite)
  task.sprite = sprite
}

// 初始化饼图
const initChart = () => {
  if (!chartRef.value) return

  chart = echarts.init(chartRef.value)

  // 确保即使没有数据也有占位圆环可见
  const getSafeData = (count: number) => count > 0 ? count : 0.1 // 最小值0.1确保渲染

  // 计算饼图大小，使用更大的百分比确保可见
  const option = {
    backgroundColor: 'transparent',
    series: [
      // 外层圆环 - 任务状态分布
      {
        type: 'pie',
        radius: ['55%', '60%'], // 增大半径，确保可见
        center: ['50%', '50%'],
        data: [
          { value: getSafeData(tasks.value.filter(t => t.status === 'planning').length), name: '计划中', itemStyle: { color: 'rgba(24, 144, 255, 0.6)' } },
          { value: getSafeData(tasks.value.filter(t => t.status === 'in-progress').length), name: '进行中', itemStyle: { color: 'rgba(250, 173, 20, 0.6)' } },
          { value: 0, name: '已完成', itemStyle: { color: 'rgba(82, 196, 26, 0.6)' } }, // 四象限不显示已完成
          { value: 0, name: '已取消', itemStyle: { color: 'rgba(217, 217, 217, 0.6)' } }  // 四象限不显示已取消
        ],
        label: { show: false },
        itemStyle: {
          borderColor: 'rgba(255, 255, 255, 0.3)',
          borderWidth: 2
        },
        silent: true // 禁止交互
      },
      // 中层圆环 - 紧急程度分布
      {
        type: 'pie',
        radius: ['42%', '48%'], // 增大半径
        center: ['50%', '50%'],
        data: [
          { value: getSafeData(tasks.value.filter(t => t.urgency === 'high' || t.urgency === 'middle').length), name: '紧急', itemStyle: { color: 'rgba(255, 77, 79, 0.7)' } },
          { value: getSafeData(tasks.value.filter(t => t.urgency === 'low' || t.urgency === '-low').length), name: '不紧急', itemStyle: { color: 'rgba(82, 196, 26, 0.7)' } },
          { value: getSafeData(tasks.value.filter(t => t.urgency === 'low' || t.urgency === '-middle' || t.urgency === '-high').length), name: '一般', itemStyle: { color: 'rgba(250, 173, 20, 0.7)' } }
        ],
        label: { show: false },
        itemStyle: {
          borderColor: 'rgba(255, 255, 255, 0.4)',
          borderWidth: 2
        },
        silent: true
      },
      // 内层圆环 - 重要程度分布
      {
        type: 'pie',
        radius: ['30%', '36%'], // 增大半径
        center: ['50%', '50%'],
        data: [
          { value: getSafeData(tasks.value.filter(t => t.priority === 'high' || t.priority === 'middle').length), name: '重要', itemStyle: { color: 'rgba(255, 77, 79, 0.8)' } },
          { value: getSafeData(tasks.value.filter(t => t.priority === 'low' || t.priority === '-low').length), name: '不重要', itemStyle: { color: 'rgba(82, 196, 26, 0.8)' } },
          { value: getSafeData(tasks.value.filter(t => t.priority === 'low' || t.priority === '-middle' || t.priority === '-high').length), name: '一般', itemStyle: { color: 'rgba(250, 173, 20, 0.8)' } }
        ],
        label: { show: false },
        itemStyle: {
          borderColor: 'rgba(255, 255, 255, 0.5)',
          borderWidth: 2
        },
        emphasis: {
          itemStyle: {
            shadowBlur: 15,
            shadowOffsetX: 0,
            shadowColor: 'rgba(0, 0, 0, 0.3)'
          }
        },
        silent: true
      }
    ],
    graphic: {
      type: 'text',
      left: 'center',
      top: 'center',
      style: {
        text: '归档',
        fontSize: 14,
        fontWeight: 'bold',
        fill: '#666',
        cursor: 'pointer'
      },
      onclick: () => {
        showArchivedView()
      }
    }
  }

  chart.setOption(option)

  // 点击中心进入归档视图
  chart.on('click', (params: any) => {
    if (params.componentType === 'series' && params.seriesIndex === 2) {
      showArchivedView()
    }
  })
}

// 显示归档视图
const showArchivedView = () => {
  isArchivedView.value = true
  // 这里可以添加进入动画
}

// 隐藏归档视图
const hideArchivedView = () => {
  isArchivedView.value = false
}

// 根据气球位置更新任务紧急重要程度
const updateTaskPriorityFromPosition = async (balloonGroup: THREE.Group) => {
  const task = tasks.value.find(t => t.id === balloonGroup.userData.taskId)
  if (!task) return

  const position = balloonGroup.position
  
  // 计算象限（基于位置）
  let priority: PriorityLevel
  let urgency: UrgencyLevel
  
  // x轴：紧急程度（正值=右=紧急，负值=左=不紧急）
  if (position.x > 2) {
    urgency = 'high'
  } else if (position.x < -2) {
    urgency = '-high'
  } else {
    urgency = 'middle'
  }
  
  // y轴：重要程度（正值=上=重要，负值=下=不重要）
  if (position.y > 2) {
    priority = 'high'
  } else if (position.y < -2) {
    priority = '-high'
  } else {
    priority = 'middle'
  }
  
  try {
    await taskStore.updateTask(task.id!, { priority, urgency })
    task.priority = priority
    task.urgency = urgency
  } catch (error) {
    console.error('更新任务重要紧急程度失败:', error)
  }
}



// 自动排布任务位置 - 在3D空间中根据象限分布球体
const arrangeTasks = () => {
  // 清除场景中现有的球体
  if (scene) {
    tasks.value.forEach(task => {
      if (task.sphere) {
        scene.remove(task.sphere)
      }
    })
  }

  // 将任务按象限分组
  const quadrantTasks = [[], [], [], []] as QuadrantTask[][]
  
  tasks.value.forEach(task => {
    const quadrantIndex = getTaskQuadrant(task)
    quadrantTasks[quadrantIndex].push(task)
  })
  
  // 3D空间中的布局参数
  const quadrantRadius = 6 // 每个象限的半径（减小以确保在相机可视范围内）
  const sphereSpacing = 1.5 // 球体之间的最小间距
  const zOffsetRange = 3 // Z轴偏移范围
  
  // 为每个象限计算中心点（在3D空间中）
  const quadrantCenters = [
    new THREE.Vector3(quadrantRadius, quadrantRadius, 0),      // 第一象限：右上（紧急重要）
    new THREE.Vector3(-quadrantRadius, quadrantRadius, 0),     // 第二象限：左上（不紧急重要）
    new THREE.Vector3(-quadrantRadius, -quadrantRadius, 0),    // 第三象限：左下（不紧急不重要）
    new THREE.Vector3(quadrantRadius, -quadrantRadius, 0)      // 第四象限：右下（紧急不重要）
  ]
  
  quadrantTasks.forEach((tasksInQuadrant, quadrantIndex) => {
    const center = quadrantCenters[quadrantIndex]
    
    // 计算球体在象限内的分布
    const sphereCount = tasksInQuadrant.length
    if (sphereCount === 0) return
    
    // 计算环形布局参数
    const ringRadius = Math.min(quadrantRadius * 0.6, sphereCount * sphereSpacing / (2 * Math.PI))
    
    tasksInQuadrant.forEach((task, index) => {
      // 计算角度（环形分布）
      const angle = (index / sphereCount) * 2 * Math.PI
      
      // 计算位置（在象限中心周围环形分布）
      const offsetX = Math.cos(angle) * ringRadius
      const offsetY = Math.sin(angle) * ringRadius
      const offsetZ = (Math.random() - 0.5) * zOffsetRange // 随机Z轴偏移
      
      const position = new THREE.Vector3(
        center.x + offsetX,
        center.y + offsetY,
        center.z + offsetZ
      )
      
      // 创建3D气球组
      createBalloonGroup(task, position)
      
      // 保留2D位置信息（用于UI显示）
      task.x = window.innerWidth / 2 + position.x * 50
      task.y = window.innerHeight / 2 - position.y * 50
      task.z = position.z
      
      // 根据重要程度调整2D显示大小
      task.radius = Math.max(30, Math.min(60, 35 + Math.abs(getPriorityScore(task.priority)) * 8))
    })
  })
}

// 获取任务所在象限（x轴=紧急程度，y轴=重要程度）
const getTaskQuadrant = (task: QuadrantTask): number => {
  const urgencyScore = getUrgencyScore(task.urgency)   // x轴：紧急程度（正值=紧急，负值=不紧急）
  const priorityScore = getPriorityScore(task.priority) // y轴：重要程度（正值=重要，负值=不重要）
  
  // 第一象限：x>0, y>0（右上）：紧急且重要
  if (urgencyScore > 0 && priorityScore > 0) return 0
  // 第二象限：x<0, y>0（左上）：不紧急但重要
  if (urgencyScore <= 0 && priorityScore > 0) return 1
  // 第三象限：x<0, y<0（左下）：不紧急且不重要
  if (urgencyScore <= 0 && priorityScore <= 0) return 2
  // 第四象限：x>0, y<0（右下）：紧急但不重要
  return 3
}

const getPriorityScore = (priority: PriorityLevel): number => {
  const scores = { '-high': -3, '-middle': -2, '-low': -1, 'low': 0, 'middle': 1, 'high': 2 }
  return scores[priority] || 0
}

const getUrgencyScore = (urgency: UrgencyLevel): number => {
  const scores = { '-high': -3, '-middle': -2, '-low': -1, 'low': 0, 'middle': 1, 'high': 2 }
  return scores[urgency] || 0
}

// 双击创建任务（基于象限的紧急/重要程度）
const createTaskInQuadrant = (quadrantIndex: number) => {
  const quadrant = QUADRANTS[quadrantIndex]
  let priority: PriorityLevel
  let urgency: UrgencyLevel
  
  // 根据象限索引确定紧急和重要程度（x轴=紧急，y轴=重要）
  switch(quadrantIndex) {
    case 0: // 第一象限：右上（x>0, y>0）
      priority = 'high'
      urgency = 'high'
      break
    case 1: // 第二象限：左上（x<0, y>0）
      priority = 'high'
      urgency = '-high'
      break
    case 2: // 第三象限：左下（x<0, y<0）
      priority = '-high'
      urgency = '-high'
      break
    case 3: // 第四象限：右下（x>0, y<0）
      priority = '-high'
      urgency = 'high'
      break
    default:
      priority = 'middle'
      urgency = 'middle'
  }
  
  const newTask: Partial<Task> = {
    title: `新任务-${quadrant.name}`,
    status: 'planning' as TaskStatus,
    priority,
    urgency,
    progress: 0
  }
  
  // 这里调用创建任务API
  console.log('创建新任务:', newTask)
  // taskStore.addTask(newTask)
}

// 双击空白处隐藏视图
const hideAllViews = () => {
  selectedTask.value = null
}



// 根据背景色获取对比色（确保文字可读性）
const getContrastColor = (hexColor: string): string => {
  // 移除#号
  const hex = hexColor.replace('#', '')
  
  // 转换为RGB
  const r = parseInt(hex.substr(0, 2), 16)
  const g = parseInt(hex.substr(2, 2), 16)
  const b = parseInt(hex.substr(4, 2), 16)
  
  // 计算亮度（YIQ公式）
  const brightness = (r * 299 + g * 587 + b * 114) / 1000
  
  // 根据亮度返回黑色或白色
  return brightness >= 128 ? '#000000' : '#ffffff'
}

// 标签拖拽
const onTagDragStart = (tag: Tag, event: DragEvent) => {
  draggingTag.value = tag
  event.dataTransfer?.setData('text/plain', tag.id.toString())
}

const onTaskTagDrop = async (task: QuadrantTask, event: DragEvent) => {
  event.preventDefault()
  const tagId = event.dataTransfer?.getData('text/plain')
  if (tagId && draggingTag.value) {
    try {
      await taskStore.updateTaskTags(task.id!, [parseInt(tagId)])
      if (!task.tags) task.tags = []
      task.tags.push(draggingTag.value)
    } catch (error) {
      console.error('添加标签失败:', error)
    }
  }
  draggingTag.value = null
}

  // 清理Three.js资源
const cleanupThree = () => {
  if (renderer) {
    renderer.dispose()
  }
  if (scene) {
    scene.clear()
  }

  // 清理echarts实例
  if (chart) {
    chart.dispose()
  }

  // 移除事件监听器
  window.removeEventListener('resize', () => {})
  window.removeEventListener('mousemove', () => {})
  window.removeEventListener('mousedown', () => {})
  window.removeEventListener('mouseup', () => {})
  window.removeEventListener('dragstart', () => {})
}

// 初始化数据
onMounted(async () => {
  await Promise.all([
    taskStore.fetchTasks(),
    tagStore.fetchTags()
  ])
  
  // 初始化任务数据，只显示非完成状态的任务（计划中、进行中、暂停）
  tasks.value = taskStore.tasks
    .filter(task => task.status !== 'completed' && task.status !== 'cancelled')
    .map(task => ({
      ...task,
      x: window.innerWidth / 2, // 初始位置在屏幕中心
      y: window.innerHeight / 2,
      z: 0,
      radius: 30,
      sphereSize: 0.5,
      isDragging: false,
      quadrant: 0
    }))
  
  tags.value = tagStore.tags as Tag[]
  
  // 初始化Three.js场景
  nextTick(() => {
    initThreeScene()
    // 排布任务位置（创建3D球体）
    arrangeTasks()
    // 初始化图表
    initChart()
  })
})

// 组件卸载时清理资源
onUnmounted(() => {
  cleanupThree()
})
</script>

<template>
  <div class="four-quadrant-container" @dblclick="hideAllViews">
    <!-- 饼状图 - 移到quadrant-area外面，作为背景 -->
    <div ref="chartRef" class="pie-chart"></div>

    <!-- 四象限区域 -->
    <div class="quadrant-area" :style="{ transform: `scale(${scale})`, top: '64px', height: 'calc(100vh - 64px)' }">
      <!-- Three.js渲染容器 -->
      <div ref="threeContainerRef" class="three-container"></div>

      <!-- 坐标轴 -->
      <div class="axis-lines">
        <div class="axis-x" :style="{ top: '50%' }"></div>
        <div class="axis-y"></div>
      </div>
      
      <!-- 四象限标签（已移除文字描述） -->
      <div class="quadrant-labels">
        <div 
          v-for="(quadrant, index) in QUADRANTS" 
          :key="index"
          class="quadrant-text"
          :class="`quadrant-text-${index}`"
          @dblclick="createTaskInQuadrant(index)"
        >
          <!-- 文字描述已移除 -->
          <span>{{quadrant.name }}</span>
        </div>
      </div>
    </div>

    <!-- 任务详情面板 -->
    <div v-if="selectedTask" class="task-detail-panel">
      <h3>{{ selectedTask.title }}</h3>
      <p>{{ selectedTask.description }}</p>
      <div class="task-info">
        <div>状态: {{ selectedTask.status }}</div>
        <div>重要程度: {{ selectedTask.priority }}</div>
        <div>紧急程度: {{ selectedTask.urgency }}</div>
      </div>
      
      <!-- 任务标签展示 -->
      <div class="task-tags-section">
        <h4>标签</h4>
        <div class="current-tags">
          <span 
            v-for="tag in selectedTask.tags" 
            :key="tag.id"
            class="task-tag-item"
            :style="{ backgroundColor: tag.color, color: getContrastColor(tag.color) }"
          >
            {{ tag.name }}
          </span>
          <span v-if="!selectedTask.tags || selectedTask.tags.length === 0" class="no-tags">暂无标签</span>
        </div>
      </div>
    </div>

    <!-- 左下角标签列表 -->
    <div class="tags-panel">
      <h3>标签</h3>
      <div class="tags-list">
        <div
          v-for="tag in tags"
          :key="tag.id"
          class="tag-item"
          draggable="true"
          @dragstart="onTagDragStart(tag, $event)"
          :style="{ backgroundColor: tag.color }"
        >
          {{ tag.name }}
        </div>
      </div>
    </div>

    <!-- 缩放控制 -->
    <div class="zoom-controls">
      <button @click="zoomIn">+</button>
      <button @click="zoomOut">-</button>
    </div>

    <!-- 归档视图 -->
    <div v-if="isArchivedView" class="archived-view">
      <div class="archived-header">
        <h2>归档任务</h2>
        <button @click="hideArchivedView">关闭</button>
      </div>
      <div class="archived-tasks">
        <!-- 归档任务列表 -->
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 特殊处理：四象限页面需要全屏显示，移除padding */
:global(.layout-container .main-content) {
  padding: 0 !important;
  overflow: hidden !important;
}

.four-quadrant-container {
  position: fixed;
  top: 64px; /* 在导航栏下方 */
  left: 0;
  width: 100vw;
  height: calc(100vh - 64px); /* 减去导航栏高度 */
  overflow: hidden !important;
  background: radial-gradient(circle at center, #ffffff 0%, #f8f9fa 100%);
  /* 隐藏滚动条 */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
  max-width: 100vw;
  max-height: 100vh;
  box-sizing: border-box;
  z-index: 1; /* 降低z-index，确保在导航栏下方 */
}

.four-quadrant-container::-webkit-scrollbar {
  display: none; /* Chrome, Safari, Opera */
}

.temporary-tasks-panel {
  position: fixed;
  top: 84px; /* 64px header + 20px margin */
  left: 20px;
  width: 220px;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 12px;
  padding: 16px;
  box-shadow:  0 4px 20px rgba(0,0,0,0.1);
  backdrop-filter: blur(10px);
  z-index: 1000;
  border: 1px solid rgba(255,255,255,0.3);
}

.temporary-tasks-list {
  max-height: 250px;
  overflow-y: auto;
}

.temporary-task {
  padding: 10px;
  margin: 6px 0;
  background: rgba(240, 240, 240, 0.7);
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.temporary-task:hover {
  background: rgba(224, 224, 224, 0.8);
  transform: translateX(2px);
}

.quadrant-area {
  position: fixed;
  top: 64px; /* 导航栏高度 */
  left: 0;
  width: 100vw;
  height: calc(100vh - 64px); /* 减去导航栏高度 */
  max-width: 100vw;
  max-height: calc(100vh - 64px);
  transform-origin: center;
  transition: transform 0.3s ease;
  overflow: hidden;
}

.three-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1;
  pointer-events: auto;
}

.pie-chart {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 80%; /* 稍微缩小，确保不被裁剪 */
  height: 80%;
  max-width: 100%;
  max-height: 100%;
  z-index: 0; /* 作为背景，但要在four-quadrant-container的背景之上 */
  opacity: 0.7; /* 增加不透明度，让饼图更明显 */
  pointer-events: none;
}

/* 坐标轴样式 */
.axis-lines {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  max-width: 100vw;
  max-height: 100vh;
  pointer-events: none;
  z-index: 2;
  overflow: hidden;
}

.axis-x {
  position: absolute;
  top: 50%;
  left: 0;
  width: 100vw;
  max-width: 100vw;
  height: 2px;
  background: linear-gradient(to right, transparent 0%, rgba(0,0,0,0.2) 5%, rgba(0,0,0,0.2) 95%, transparent 100%);
  transform: translateY(-50%);
}

.axis-y {
  position: absolute;
  top: 0;
  left: 50%;
  width: 2px;
  height: 100vh;
  max-height: 100vh;
  background: linear-gradient(to bottom, transparent 0%, rgba(0,0,0,0.2) 5%, rgba(0,0,0,0.2) 95%, transparent 100%);
  transform: translateX(-50%);
}

/* 象限标签样式 */
.quadrant-labels {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  max-width: 100vw;
  max-height: 100vh;
  pointer-events: none;
  z-index: 3;
  overflow: hidden;
}

.quadrant-text {
  position: absolute;
  font-size: 18px;
  font-weight: 700;
  color: rgba(0, 0, 0, 0.5);
  pointer-events: auto;
  cursor: pointer;
  transition: all 0.3s ease;
  padding: 8px 16px;
  border-radius: 8px;
  backdrop-filter: blur(5px);
  background: rgba(255,255,255,0.2);
  border: 1px solid rgba(255,255,255,0.3);
  white-space: nowrap;
}

.quadrant-text:hover {
  background: rgba(255,255,255,0.4);
  transform: scale(1.05);
}

.quadrant-text-0 {
  top: 60px; /* 避开顶部导航栏 */
  left: 50%;
  transform: translateX(-50%);
}

.quadrant-text-1 {
  top: 50%;
  left: 20px;
  transform: translateY(-50%);
}

.quadrant-text-2 {
  top: 50%;
  right: 20px;
  transform: translateY(-50%);
}

.quadrant-text-3 {
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
}

/* 任务层 */
.tasks-layer {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  max-width: 100vw;
  max-height: 100vh;
  z-index: 4;
  overflow: hidden;
  pointer-events: none;
}

.tasks-layer > * {
  pointer-events: auto;
}

.task-ball {
  position: absolute;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.9);
  border: 3px solid;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: grab;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 4px 20px rgba(0,0,0,0.15);
  backdrop-filter: blur(5px);
  z-index: 5;
  font-size: 11px;
  font-weight: 500;
  text-align: center;
  word-wrap: break-word;
  overflow: hidden;
  padding: 4px;
}

.task-ball:hover {
  transform: scale(1.15);
  box-shadow: 0 8px 30px rgba(0,0,0,0.2);
  z-index: 10;
}

.task-ball:active {
  cursor: grabbing;
  transform: scale(1.25);
  box-shadow: 0 12px 40px rgba(0,0,0,0.25);
}

.task-title {
  font-size: 11px;
  text-align: center;
  padding: 2px;
  word-break: break-word;
  line-height: 1.2;
  max-height: 100%;
  overflow: hidden;
}

.task-progress {
  position: absolute;
  bottom: 0;
  left: 0;
  height: 4px;
  background: #52c41a;
  border-radius: 0 0 50% 50%;
  transition: width 0.3s ease;
}

.task-tags {
  position: absolute;
  top: -6px;
  right: -6px;
  display: flex;
  gap: 3px;
  flex-wrap: wrap;
  max-width: 60px;
}

.task-tag {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  border: 1px solid white;
}

.task-detail-panel {
  position: fixed;
  top: 20px;
  right: 20px;
  width: 320px;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 8px 32px rgba(0,0,0,0.1);
  z-index: 1000;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255,255,255,0.3);
}

.task-detail-panel h3 {
  margin: 0 0 12px 0;
  font-size: 18px;
  color: #333;
}

.task-detail-panel p {
  margin: 0 0 16px 0;
  color: #666;
  font-size: 14px;
  line-height: 1.5;
}

.task-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.task-info div {
  font-size: 13px;
  color: #555;
}

.task-info div strong {
  color: #333;
}

/* 任务标签展示样式 */
.task-tags-section {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}

.task-tags-section h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  color: #333;
  font-weight: 600;
}

.current-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.task-tag-item {
  display: inline-flex;
  align-items: center;
  padding: 4px 8px;
  border-radius: 12px;
  color: white;
  font-size: 11px;
  font-weight: 500;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.no-tags {
  font-size: 12px;
  color: #999;
  font-style: italic;
}

.tags-panel {
  position: fixed;
  bottom: 20px;
  left: 20px;
  height: 300px;
  width: 400px;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 12px;
  padding: 16px;
  box-shadow: 0 8px 32px rgba(0,0,0,0.1);
  z-index: 1000;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255,255,255,0.3);
}

.tags-panel h3 {
  margin: 0 0 12px 0;
  font-size: 16px;
  color: #333;
}

.tags-list {
  max-height: 250px;
  overflow-y: auto;
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  radius: 50%;
}

.tag-item {
  padding: 6px 10px;
  border-radius: 20px;
  color: white;
  font-size: 12px;
  cursor: grab;
  font-weight: 500;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  transition: all 0.2s;
}

.tag-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
}

.tag-item:active {
  cursor: grabbing;
  transform: scale(0.95);
}

.zoom-controls {
  position: fixed;
  bottom: 20px;
  right: 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  z-index: 1000;
}

.zoom-controls button {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: rgba(24, 144, 255, 0.9);
  color: white;
  font-size: 20px;
  cursor: pointer;
  box-shadow: 0 4px 16px rgba(0,0,0,0.15);
  transition: all 0.2s;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(255,255,255,0.3);
  font-weight: bold;
}

.zoom-controls button:hover {
  transform: scale(1.1);
  background: rgba(24, 144, 255, 1);
  box-shadow: 0 6px 20px rgba(0,0,0,0.2);
}

.zoom-controls button:active {
  transform: scale(0.95);
}

.archived-view {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: white;
  z-index: 100;
  animation: slideIn 0.3s ease;
}

@keyframes slideIn {
  from { transform: translateY(100%); }
  to { transform: translateY(0); }
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .quadrant-text {
    font-size: 16px;
  }
}

@media (max-width: 768px) {
  .temporary-tasks-panel,
  .tags-panel,
  .task-detail-panel {
    width: 180px;
  }
  
  .quadrant-text {
    font-size: 14px;
    padding: 6px 12px;
  }
}
</style>