# 多语言上下文工程设置指南

这个仓库提供了针对不同编程语言的上下文工程模板，帮助您为AI编程助手提供更好的上下文信息。

## 🌟 概述

本仓库包含三个主要分支，每个分支都针对特定编程语言进行了优化：

- **`main`** - 通用的上下文工程模板和说明
- **`flutter-project`** - Flutter/Dart项目的上下文工程模板  
- **`rust-project`** - Rust项目的上下文工程模板

## 🚀 快速开始

### 1. 选择您的技术栈

根据您要开发的项目类型，切换到对应的分支：

```bash
# Flutter 移动应用开发
git checkout flutter-project

# Rust 系统/Web 开发
git checkout rust-project

# 通用模板（任何语言）
git checkout main
```

### 2. 查看分支特定的配置

每个分支都包含针对该语言优化的：

- **CLAUDE.md** - AI助手的语言特定规则
- **INITIAL_EXAMPLE.md** - 该语言的项目示例
- **examples/** - 最佳实践代码示例

## 📁 分支详细说明

### Flutter 分支 (`flutter-project`)

**适用于：** Flutter移动应用、跨平台应用开发

**包含内容：**
- Material Design 3 UI组件示例
- Provider/Riverpod状态管理模式
- 响应式设计和错误处理模式
- API集成和数据缓存示例
- Flutter特定的测试模式

**AI助手规则重点：**
- Widget组织和性能优化
- 状态管理最佳实践
- 跨平台兼容性
- Material Design规范

### Rust 分支 (`rust-project`)

**适用于：** Web API、系统工具、高性能应用

**包含内容：**
- Axum web框架Handler示例
- SQLx数据库集成模式
- 全面的错误处理系统
- 异步编程最佳实践
- JWT认证和中间件示例

**AI助手规则重点：**
- 所有权和借用模式
- 错误处理和安全性
- 异步编程规范
- 性能优化技巧

## 🛠️ 使用工作流程

### 1. 项目初始化

```bash
# 切换到目标分支
git checkout <target-branch>

# 复制模板到新项目
cp -r . /path/to/your/new/project/

# 初始化新的Git仓库
cd /path/to/your/new/project/
git init
```

### 2. 自定义项目配置

编辑关键文件以适应您的项目：

- **INITIAL.md** - 描述您要构建的功能
- **CLAUDE.md** - 根据需要调整AI助手规则
- **examples/** - 添加项目特定的代码示例

### 3. 生成和执行PRP

```bash
# 在Claude Code中运行
/generate-prp INITIAL.md

# 执行生成的PRP
/execute-prp PRPs/your-feature-name.md
```

## 📝 自定义指南

### 添加新的语言特定规则

在`CLAUDE.md`中添加您项目的特定约定：

```markdown
### 🎯 项目特定规则
- 您的自定义编码标准
- 特定库的使用模式
- 团队约定和命名规范
```

### 扩展示例集合

在`examples/`文件夹中添加更多示例：

1. 创建新的子目录（如`examples/auth/`）
2. 添加展示最佳实践的代码文件
3. 更新`examples/README.md`说明新示例的用途

### 调整PRP模板

修改`PRPs/templates/prp_base.md`以包含：
- 项目特定的验证步骤
- 自定义的成功标准
- 团队特定的代码审查要求

## 🔄 分支同步

当主分支有更新时，您可以选择性地合并改进：

```bash
# 查看主分支的更新
git checkout main
git pull origin main

# 切换回您的工作分支
git checkout flutter-project  # 或 rust-project

# 选择性合并更新（如README改进）
git cherry-pick <commit-hash>
```

## 💡 最佳实践

### 1. 保持示例代码的相关性
- 定期更新examples/文件夹中的代码
- 确保示例反映当前的最佳实践
- 添加新功能时同步更新示例

### 2. 维护清晰的上下文
- 在INITIAL.md中详细描述功能需求
- 提供充足的业务背景信息
- 包含相关的API文档链接

### 3. 持续改进规则
- 根据项目经验调整CLAUDE.md规则
- 记录常见的AI助手错误并添加预防规则
- 与团队分享有效的上下文工程实践

## 🤝 贡献

如果您有改进建议或新的语言支持需求：

1. Fork这个仓库
2. 创建新的特性分支
3. 提交您的改进
4. 发起Pull Request

每个新语言分支应该包含：
- 语言特定的CLAUDE.md规则
- 实用的INITIAL_EXAMPLE.md
- 全面的examples/代码示例
- 清晰的README说明

## 📚 相关资源

- [上下文工程最佳实践](https://www.philschmid.de/context-engineering)
- [Claude Code文档](https://docs.anthropic.com/en/docs/claude-code)
- [Flutter文档](https://docs.flutter.dev/) (Flutter分支)
- [Rust官方文档](https://doc.rust-lang.org/) (Rust分支)