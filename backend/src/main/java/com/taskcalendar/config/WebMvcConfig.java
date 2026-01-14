package com.taskcalendar.config;

import com.taskcalendar.interceptor.AuthInterceptor;
import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.InterceptorRegistry;
import org.springframework.web.servlet.config.annotation.ResourceHandlerRegistry;
import org.springframework.web.servlet.config.annotation.ViewControllerRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

@Configuration
@RequiredArgsConstructor
public class WebMvcConfig implements WebMvcConfigurer {

    private final AuthInterceptor authInterceptor;

    @Value("${static.path:/Users/pengjuyan/Documents/WORK/codebuddy/calendar-todo/frontend/dist}")
    private String STATIC_PATH;

    @Override
    public void addInterceptors(InterceptorRegistry registry) {
        registry.addInterceptor(authInterceptor)
                .addPathPatterns("/**")
                .excludePathPatterns("/auth/login", "/auth/register", "/error");
    }

    @Override
    public void addResourceHandlers(ResourceHandlerRegistry registry) {
        // 优先使用外部静态资源（如果存在）
        java.io.File staticDir = new java.io.File(STATIC_PATH);
        if (staticDir.exists() && staticDir.isDirectory()) {
            // 外部静态资源映射（优先）
            String externalPath = "file:" + STATIC_PATH.replace("\\", "/") + "/";

            registry.addResourceHandler("/**")
                    .addResourceLocations(externalPath, "classpath:/static/");

            registry.addResourceHandler("/assets/**")
                    .addResourceLocations(externalPath + "assets/", "classpath:/static/assets/");

            registry.addResourceHandler("/favicon.ico")
                    .addResourceLocations(externalPath, "classpath:/static/");
        } else {
            // 回退到classpath静态资源
            registry.addResourceHandler("/**")
                    .addResourceLocations("classpath:/static/");

            registry.addResourceHandler("/assets/**")
                    .addResourceLocations("classpath:/static/assets/");

            registry.addResourceHandler("/favicon.ico")
                    .addResourceLocations("classpath:/static/");
        }
    }

    @Override
    public void addViewControllers(ViewControllerRegistry registry) {
        // SPA支持：所有前端路由都指向index.html
        registry.addViewController("/").setViewName("forward:/index.html");
        registry.addViewController("/home").setViewName("forward:/index.html");
        registry.addViewController("/login").setViewName("forward:/index.html");
        registry.addViewController("/tasks").setViewName("forward:/index.html");
        registry.addViewController("/calendar").setViewName("forward:/index.html");
        registry.addViewController("/reports").setViewName("forward:/index.html");
        registry.addViewController("/archived").setViewName("forward:/index.html");
        registry.addViewController("/tags").setViewName("forward:/index.html");
    }
}