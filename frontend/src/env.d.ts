/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

// Element Plus 类型扩展
declare module 'element-plus' {
  export interface ElMessage {
    (options: any): any
    success(message: string): any
    warning(message: string): any
    error(message: string): any
    info(message: string): any
  }
  
  export interface ElMessageBox {
    confirm(message: string, title: string, options?: any): Promise<any>
    alert(message: string, title: string, options?: any): Promise<any>
    prompt(message: string, title: string, options?: any): Promise<any>
  }
}

declare const ElMessage: import('element-plus').ElMessage
declare const ElMessageBox: import('element-plus').ElMessageBox