# Week 3 Solutions

## Vulnerable HTTP Server

How to crash the server:

1. Make a request to the `/math` endpoint with a division by zero operation:
```bash
 curl -X POST http://localhost:3000/math \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 0, "operation": "division"}'
```

2. Make a request to the `/math` endpoint with a subtratcion operation where `a` is less than `b`:
```bash
 curl -X POST http://localhost:3000/math \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 20, "operation": "subtraction"}'
```
3. Make a request to the `/math` endpoint with a addition or multiplication operation where the result exceeds u64::MAX:
```bash
 curl -X POST http://localhost:3000/math \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 20, "operation": "addition"}'
```
