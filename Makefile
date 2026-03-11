.PHONY: all build run dev test clean docker-build docker-run help

# 项目名称
PROJECT_NAME = rust-vue-admin
SERVER_DIR = server
WEB_DIR = web

# 颜色输出
GREEN  := $(shell tput -Txterm setaf 2)
YELLOW := $(shell tput -Txterm setaf 3)
WHITE  := $(shell tput -Txterm setaf 7)
RESET  := $(shell tput -Txterm sgr0)

## 默认目标
all: build

## ===== 后端 Rust 相关命令 =====

## 编译后端（debug 模式）
build-server:
	@echo "$(GREEN)Building Rust server...$(RESET)"
	cd $(SERVER_DIR) && cargo build

## 编译后端（release 模式）
build-server-release:
	@echo "$(GREEN)Building Rust server (release)...$(RESET)"
	cd $(SERVER_DIR) && cargo build --release

## 运行后端开发服务器
run-server:
	@echo "$(GREEN)Running Rust server...$(RESET)"
	cd $(SERVER_DIR) && cargo run

## 运行后端（release 模式）
run-server-release:
	@echo "$(GREEN)Running Rust server (release)...$(RESET)"
	cd $(SERVER_DIR) && cargo run --release

## 热重载开发（需要安装 cargo-watch: cargo install cargo-watch）
dev-server:
	@echo "$(GREEN)Running Rust server with hot-reload...$(RESET)"
	cd $(SERVER_DIR) && cargo watch -x run

## 运行后端测试
test-server:
	@echo "$(GREEN)Running Rust tests...$(RESET)"
	cd $(SERVER_DIR) && cargo test

## 代码格式化
fmt:
	@echo "$(GREEN)Formatting Rust code...$(RESET)"
	cd $(SERVER_DIR) && cargo fmt

## 代码检查
lint:
	@echo "$(GREEN)Linting Rust code...$(RESET)"
	cd $(SERVER_DIR) && cargo clippy -- -D warnings

## 数据库迁移（需要安装 sea-orm-cli: cargo install sea-orm-cli）
migrate-up:
	@echo "$(GREEN)Running database migrations...$(RESET)"
	cd $(SERVER_DIR) && sea-orm-cli migrate up

migrate-down:
	@echo "$(YELLOW)Rolling back database migrations...$(RESET)"
	cd $(SERVER_DIR) && sea-orm-cli migrate down

migrate-fresh:
	@echo "$(YELLOW)Resetting database...$(RESET)"
	cd $(SERVER_DIR) && sea-orm-cli migrate fresh

## 生成 SeaORM 实体
generate-entity:
	@echo "$(GREEN)Generating SeaORM entities...$(RESET)"
	cd $(SERVER_DIR) && sea-orm-cli generate entity -o src/model/entity

## ===== 前端 Vue 相关命令 =====

## 安装前端依赖
install-web:
	@echo "$(GREEN)Installing web dependencies...$(RESET)"
	cd $(WEB_DIR) && npm install

## 运行前端开发服务器
dev-web:
	@echo "$(GREEN)Running Vue dev server...$(RESET)"
	cd $(WEB_DIR) && npm run dev

## 构建前端
build-web:
	@echo "$(GREEN)Building Vue frontend...$(RESET)"
	cd $(WEB_DIR) && npm run build

## ===== 全栈命令 =====

## 同时启动前后端（需要 tmux 或分别在不同终端运行）
dev:
	@echo "$(GREEN)Starting full-stack development...$(RESET)"
	@echo "$(YELLOW)Please run 'make dev-server' and 'make dev-web' in separate terminals$(RESET)"

## 构建全部
build: build-server build-web

## ===== Docker 相关命令 =====

## 构建 Docker 镜像
docker-build:
	@echo "$(GREEN)Building Docker images...$(RESET)"
	docker build -t $(PROJECT_NAME)-server:latest -f deploy/docker/Dockerfile.server .
	docker build -t $(PROJECT_NAME)-web:latest -f deploy/docker/Dockerfile.web .

## 使用 docker-compose 启动
docker-up:
	@echo "$(GREEN)Starting with docker-compose...$(RESET)"
	docker-compose -f deploy/docker-compose/docker-compose.yaml up -d

## 停止 docker-compose
docker-down:
	@echo "$(YELLOW)Stopping docker-compose...$(RESET)"
	docker-compose -f deploy/docker-compose/docker-compose.yaml down

## ===== 清理命令 =====

## 清理构建产物
clean:
	@echo "$(YELLOW)Cleaning build artifacts...$(RESET)"
	cd $(SERVER_DIR) && cargo clean
	rm -rf $(WEB_DIR)/dist

## ===== 帮助 =====

## 显示帮助信息
help:
	@echo ''
	@echo 'Usage:'
	@echo '  ${YELLOW}make${RESET} ${GREEN}<target>${RESET}'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\_0-9]+:/ { \
		helpMessage = match(lastLine, /^## (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")-1); \
			helpMessage = substr(lastLine, RSTART + 3, RLENGTH); \
			printf "  ${YELLOW}%-20s${RESET} ${GREEN}%s${RESET}\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)
