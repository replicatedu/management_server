# Stop all containers 
docker stop $(docker ps -a -q)
# Delete all containers
docker rm $(docker ps -a -q)
# Delete all images
docker rmi -f $(docker images -q)
# Build the image 

rm -rf docker_context
cargo clean

docker build . -t dind_base

docker run --privileged -e CODER_PASSWORD="password" -p 8111:8443 --name dind1 -d dind_base
docker exec -i -t dind1 /bin/ash -c ./start_dind.sh