`src/1m_random_deflated` was created on a mac with:

```
bash-3.2$ head -c 1000000 </dev/urandom | gzip --no-name |   ( dd of=/dev/null bs=1 count=10; cat > gzip-without-header )
bash-3.2$ dd if=gzip-without-header of=gzip-without-anything     bs=1 count=$[ `stat -f '%z' gzip-without-header` - 8 ]
bash-3.2$ mv gzip-without-anything src/1m_random_deflated
```
