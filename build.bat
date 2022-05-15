echo Building jewelexx/rpi-fan:build

docker build -t jewelexx/rpi-fan:build . -f Dockerfile.build

docker container create --name extract jewelexx/rpi-fan:build  
docker container cp extract:/src/ ./src-build  
docker container rm -f extract

echo Building jewelexx/rpi-fan:latest

docker build --no-cache -t jewelexx/rpi-fan:latest .

del /q ./src-build