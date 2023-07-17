nats context save m0 --user m0 --password m0
nats context save l0 --user l0 --password l0

nats context save main --server "nats://m0:m0@0.0.0.0:4222"
nats context save leaf --server "nats://l0:l0@0.0.0.0:4223"

