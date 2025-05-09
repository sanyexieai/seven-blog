#!/bin/bash

# 设置环境变量
export DATABASE_TYPE="sqlite"
export DATABASE_URL="sqlite:seven_blog.db"
export JWT_SECRET="your_jwt_secret_key_here"
export PORT="8000"

# 检查是否需要重置数据库
if [ "$1" = "--reset" ]; then
    echo "Resetting database..."
    rm -f seven_blog.db
fi

# 执行迁移
echo "Starting database migration..."

# 创建数据库
if ! sqlx database create; then
    echo "Error: Failed to create database"
    exit 1
fi

# 执行迁移
if ! sqlx migrate run; then
    echo "Error: Failed to run migrations"
    exit 1
fi

echo "Database migration completed!" 