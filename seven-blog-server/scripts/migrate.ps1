# 设置控制台编码为 UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

# 设置环境变量
$env:DATABASE_TYPE = "sqlite"
$env:DATABASE_URL = "sqlite:seven_blog.db"
$env:JWT_SECRET = "your_jwt_secret_key_here"
$env:PORT = "8000"

# 检查是否需要重置数据库
$reset = $args[0] -eq "--reset"

# 执行迁移
Write-Host "Starting database migration..."

try {
    if ($reset) {
        Write-Host "Resetting database..."
        # 尝试关闭所有 SQLite 连接
        Get-Process | Where-Object { $_.ProcessName -like "*sqlite*" } | Stop-Process -Force -ErrorAction SilentlyContinue
        
        if (Test-Path "seven_blog.db") {
            Remove-Item "seven_blog.db" -Force
            # 等待文件系统更新
            Start-Sleep -Seconds 2
        }
    }

    # 确保数据库文件不存在
    if (Test-Path "seven_blog.db") {
        Remove-Item "seven_blog.db" -Force
        Start-Sleep -Seconds 2
    }

    # 删除 SQLite 的临时文件
    if (Test-Path "seven_blog.db-journal") {
        Remove-Item "seven_blog.db-journal" -Force
    }

    sqlx database create
    if ($LASTEXITCODE -ne 0) { throw "Failed to create database" }
    
    sqlx migrate run
    if ($LASTEXITCODE -ne 0) { throw "Failed to run migrations" }
    
    Write-Host "Database migration completed!"
} catch {
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
} 