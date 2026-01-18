# test_client.py
import json
import socket

def test_mcp_server():
    # 连接到服务器
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect(('127.0.0.1', 8080))

    # 发送工具列表请求
    request = {
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 1
    }

    sock.sendall((json.dumps(request) + '\n').encode())

    # 接收响应
    response = sock.recv(4096).decode()
    print("工具列表响应:")
    print(json.dumps(json.loads(response), indent=2))

    # 调用工具
    request = {
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_weather",
            "arguments": {
                "city": "北京"
            }
        },
        "id": 2
    }

    sock.sendall((json.dumps(request) + '\n').encode())
    response = sock.recv(4096).decode()
    print("\n天气查询响应:")
    print(json.dumps(json.loads(response), indent=2))

    sock.close()

if __name__ == "__main__":
    test_mcp_server()
