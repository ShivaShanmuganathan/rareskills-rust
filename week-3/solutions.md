# Week 3 Solutions

## Vulnerable NFT Soroban

### 🔍 Potential Issues & Security Concerns

❗️1. Use of temporary() storage for admin
Problem :
Temporary storage in Soroban has a TTL and can expire unexpectedly.

If TTL expires, the admin value will be deleted automatically by the Soroban runtime.

Consequence:
Admin access may vanish unless TTL is explicitly extended (which this contract does not do).

Could lead to a permanently locked contract with no admin controls.

❗️2. No access control on write_administrator
Problem:
Any caller can invoke write_administrator(...)

There's no check to ensure that only an existing admin can overwrite it

Consequence:
Admin privileges are open to any caller, leading to admin takeover

❗️3. No TTL extension
Even though it's using .temporary(), there's no call to .extend_ttl(). That means:

The admin entry might live for only a few ledgers (e.g. 1–30 seconds)

After expiration, read_administrator will return Error::NotFound






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
