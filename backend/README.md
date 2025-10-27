# 任务日历后端项目

基于 Spring Boot 3 + MyBatis Plus + Spring AI 的任务日历后端服务。

## 功能特性

- 🔐 **用户认证**：JWT Token 认证机制
- 📊 **任务管理**：完整的任务 CRUD 操作
- 🏷️ **标签系统**：灵活的标签分类管理
- 📅 **日历功能**：任务时间线管理
- 🤖 **AI 集成**：基于 Spring AI 的智能功能
- 📈 **报告生成**：日报、周报自动生成

## 技术栈

- **框架**：Spring Boot 3.2.0
- **ORM**：MyBatis Plus 3.5.4
- **数据库**：MySQL 8.0
- **安全**：Spring Security + JWT
- **AI**：Spring AI (OpenAI)
- **工具**：Lombok、Validation

## 项目结构

```
src/main/java/com/taskcalendar/
├── entity/          # 实体类
│   ├── User.java    # 用户实体
│   ├── Task.java    # 任务实体
│   ├── Tag.java     # 标签实体
│   └── ...
├── dto/             # 数据传输对象
├── service/         # 业务逻辑层
├── controller/      # 控制器层
├── config/          # 配置类
└── TaskCalendarApplication.java # 启动类
```

## 数据库设计

### 核心表结构

#### users (用户表)
- id, username, password, nickname, avatar, email
- created_at, updated_at, deleted

#### tasks (任务表)
- id, title, description, status, progress, priority
- start_date, end_date, user_id
- created_at, updated_at, deleted

#### tags (标签表)
- id, name, color, user_id
- created_at, updated_at, deleted

#### task_tags (任务标签关联表)
- id, task_id, tag_id, created_at

#### reports (报告表)
- id, type, title, content, report_date, user_id
- created_at, updated_at, deleted

## API 接口设计

### 认证接口
- `POST /auth/login` - 用户登录
- `POST /auth/logout` - 用户登出
- `GET /auth/profile` - 获取用户信息

### 任务接口
- `GET /tasks` - 获取任务列表
- `POST /tasks` - 创建任务
- `PUT /tasks/{id}` - 更新任务
- `DELETE /tasks/{id}` - 删除任务
- `PUT /tasks/{id}/status` - 更新任务状态

### 日历接口
- `GET /calendar/month/{year}/{month}` - 获取月视图数据
- `GET /calendar/daily-report/{date}` - 生成日报
- `GET /calendar/weekly-report/{year}/{week}` - 生成周报

### 标签接口
- `GET /tags` - 获取标签列表
- `POST /tags` - 创建标签
- `PUT /tags/{id}` - 更新标签
- `DELETE /tags/{id}` - 删除标签

### AI 接口
- `POST /ai/polish-report` - 润色报告
- `POST /ai/task-suggestions` - 任务建议

## 快速开始

### 环境要求

- Java 17+
- Maven 3.6+
- MySQL 8.0+
- OpenAI API Key (可选)

### 数据库配置

1. 创建数据库
```sql
CREATE DATABASE task_calendar CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

2. 执行初始化脚本
```sql
source src/main/resources/schema.sql
```

### 配置修改

编辑 `src/main/resources/application.yml`：

```yaml
spring:
  datasource:
    url: jdbc:mysql://localhost:3306/task_calendar
    username: your-username
    password: your-password
  
  ai:
    openai:
      api-key: your-openai-api-key
```

### 启动应用

```bash
mvn spring-boot:run
```

访问 http://localhost:8080/api

## 测试数据

应用启动后会自动插入测试数据：

- 用户：admin / 123456
- 示例任务和标签

## 开发说明

### 实体类规范
- 使用 Lombok @Data 注解
- 使用 MyBatis Plus 注解
- 包含逻辑删除字段
- 包含创建/更新时间字段

### API 设计原则
- RESTful 风格
- 统一响应格式
- JWT Token 认证
- 参数验证

### 安全配置
- Spring Security 配置
- JWT Token 管理
- 密码加密存储
- 跨域配置

## 部署说明

### 打包应用

```bash
mvn clean package
```

### 运行 JAR 文件

```bash
java -jar target/task-calendar-backend-1.0.0.jar
```

### Docker 部署

```dockerfile
FROM openjdk:17-jdk-slim
COPY target/task-calendar-backend-1.0.0.jar app.jar
ENTRYPOINT ["java", "-jar", "/app.jar"]
```

## 注意事项

1. **OpenAI API Key**：AI 功能需要配置有效的 OpenAI API Key
2. **数据库版本**：建议使用 MySQL 8.0+ 版本
3. **时区配置**：确保数据库和应用的时区配置一致
4. **安全配置**：生产环境需要修改 JWT Secret 和数据库密码