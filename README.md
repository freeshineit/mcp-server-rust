```bash
# 构建项目
cargo build --release

# 运行服务器
cargo run -- start

# 指定端口运行
cargo run -- start --address 127.0.0.1:3000

# 列出工具
cargo run -- list-tools

# 列出资源
cargo run -- list-resources

# test
python3 test_client.py 

# docker deploy
docker-compose -f docker-compose.yml up -d
```
